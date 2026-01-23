use tower_lsp::lsp_types::Position;

#[derive(Clone, Debug)]
pub struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut line_starts = vec![0];
        for (idx, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(idx + 1);
            }
        }
        Self { line_starts }
    }

    pub fn offset_to_position(&self, text: &str, offset: usize) -> Position {
        let clamped = offset.min(text.len());
        let line = match self.line_starts.binary_search(&clamped) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };
        let line_start = self.line_starts.get(line).copied().unwrap_or(0);
        let line_text = &text[line_start..clamped];
        let character = line_text.encode_utf16().count() as u32;
        Position::new(line as u32, character)
    }

    pub fn position_to_offset(&self, text: &str, position: Position) -> usize {
        let line = position.line as usize;
        if self.line_starts.is_empty() {
            return 0;
        }
        if line >= self.line_starts.len() {
            return text.len();
        }
        let line_start = self.line_starts[line];
        let line_end = if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1]
        } else {
            text.len()
        };
        let mut utf16_count = 0u32;
        let mut last_byte = line_start;
        for (byte_idx, ch) in text[line_start..line_end].char_indices() {
            let ch_utf16 = ch.len_utf16() as u32;
            if utf16_count + ch_utf16 > position.character {
                return last_byte;
            }
            utf16_count += ch_utf16;
            last_byte = line_start + byte_idx + ch.len_utf8();
        }
        if position.character > utf16_count {
            line_end
        } else {
            last_byte
        }
    }
}
