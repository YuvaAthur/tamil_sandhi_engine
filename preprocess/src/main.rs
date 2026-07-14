use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;

#[derive(Serialize, Debug)]
struct VerbRecord {
    root: String,
    meaning: String,
    class: String,
    past_marker: String,
}

#[derive(Serialize, Debug)]
struct VerbDatabase { verbs: Vec<VerbRecord> }

#[derive(Serialize, Debug)]
struct NounRecord {
    base: String,
    meaning: String,
    is_primitive: bool,
}

#[derive(Serialize, Debug)]
struct NounDatabase { nouns: Vec<NounRecord> }

#[derive(Serialize, Debug)]
struct UriRecord {
    token: String,
    meaning: String,
    behavior: String,
}

#[derive(Serialize, Debug)]
struct UriDatabase { modifiers: Vec<UriRecord> }

fn main() {
    println!("[Preprocess] Starting raw data compilation pipeline...");

    // Mock representation of unstructured text lines from ThaniThamizhAkarathiKalanjiyam
    let raw_akarathi_dump = vec![
        "ஊடு | வினை | to sulk in love | weak",
        "அறு | வினை | to cut or define | strong",
        "நட | வினை | to walk | weak",
        "தொடு | வினை | to touch | weak",
        "மரம் | பெயர் | tree | primitive",
        "வானம் | பெயர் | sky | primitive",
        "திங்கள் | பெயர் | moon or month | compound",
        "முகை | பெயர் | flower bud | primitive",
        "நறு | உரி | fragrant | direct_attachment",
        "மா | உரி | great | doubles_consonant",
        "மழ | உரி | infantile | vowel_glide",
    ];

    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    let mut modifiers = Vec::new();

    // Map rows based on parts of speech (Peyar, Vinai, Uri)
    for line in raw_akarathi_dump {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() < 3 { continue; }

        let word = parts[0].to_string();
        let class_tag = parts[1];
        let meaning = parts[2].to_string();

        match class_tag {
            "வினை" => {
                let strength = parts.get(3).unwrap_or(&"weak").to_string();
                let marker = if strength == "strong" { "த்த்" } else { "த்" };
                verbs.push(VerbRecord { root: word, meaning, class: strength, past_marker: marker.to_string() });
            },
            "பெயர்" => {
                let prim_flag = parts.get(3).unwrap_or(&"primitive") == &"primitive";
                nouns.push(NounRecord { base: word, meaning, is_primitive: prim_flag });
            },
            "உரி" => {
                let behavior = parts.get(3).unwrap_or(&"direct_attachment").to_string();
                modifiers.push(UriRecord { token: word, meaning, behavior });
            },
            _ => {}
        }
    }

    // Ensure output target directory wrapper path is available
    create_dir_all("data").expect("Could not build data directory directory location");

    // Write file outputs
    write_json("data/akhyata_roots.json", &VerbDatabase { verbs });
    write_json("data/nama_bases.json", &NounDatabase { nouns });
    write_json("data/uri_modifiers.json", &UriDatabase { modifiers });

    println!("[Preprocess] Complete. Target lexicon classifications generated successfully.");
}

fn write_json<T: Serialize>(file_path: &str, data: &T) {
    let mut file = File::create(file_path).expect("Could not create structural schema JSON output target");
    let payload = serde_json::to_string_pretty(data).expect("Serialization failed on model array conversion");
    file.write_all(payload.as_bytes()).expect("Write I/O pipeline fault caught");
    println!("  ├── Generated database output asset -> {}", file_path);
}
