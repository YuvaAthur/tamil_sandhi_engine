mod token;
mod engine;

use engine::SandhiEngine;

fn main() {
    // Initializes the parser core which reads our JSON datasets dynamically
    let engine = SandhiEngine::new();
    
    // Testing array now includes compound descriptive nouns like நறுமுகை
    let test_words = vec!["ஊடல்", "அறம்", "நறுமுகை", "வானம்", "நட"];

    println!("================================================================");
    println!("    TAMIL NIRUKTA ENGINE - DYNAMIC LEXICAL SYSTEMS MODULE     ");
    println!("================================================================");
    println!("Loaded Lexicon Totals -> Verbs: {}, Nouns: {}, Modifiers: {}", 
             engine.verbal_roots.len(), engine.nominal_bases.len(), engine.uri_modifiers.len());

    for word in test_words {
        println!("\nEvaluating Input Vector: {}", word);
        match engine.reverse_sandhi(word) {
            Some(result) => {
                println!("  ├── a/ Paguthi (Root Stem)  : {}", result.paguthi_root);
                println!("  ├── b/ Radical Core Meaning : {}", result.root_meaning);
                println!("  ├── c/ Viguthi (Suffix Target): {}", result.viguthi_suffix);
                println!("  ├── d/ Modification Process : {}", result.modification_rule);
                println!("  └── e/ Nirukta Category     : {:?}", result.semantic_class);
            }
            None => {
                println!("  └── [Status] Unresolved: Morpheme fingerprint signatures not found in database registry.");
            }
        }
    }
    println!("\n================================================================");
}
