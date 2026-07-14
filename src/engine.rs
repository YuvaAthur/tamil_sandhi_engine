use crate::token::tokenize_tamil;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NiruktaClass {
    Nama,    // Noun / Peyar
    Akhyata, // Verb / Vinai
    Uri,     // Qualifier / Root Attribute
}

#[derive(Debug, Clone)]
pub struct SandhiSplitResult {
    pub original: String,
    pub paguthi_root: String,
    pub viguthi_suffix: String,
    pub modification_rule: String,
    pub semantic_class: NiruktaClass,
}

pub struct SandhiEngine {
    verbal_roots: Vec<String>,
}

impl SandhiEngine {
    pub fn new() -> Self {
        Self {
            verbal_roots: vec![
                "ஊடு".to_string(), 
                "அறு".to_string(), 
                "தொடு".to_string(),
            ],
        }
    }

    pub fn reverse_sandhi(&self, word: &str) -> Option<SandhiSplitResult> {
        let tokens = tokenize_tamil(word);
        if tokens.is_empty() { return None; }

        if tokens.last().map(|s| s.as_str()) == Some("ல்") {
            if tokens.len() >= 3 {
                let middle_token = &tokens[tokens.len() - 2];
                if middle_token == "ட" {
                    let mut base_tokens = tokens[0..tokens.len() - 2].to_vec();
                    base_tokens.push("டு".to_string());
                    let reconstructed_root = base_tokens.join("");
                    if self.verbal_roots.contains(&reconstructed_root) {
                        return Some(SandhiSplitResult {
                            original: word.to_string(),
                            paguthi_root: reconstructed_root,
                            viguthi_suffix: "அல்".to_string(),
                            modification_rule: "Keduthal: Elision of root-terminal 'u' to resolve incoming vowel suffix '-al'".to_string(),
                            semantic_class: NiruktaClass::Nama,
                        });
                    }
                }
            }
        }

        if tokens.last().map(|s| s.as_str()) == Some("ம்") {
            if tokens.len() >= 3 && tokens[tokens.len() - 2] == "ற" {
                let mut base_tokens = tokens[0..tokens.len() - 2].to_vec();
                base_tokens.push("று".to_string());
                let reconstructed_root = base_tokens.join("");
                if self.verbal_roots.contains(&reconstructed_root) {
                    return Some(SandhiSplitResult {
                        original: word.to_string(),
                        paguthi_root: reconstructed_root,
                        viguthi_suffix: "அம்".to_string(),
                        modification_rule: "Thiriandhal: Trailing hard consonant cluster modification into concept noun".to_string(),
                        semantic_class: NiruktaClass::Nama,
                    });
                }
            }
        }

        if self.verbal_roots.contains(&word.to_string()) {
            return Some(SandhiSplitResult {
                original: word.to_string(),
                paguthi_root: word.to_string(),
                viguthi_suffix: "None (Base Stem)".to_string(),
                modification_rule: "None: Is primitive component root".to_string(),
                semantic_class: NiruktaClass::Akhyata,
            });
        }

        None
    }
}
