use unicode_segmentation::UnicodeSegmentation;

/// Tokenizes a Tamil string into distinct, atomic grapheme clusters.
pub fn tokenize_tamil(text: &str) -> Vec<String> {
    text.graphemes(true)
        .map(|s| s.to_string())
        .collect()
}

pub struct Phonetics;

impl Phonetics {
    pub fn is_pure_consonant(token: &str) -> bool {
        let pure_consonants = [
            "க்", "ங்", "ச்", "ஞ்", "ட்", "ண்", "த்", "ந்", "ப்", "ம்", 
            "ய்", "ர்", "ல்", "வ்", "ழ்", "ள்", "ற்", "ன்"
        ];
        pure_consonants.contains(&token)
    }

    pub fn is_primary_vowel(token: &str) -> bool {
        let vowels = ["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "எ", "ஏ", "ஐ", "ஒ", "ஓ", "ஔ"];
        vowels.contains(&token)
    }
}
