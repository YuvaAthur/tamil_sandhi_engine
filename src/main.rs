mod token;
mod engine;

use engine::SandhiEngine;

fn main() {
    let engine = SandhiEngine::new();
    let test_words = vec!["ஊடல்", "அறம்", "ஊடு", "தொடு"];

    println!("================================================================");
    println!("        TAMIL NIRUKTA-BASED SANDHI DECONSTRUCTION ENGINE        ");
    println!("================================================================");

    for word in test_words {
        println!("\nAnalyzing Word Input: {}", word);
        match engine.reverse_sandhi(word) {
            Some(result) => {
                println!("  ├── a/ Paguthi (Root)       : {}", result.paguthi_root);
                println!("  ├── b/ Viguthi (Suffix)     : {}", result.viguthi_suffix);
                println!("  ├── c/ Modification Process : {}", result.modification_rule);
                println!("  ├── d/ Nirukta Classification: {:?}", result.semantic_class);
                println!("  └── e/ System Status        : Successfully Resolved");
            }
            None => {
                println!("  └── Status: Complex architecture / Unknown root registry reference.");
            }
        }
    }
    println!("\n================================================================");
}
