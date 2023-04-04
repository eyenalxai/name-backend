use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Syllables {
    pub syllables_list: Vec<String>,
}

impl Syllables {
    fn generate_string(&self, min_length: usize, max_length: Option<usize>) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::new();

        let target_length = match max_length {
            Some(max) => rng.gen_range(min_length..=max),
            None => min_length,
        };

        while result.len() < target_length {
            let syllable = self.syllables_list.choose(&mut rng).unwrap();
            result.push_str(syllable);
        }

        result
    }

    pub fn generate_username(&self, min_length: usize) -> String {
        self.generate_string(min_length, None)
    }

    pub fn generate_name(&self, min_length: usize, max_length: usize) -> String {
        let name = self.generate_string(min_length, Some(max_length));

        let mut chars = name.chars();
        let first_char = chars
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_default();
        let rest = chars.collect::<String>();

        format!("{}{}", first_char, rest)
    }
}
