pub struct Solution;

#[cfg(test)]
mod tests;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter_entries = Vec::<Entry>::new();
        let mut digit_entries = Vec::<Entry>::new();

        for (index, entry_text) in logs.iter().enumerate() {
            let id = entry_text.split_ascii_whitespace().next().expect("logs[i] is guaranteed to have an identifier and at least one word after the identifier.");
            let content = &entry_text[id.len() + 1..];

            let entry = Entry { index, id, content };

            let first_char = content.chars().next().expect("logs[i] is guaranteed to have an identifier and at least one word after the identifier.");

            if first_char.is_numeric() {
                digit_entries.push(entry);
            } else {
                assert!(first_char.is_alphabetic());
                letter_entries.push(entry);
            }
        }

        letter_entries
            .sort_by(|left, right| (left.content, left.id).cmp(&(right.content, right.id)));

        let mut out = Vec::with_capacity(logs.len());

        for entry in letter_entries {
            out.push(logs[entry.index].to_owned());
        }
        for entry in digit_entries {
            out.push(logs[entry.index].to_owned())
        }

        out
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry<'a> {
    index: usize,
    id: &'a str,
    content: &'a str,
}
