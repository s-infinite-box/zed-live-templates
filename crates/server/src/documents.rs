#[derive(Clone)]
pub struct Document {
    pub text: String,
    pub language: String,
    pub version: i32,
}

pub fn byte_offset(line: &str, character: u32) -> Option<usize> {
    let mut units = 0;
    for (offset, c) in line.char_indices() {
        if units == character {
            return Some(offset);
        }
        units += c.len_utf16() as u32;
    }
    (units == character).then_some(line.len())
}

pub fn trigger_start(line: &str, offset: usize, trigger: &str) -> Option<usize> {
    if !line[..offset].ends_with(trigger) {
        return None;
    }
    let start = offset - trigger.len();
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    if line[..start].chars().next_back().is_some_and(is_word)
        || line[offset..].chars().next().is_some_and(is_word)
    {
        return None;
    }
    Some(start)
}
