use std::ops::Range;

use tower_lsp::lsp_types;

pub fn range(range: lsp_types::Range, document: &str) -> Option<Range<usize>> {
    // TODO optimization: end > start, we are wasting time going through all the lines up to start twice
    Some(position(range.start, document)?..position(range.end, document)?)
}

pub fn position(position: lsp_types::Position, document: &str) -> Option<usize> {
    let line = document.lines().nth(position.line as usize)?;
    let line_position = line.as_ptr() as usize - document.as_ptr() as usize;
    dbg!(line_position);

    let mut utf16_offset = position.character as usize;
    for (index, char) in line.char_indices() {
        let len = char.len_utf16();
        if utf16_offset < len {
            return Some(line_position + index);
        }
        utf16_offset -= len;
    }

    Some(line_position + line.len())
}

#[cfg(test)]
#[test]
fn convert() {
    let document = include_str!("../LICENSE-MIT");

    assert_eq!(
        position(lsp_types::Position::new(7, 7), document),
        Some(387)
    );
}
