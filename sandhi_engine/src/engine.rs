use crate::token::tokenize_tamil;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NiruktaClass {
    Nama,     // Noun / Peyar
    Akhyata,  // Verb / Vinai
    Uri,      // Qualifier / Root Attribute
}

// --- Serde Mapping Structs for Data Imports ---

#[derive(Deserialize, Debug, Clone)]
pub struct VerbRecord {
    pub root: String,
    pub meaning: String,
    pub class: String,
    pub past_marker: String,
}

#[derive(Deserialize, Debug)]
struct VerbDatabase {
    verbs: Vec<VerbRecord>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NounRecord {
    pub base: String,
    pub meaning: String,
    pub is_primitive: bool,
}

#[derive(Deserialize, Debug)]
struct NounDatabase {
    nouns: Vec<NounRecord>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UriRecord {
    pub token: String,
    pub meaning: String,
    pub behavior: String,
}

#[derive(Deserialize, Debug)]
struct UriDatabase {
    modifiers: Vec<UriRecord>,
}

// --- Combined Result Structural Framework ---

#[derive(Debug, Clone)]
pub struct SandhiSplitResult {
    pub original: String,
    pub paguthi_root: String,
    pub root_meaning: String,
    pub viguthi_suffix: String,
    pub modification_rule: String,
    pub semantic_class: NiruktaClass,
}

pub struct SandhiEngine {
    pub verbal_roots: Vec<VerbRecord>,
    pub nominal_bases: Vec<NounRecord>,
    pub uri_modifiers: Vec<UriRecord>,
}

impl SandhiEngine {
    /// Initializes the engine loop by pulling raw data models directly from JSON assets.
    pub fn new() -> Self {
        let verbal_roots = Self::load_json_database::<VerbDatabase, VerbRecord>(
            "data/akhyata_roots.json", 

            |db| db.verbs
        );
        let nominal_bases = Self::load_json_database::<NounDatabase, NounRecord>(
            "data/nama_bases.json", 

            |db| db.nouns
        );
        let uri_modifiers = Self::load_json_database::<UriDatabase, UriRecord>(
            "data/uri_modifiers.json", 

            |db| db.modifiers
        );

        Self {
            verbal_roots,
            nominal_bases,
            uri_modifiers,
        }
    }

    /// Generic background filesystem helper function to bind file streams safely.
    fn load_json_database<T, R>(file_path: &str, extractor: fn(T) -> Vec<R>) -> Vec<R> 
    where 
        for<'de> T: Deserialize<'de> 
    {
        if !Path::new(file_path).exists() {
            println!("[Warning] Asset target '{}' missing on disk! Using empty fallback index.", file_path);
            return Vec::new();
        }

        let mut file = File::open(file_path).expect("Could not open JSON engine data asset target");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Corrupted data stream reading file content logs");
        
        let database: T = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Syntax schema evaluation error in {}: {}", file_path, e));
        
        extractor(database)
    }

    /// Primary analysis engine loop executing reverse parsing strategies using active lexicons.
    pub fn reverse_sandhi(&self, word: &str) -> Option<SandhiSplitResult> {
        let tokens = tokenize_tamil(word);
        if tokens.is_empty() { return None; }

        // 1. Evaluate Rule Group: Nominalized Verbs ending in "-al" (அல்)
        if tokens.last().map(|s| s.as_str()) == Some("ல்") && tokens.len() >= 3 {
            let middle_token = &tokens[tokens.len() - 2];
            if middle_token == "ட" {
                let mut base_tokens = tokens[0..tokens.len() - 2].to_vec();
                base_tokens.push("டு".to_string());
                let reconstructed_root = base_tokens.join("");

                if let Some(v_rec) = self.verbal_roots.iter().find(|v| v.root == reconstructed_root) {
                    return Some(SandhiSplitResult {
                        original: word.to_string(),
                        paguthi_root: v_rec.root.clone(),
                        root_meaning: v_rec.meaning.clone(),
                        viguthi_suffix: "அல் (-al)".to_string(),
                        modification_rule: "Keduthal: Drop root-terminal vowel 'u' sound to append nominalizer particle.".to_string(),
                        semantic_class: NiruktaClass::Nama,
                    });
                }
            }
        }

        // 2. Evaluate Rule Group: Absolute Conceptual Nouns ending in "-am" (அம்)
        if tokens.last().map(|s| s.as_str()) == Some("ம்") && tokens.len() >= 3 {
            if tokens[tokens.len() - 2] == "ற" {
                let mut base_tokens = tokens[0..tokens.len() - 2].to_vec();
                base_tokens.push("று".to_string());
                let reconstructed_root = base_tokens.join("");

                if let Some(v_rec) = self.verbal_roots.iter().find(|v| v.root == reconstructed_root) {
                    return Some(SandhiSplitResult {
                        original: word.to_string(),
                        paguthi_root: v_rec.root.clone(),
                        root_meaning: v_rec.meaning.clone(),
                        viguthi_suffix: "அம் (-am)".to_string(),
                        modification_rule: "Thiriandhal: Alveolar liquid shift converting dynamic action to concept base.".to_string(),
                        semantic_class: NiruktaClass::Nama,
                    });
                }
            }
        }

        // 3. Evaluate Rule Group: Panbuthhogai Compound Nouns combining Uri qualifiers (e.g. நறுமுகை)
        // Find if the word starts with any registered Uri modifier string token
        for modifier in &self.uri_modifiers {
            if word.starts_with(&modifier.token) && word.len() > modifier.token.len() {
                let remainder_stem = &word[modifier.token.len()..];
                
                // Confirm if remaining segment matches a known absolute base noun inside our data records
                if let Some(n_rec) = self.nominal_bases.iter().find(|n| n.base == remainder_stem) {
                    return Some(SandhiSplitResult {
                        original: word.to_string(),
                        paguthi_root: modifier.token.clone(),
                        root_meaning: format!("Qualifier: '{}' -> modifying target concept '{}'", modifier.meaning, n_rec.meaning),
                        viguthi_suffix: n_rec.base.clone(),
                        modification_rule: format!("Panbuththogai Compound: Combining qualitative modifier via strategy '{}'", modifier.behavior),
                        semantic_class: NiruktaClass::Nama,
                    });
                }
            }
        }

        // 4. Primitive Fallback Matching
        if let Some(v_rec) = self.verbal_roots.iter().find(|v| v.root == word) {
            return Some(SandhiSplitResult {
                original: word.to_string(),
                paguthi_root: v_rec.root.clone(),
                root_meaning: v_rec.meaning.clone(),
                viguthi_suffix: "None (Base Stem)".to_string(),
                modification_rule: "None: Primal action identifier root match".to_string(),
                semantic_class: NiruktaClass::Akhyata,
            });
        }

        if let Some(n_rec) = self.nominal_bases.iter().find(|n| n.base == word) {
            return Some(SandhiSplitResult {
                original: word.to_string(),
                paguthi_root: n_rec.base.clone(),
                root_meaning: n_rec.meaning.clone(),
                viguthi_suffix: "None (Base Stem)".to_string(),
                modification_rule: "None: Primal substantive noun match".to_string(),
                semantic_class: NiruktaClass::Nama,
            });
        }

        None
    }
}
