use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Range};
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(super) fn range_for_node(&self, node: Node) -> Range {
        let start = self
            .line_index
            .offset_to_position(self.text, node.start_byte());
        let end = self
            .line_index
            .offset_to_position(self.text, node.end_byte());
        Range { start, end }
    }

    pub(super) fn node_text(&self, node: Node) -> String {
        self.text.get(node.byte_range()).unwrap_or("").to_string()
    }

    pub(super) fn call_name_from_node(&self, function_node: Node) -> String {
        match function_node.kind() {
            "identifier" => self.node_text(function_node),
            "attribute" => self
                .attribute_chain_name(function_node)
                .unwrap_or_else(|| self.node_text(function_node)),
            "template_function" => {
                if let Some(name_node) = function_node.child_by_field_name("name") {
                    if name_node.kind() == "attribute" {
                        return self
                            .attribute_chain_name(name_node)
                            .unwrap_or_else(|| self.node_text(name_node));
                    }
                    if name_node.kind() == "identifier" {
                        return self.node_text(name_node);
                    }
                }
                self.node_text(function_node)
            }
            _ => self.node_text(function_node),
        }
    }

    pub(super) fn template_type_name(&self, function_node: Node) -> Option<String> {
        if function_node.kind() != "template_function" {
            return None;
        }
        let args_node = function_node.child_by_field_name("arguments")?;
        let mut cursor = args_node.walk();
        for child in args_node.named_children(&mut cursor) {
            if child.kind() == "base_type" {
                let name = self.node_text(child);
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
        None
    }

    pub(super) fn attribute_chain_name(&self, node: Node) -> Option<String> {
        if node.kind() != "attribute" {
            return None;
        }
        let object = node.child_by_field_name("object")?;
        let attr = node.child_by_field_name("attribute")?;
        let attr_name = self.node_text(attr);
        let object_name = match object.kind() {
            "identifier" => self.node_text(object),
            "attribute" => self.attribute_chain_name(object)?,
            "primary_expression" => {
                let child = object.named_child(0)?;
                match child.kind() {
                    "identifier" => self.node_text(child),
                    "attribute" => self.attribute_chain_name(child)?,
                    _ => return None,
                }
            }
            _ => return None,
        };
        Some(format!("{}.{}", object_name, attr_name))
    }

    pub(super) fn is_builtin_namespace(&self, name: &str) -> bool {
        matches!(
            name,
            "ta" | "math" | "input" | "request" | "strategy" | "syminfo" | "color"
        )
    }

    pub(super) fn push_diagnostic(
        &mut self,
        range: Range,
        severity: DiagnosticSeverity,
        message: String,
        code: String,
    ) {
        let range = self.normalize_diagnostic_range(range);
        self.diagnostics.push(Diagnostic {
            range,
            severity: Some(severity),
            code: Some(NumberOrString::String(code)),
            code_description: None,
            source: Some("pinescript-vsc-server-rust".to_string()),
            message,
            related_information: None,
            tags: None,
            data: None,
        });
    }

    fn normalize_diagnostic_range(&self, range: Range) -> Range {
        if range.start.line != range.end.line {
            return range;
        }
        let len = range.end.character.saturating_sub(range.start.character);
        if len >= 3 {
            return range;
        }
        let expanded = self.expand_range_to_word(range);
        let expanded_len = expanded
            .end
            .character
            .saturating_sub(expanded.start.character);
        if expanded_len > len { expanded } else { range }
    }

    fn expand_range_to_word(&self, range: Range) -> Range {
        let mut offset = self.line_index.position_to_offset(self.text, range.start);
        let bytes = self.text.as_bytes();
        if offset >= bytes.len() {
            return range;
        }
        if !is_word_byte(bytes[offset]) {
            if offset == 0 || !is_word_byte(bytes[offset - 1]) {
                return range;
            }
            offset = offset.saturating_sub(1);
        }

        let mut left = offset;
        while left > 0 && bytes[left - 1] != b'\n' && is_word_byte(bytes[left - 1]) {
            left -= 1;
        }
        let mut right = offset;
        while right < bytes.len() && bytes[right] != b'\n' && is_word_byte(bytes[right]) {
            right += 1;
        }

        let start = self.line_index.offset_to_position(self.text, left);
        let end = self.line_index.offset_to_position(self.text, right);
        Range { start, end }
    }
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.'
}
