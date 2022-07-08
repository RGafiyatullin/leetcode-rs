pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails.iter() {
            unique_emails.insert(normalize(email));
        }

        unique_emails.len() as i32
    }
}

fn normalize(input: &str) -> (String, &str) {
    let (user, domain) = input
        .split_once('@')
        .expect("Constraints: Each emails[i] contains exactly one '@' character.");

    let (user, _topic) = user
        .split_once('+')
        .map(|(id, topic)| (id, Some(topic)))
        .unwrap_or((user, None));

    (user.replace('.', ""), domain)
}
