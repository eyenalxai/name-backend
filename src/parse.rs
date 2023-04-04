use crate::syllables::Syllables;

pub fn get_syllables_from_json(path: &str) -> Syllables {
    Syllables {
        syllables_list: serde_json::from_str(path)
            .unwrap_or_else(|_| panic!("Failed to parse {}", path)),
    }
}
