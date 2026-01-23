use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::analysis::{AnalysisResult, AnalysisSettings, SymbolKind};
use crate::builtins::Builtins;
use crate::document::Document;

pub struct Backend {
    client: Client,
    state: Arc<RwLock<State>>,
    builtins: Builtins,
}

struct State {
    documents: HashMap<Url, Document>,
    settings: AnalysisSettings,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Arc::new(RwLock::new(State {
                documents: HashMap::new(),
                settings: AnalysisSettings::default(),
            })),
            builtins: Builtins::new(),
        }
    }

    async fn publish_diagnostics(&self, uri: Url, analysis: &AnalysisResult) {
        self.client
            .publish_diagnostics(uri, analysis.diagnostics.clone(), None)
            .await;
    }

    fn find_occurrence(doc: &Document, position: Position) -> Option<(usize, String)> {
        for (idx, def) in doc.analysis.definitions.iter().enumerate() {
            if def.is_builtin {
                continue;
            }
            if range_contains(def.selection_range, position) {
                return Some((idx, def.name.clone()));
            }
        }
        for reference in &doc.analysis.references {
            if range_contains(reference.range, position)
                && let Some(def_index) = reference.def_index
            {
                return Some((def_index, reference.name.clone()));
            }
        }
        None
    }

    fn find_hover_symbol(doc: &Document, position: Position) -> Option<(Option<usize>, String)> {
        for (idx, def) in doc.analysis.definitions.iter().enumerate() {
            if range_contains(def.selection_range, position) {
                return Some((Some(idx), def.name.clone()));
            }
        }
        for reference in &doc.analysis.references {
            if range_contains(reference.range, position) {
                return Some((reference.def_index, reference.name.clone()));
            }
        }
        None
    }

    fn enum_completion_items(doc: &Document, position: Position) -> Option<Vec<CompletionItem>> {
        let offset = doc.line_index.position_to_offset(&doc.text, position);
        if offset == 0 {
            return None;
        }
        let prefix = &doc.text[..offset];
        let bytes = prefix.as_bytes();
        let mut start = offset;
        while start > 0 {
            let ch = bytes[start - 1];
            if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'.' {
                start -= 1;
            } else {
                break;
            }
        }
        let segment = &prefix[start..];
        let dot_pos = segment.rfind('.')?;
        let object = &segment[..dot_pos];
        if object.is_empty() || object.contains('.') {
            return None;
        }
        let members = doc.analysis.enums.get(object)?;
        let mut items = Vec::with_capacity(members.len());
        for member in members {
            items.push(CompletionItem {
                label: member.clone(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                detail: Some(format!("{}.{}", object, member)),
                ..CompletionItem::default()
            });
        }
        Some(items)
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            completion_provider: Some(CompletionOptions {
                trigger_characters: Some(vec![".".to_string()]),
                resolve_provider: Some(false),
                ..CompletionOptions::default()
            }),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                retrigger_characters: None,
                work_done_progress_options: Default::default(),
            }),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            rename_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            ..ServerCapabilities::default()
        };

        Ok(InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "pinescript-vsc-server-rust".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Pine v6 LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let settings = {
            let state = self.state.read().await;
            state.settings.clone()
        };
        match Document::new_with_settings(params.text_document.text, settings) {
            Ok(doc) => {
                let mut state = self.state.write().await;
                state.documents.insert(uri.clone(), doc.clone());
                self.publish_diagnostics(uri, &doc.analysis).await;
            }
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Failed to open doc: {}", err))
                    .await;
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut state = self.state.write().await;
        if let Some(doc) = state.documents.get_mut(&uri) {
            if let Err(err) = doc.apply_changes(params.content_changes) {
                self.client
                    .log_message(MessageType::ERROR, format!("Failed to update doc: {}", err))
                    .await;
                return;
            }
            let analysis = doc.analysis.clone();
            drop(state);
            self.publish_diagnostics(uri, &analysis).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut state = self.state.write().await;
        state.documents.remove(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        #[derive(serde::Deserialize, Default)]
        #[serde(default)]
        struct ClientSettings {
            analysis: AnalysisSettings,
        }

        let mut settings = AnalysisSettings::default();
        if let Ok(wrapper) = serde_json::from_value::<ClientSettings>(params.settings.clone()) {
            settings = wrapper.analysis;
        } else if let Ok(direct) = serde_json::from_value::<AnalysisSettings>(params.settings) {
            settings = direct;
        }

        let mut state = self.state.write().await;
        state.settings = settings.clone();
        let mut updates = Vec::new();
        for (uri, doc) in state.documents.iter_mut() {
            if let Err(err) = doc.update_settings(settings.clone()) {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to update settings: {}", err),
                    )
                    .await;
                continue;
            }
            updates.push((uri.clone(), doc.analysis.clone()));
        }
        drop(state);
        for (uri, analysis) in updates {
            self.publish_diagnostics(uri, &analysis).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut items = Vec::new();
        let mut seen = HashSet::new();
        let position = params.text_document_position.position;
        let uri = params.text_document_position.text_document.uri;

        let state = self.state.read().await;
        let doc = state.documents.get(&uri);
        if let Some(doc) = doc
            && let Some(enum_items) = Self::enum_completion_items(doc, position)
        {
            return Ok(Some(CompletionResponse::Array(enum_items)));
        }

        for func in self.builtins.functions() {
            if seen.insert(func.name.to_string()) {
                items.push(CompletionItem {
                    label: func.name.to_string(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(func.signature.to_string()),
                    ..CompletionItem::default()
                });
            }
        }

        for (name, ty) in self.builtins.values() {
            if seen.insert(name.clone()) {
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some(ty.display_name()),
                    ..CompletionItem::default()
                });
            }
        }

        if let Some(doc) = doc {
            for def in &doc.analysis.definitions {
                if def.is_builtin {
                    continue;
                }
                if !seen.insert(def.name.clone()) {
                    continue;
                }
                let kind = match def.kind {
                    SymbolKind::Function => CompletionItemKind::FUNCTION,
                    SymbolKind::Parameter => CompletionItemKind::VARIABLE,
                    SymbolKind::LoopVariable | SymbolKind::Variable => CompletionItemKind::VARIABLE,
                    SymbolKind::Builtin => CompletionItemKind::CONSTANT,
                    SymbolKind::Type => {
                        if doc.analysis.enums.contains_key(&def.name) {
                            CompletionItemKind::ENUM
                        } else {
                            CompletionItemKind::CLASS
                        }
                    }
                };
                items.push(CompletionItem {
                    label: def.name.clone(),
                    kind: Some(kind),
                    detail: Some(def.ty.display_name()),
                    ..CompletionItem::default()
                });
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        let Some((def_index, name)) = Self::find_hover_symbol(doc, position) else {
            return Ok(None);
        };

        let def = def_index.and_then(|idx| doc.analysis.definitions.get(idx));
        let builtin_func = self.builtins.function(&name);
        let builtin_value = self.builtins.value_type(&name);

        let mut lines = Vec::new();
        lines.push(format!("**{}**", name));

        let kind_label = if let Some(def) = def {
            match def.kind {
                SymbolKind::Variable => "variable",
                SymbolKind::Function => "function",
                SymbolKind::Parameter => "parameter",
                SymbolKind::LoopVariable => "loop variable",
                SymbolKind::Builtin => "builtin",
                SymbolKind::Type => "type",
            }
        } else if builtin_func.is_some() {
            "builtin function"
        } else if builtin_value.is_some() {
            "builtin value"
        } else {
            "symbol"
        };
        lines.push(format!("Kind: `{}`", kind_label));

        if let Some(func) = builtin_func {
            lines.push(format!("Signature: `{}`", func.signature));
            lines.push(format!("Returns: `{}`", func.return_type.display_name()));
        } else if let Some(def) = def {
            lines.push(format!("Type: `{}`", def.ty.display_name()));
        } else if let Some(ty) = builtin_value {
            lines.push(format!("Type: `{}`", ty.display_name()));
        }

        let contents = lines.join("\n");

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: contents,
            }),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        let Some((def_index, _)) = Self::find_occurrence(doc, position) else {
            return Ok(None);
        };
        let Some(def) = doc.analysis.definitions.get(def_index) else {
            return Ok(None);
        };
        if def.is_builtin {
            return Ok(None);
        }
        let location = Location::new(uri, def.selection_range);
        Ok(Some(GotoDefinitionResponse::Scalar(location)))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        let Some((def_index, _)) = Self::find_occurrence(doc, position) else {
            return Ok(None);
        };

        let mut locations = Vec::new();
        for reference in &doc.analysis.references {
            if reference.def_index == Some(def_index) {
                locations.push(Location::new(uri.clone(), reference.range));
            }
        }

        if params.context.include_declaration
            && let Some(def) = doc.analysis.definitions.get(def_index)
            && !def.is_builtin
        {
            locations.push(Location::new(uri, def.selection_range));
        }

        Ok(Some(locations))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        let Some((def_index, _)) = Self::find_occurrence(doc, position) else {
            return Ok(None);
        };

        let mut edits = Vec::new();
        for reference in &doc.analysis.references {
            if reference.def_index == Some(def_index) {
                edits.push(TextEdit {
                    range: reference.range,
                    new_text: new_name.clone(),
                });
            }
        }

        if let Some(def) = doc.analysis.definitions.get(def_index)
            && !def.is_builtin
        {
            edits.push(TextEdit {
                range: def.selection_range,
                new_text: new_name.clone(),
            });
        }

        let mut changes = HashMap::new();
        changes.insert(uri, edits);

        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        Ok(Some(DocumentSymbolResponse::Nested(
            doc.analysis.document_symbols.clone(),
        )))
    }

    #[allow(deprecated)]
    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let mut results = Vec::new();
        let state = self.state.read().await;
        for (uri, doc) in &state.documents {
            for def in &doc.analysis.definitions {
                if def.is_builtin {
                    continue;
                }
                if !def.name.contains(&params.query) {
                    continue;
                }
                results.push(SymbolInformation {
                    name: def.name.clone(),
                    kind: def.kind.to_lsp(),
                    tags: None,
                    deprecated: None,
                    location: Location::new(uri.clone(), def.selection_range),
                    container_name: None,
                });
            }
        }
        Ok(Some(results))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let state = self.state.read().await;
        let Some(doc) = state.documents.get(&uri) else {
            return Ok(None);
        };

        let offset = doc.line_index.position_to_offset(&doc.text, position);
        // Find the smallest node that contains the offset
        let mut node = doc.tree.root_node();
        loop {
            let mut found_child = None;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.start_byte() <= offset && offset <= child.end_byte() {
                    found_child = Some(child);
                    break;
                }
            }
            match found_child {
                Some(child) => node = child,
                None => break,
            }
        }

        // Walk up to enclosing call node
        let mut cur = node;
        let mut call_node_opt = None;
        loop {
            if cur.kind() == "call" {
                call_node_opt = Some(cur);
                break;
            }
            if let Some(parent) = cur.parent() {
                cur = parent;
            } else {
                break;
            }
        }
        let Some(call_node) = call_node_opt else {
            return Ok(None);
        };

        let function_node = call_node.child_by_field_name("function");
        let Some(function_node) = function_node else {
            return Ok(None);
        };

        // Resolve function name (supports attribute chains like `ta.macd`)
        let name = if function_node.kind() == "identifier" {
            doc.text
                .get(function_node.byte_range())
                .unwrap_or("")
                .to_string()
        } else if function_node.kind() == "attribute" {
            fn attribute_chain_name(doc: &Document, node: tree_sitter::Node) -> Option<String> {
                if node.kind() != "attribute" {
                    return None;
                }
                let object = node.child_by_field_name("object")?;
                let attr = node.child_by_field_name("attribute")?;
                let attr_name = doc.text.get(attr.byte_range())?.to_string();
                let object_name = match object.kind() {
                    "identifier" => doc.text.get(object.byte_range())?.to_string(),
                    "attribute" => attribute_chain_name(doc, object)?,
                    "primary_expression" => {
                        let child = object.named_child(0)?;
                        match child.kind() {
                            "identifier" => doc.text.get(child.byte_range())?.to_string(),
                            "attribute" => attribute_chain_name(doc, child)?,
                            _ => return None,
                        }
                    }
                    _ => return None,
                };
                Some(format!("{}.{}", object_name, attr_name))
            }
            attribute_chain_name(doc, function_node).unwrap_or_else(|| {
                doc.text
                    .get(function_node.byte_range())
                    .unwrap_or("")
                    .to_string()
            })
        } else {
            doc.text
                .get(function_node.byte_range())
                .unwrap_or("")
                .to_string()
        };

        if let Some(func) = self.builtins.function(&name) {
            // parse signature string into parameter labels
            let _params_vec = parse_signature_params(func.signature);

            // determine active parameter index by examining call arguments
            let active_param = active_param_index_for_call(&doc.text, call_node, offset) as u32;

            let sig_info = SignatureInformation {
                label: func.signature.to_string(),
                documentation: None,
                parameters: None,
                active_parameter: None,
            };

            return Ok(Some(SignatureHelp {
                signatures: vec![sig_info],
                active_signature: Some(0),
                active_parameter: Some(active_param),
            }));
        }

        Ok(None)
    }
}

fn parse_signature_params(sig: &str) -> Vec<String> {
    if let Some(start) = sig.find('(')
        && let Some(end) = sig.rfind(')')
    {
        return sig[start + 1..end]
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
    }
    Vec::new()
}

fn active_param_index_for_call(text: &str, call_node: tree_sitter::Node, offset: usize) -> usize {
    let start = call_node.start_byte();
    if offset <= start {
        return 0;
    }
    let end = offset.min(call_node.end_byte());
    let slice = match text.get(start..end) {
        Some(value) => value,
        None => return 0,
    };

    let mut idx = 0usize;
    let mut depth_paren = 0u32;
    let mut depth_bracket = 0u32;
    let mut depth_brace = 0u32;
    let mut in_string: Option<char> = None;
    let mut escape = false;

    for ch in slice.chars() {
        if let Some(quote) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == quote {
                in_string = None;
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            in_string = Some(ch);
            continue;
        }

        match ch {
            '(' => depth_paren += 1,
            ')' => depth_paren = depth_paren.saturating_sub(1),
            '[' => depth_bracket += 1,
            ']' => depth_bracket = depth_bracket.saturating_sub(1),
            '{' => depth_brace += 1,
            '}' => depth_brace = depth_brace.saturating_sub(1),
            ',' if depth_paren == 1 && depth_bracket == 0 && depth_brace == 0 => idx += 1,
            _ => {}
        }
    }

    idx
}

fn range_contains(range: Range, position: Position) -> bool {
    position_leq(range.start, position) && position_lt(position, range.end)
}

fn position_leq(left: Position, right: Position) -> bool {
    left.line < right.line || (left.line == right.line && left.character <= right.character)
}

fn position_lt(left: Position, right: Position) -> bool {
    left.line < right.line || (left.line == right.line && left.character < right.character)
}
