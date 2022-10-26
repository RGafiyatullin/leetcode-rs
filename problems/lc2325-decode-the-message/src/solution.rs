pub struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        eprintln!("key: {:?}", key);
        eprintln!("msg: {:?}", message);

        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut key_chars = key.chars().filter(|ch| ch.is_ascii_alphabetic());
        let mut map = HashMap::new();

        for ch_p in alphabet.chars() {
            (&mut key_chars)
                .take_while(|ch_s| {
                    let ch_s = *ch_s;

                    if let Entry::Vacant(vacant) = map.entry(ch_s) {
                        vacant.insert(ch_p);
                        false
                    } else {
                        true
                    }
                })
                .count();
        }

        eprintln!("map: {:?}", {
            let mut vec = map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
            vec.sort_by_key(|(_, val)| *val);
            vec
        });

        message
            .chars()
            .map(|ch_s| if ch_s.is_whitespace() { ch_s } else { map.get(&ch_s).copied().unwrap() })
            .collect()
    }
}
