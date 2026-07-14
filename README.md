# Tamil Nirukta-Based Sandhi Engine

A high-performance, structural morphological parser written in Rust. This engine tokenizes classical Tamil (*Sentamil* / செந்தமிழ்) text using strict non-destructive phonetic rules, decompiles agglutinative *Sandhi* (புணர்ச்சி) alterations, and classifies isolated roots into semantic categories adapted from Yāska’s *Nirukta* framework and Tolkāppiyar’s *Collatikāram*.

---

## 🗺️ Conceptual Framework & Mapping

This engine bridges classic Indian linguistic philosophy with deterministic software architecture by cross-compiling morphological parts of speech into four universal fields (*Chatvari Padajatani*):

1. **Nāma (Noun) ↔ Peyarchol (பெயர்ச்சொல்):** Concrete entities, abstractions, and derived verbal nouns.
2. **Ākhyāta (Verb) ↔ Vinaichol (வினைச்சொல்):** Dynamic vectors, continuous actions, and temporal state changes.
3. **Upasarga (Prefix Modifiers) ↔ Urichol (உரிச்சொல்):** Qualitative attributes bound to the front of roots to shift intensity.
4. **Nipāta (Particles) ↔ Idaichol (இடைச்சொல்):** Bound morphemes, case inflections, and grammatical linkers.

---

## 🛠️ Project Structure Layout

Ensure your local project directory is structured exactly as follows:

```text
tamil_sandhi_engine/
├── Cargo.toml          # Package metadata and dependencies
├── README.md           # Documentation and operational manual
└── src/
    ├── main.rs         # Application entry point and testing vectors
    ├── token.rs        # Grapheme cluster tokenizer (Unicode processing)
    └── engine.rs       # Reverse-Sandhi state machine and parser loop
```

---

## 🚀 Step-by-Step Local Setup & Execution

### 1. Initialize the Environment Locally
If you haven't created the project folder yet, open your terminal or command prompt and run:
```bash
# Create a fresh binary container framework
cargo new tamil_sandhi_engine --bin
cd tamil_sandhi_engine
```

### 2. Verify Compilation
Ensure your Rust toolchain is up-to-date, then execute the compiler to run the test suite layout embedded within `src/main.rs`:
```bash
cargo run
```

---

## 🐙 How to Synchronize and Push to GitHub

If you run into repository location errors during deployment, follow this exact sequence to wipe out incorrect remote targets and establish a clean connection to your account.

### 1. Find Your GitHub Username
1. Open your browser and log into [github.com](https://github.com).
2. Click your profile avatar in the upper-right corner.
3. The bold string directly under your display name is your unique **GitHub Username**.

### 2. Connect and Upload the Codebase
Create a fresh, blank repository on GitHub's website named `tamil_sandhi_engine`. Do **not** check the boxes to generate a default README or `.gitignore`. 

Then, run the following script inside your terminal (replacing `YOUR_EXACT_USERNAME` with your real account handle):

```bash
# 1. Initialize git tracking configuration
git init

# 2. Exclude binary target outputs from cluttering tracking records
echo "/target" > .gitignore

# 3. Stage and record your structural files locally
git add .
git commit -m "Initial commit: Core Tamil Sandhi decomposition and Nirukta classification architecture"

# 4. Strip out any incomplete or broken home URLs
git remote remove origin 2>/dev/null

# 5. Attach your unique, explicit target repository path
git remote add origin github.com

# 6. Branch and force the synchronization up to your profile cloud
git branch -M main
git push -u origin main
```

---

## 🔬 Under the Hood: Algorithmic Logic

### Non-Destructive Grapheme Cluster Tokenization
Tamil characters are highly agglutinative compound phonetic strings. A raw word like **டை** (*dai*) appears to standard string parsers as two bytes or characters, which routinely corrupts when parsed from right to left. 

This engine uses the `unicode-segmentation` library to group text into whole phonetic units (*Uyirmei* letters), protecting the integrity of individual letters during deep grammatical deconstruction.

### Implemented Reverse-Sandhi Logic Modifiers:
* **Elision Reversal (உயிரீறு கெடுதல்):** Reconstructs base verbal stems by evaluating dropped vowels when moving into a nominalized state (e.g., recovering `ஊடு` from the input string `ஊடல்`).
* **Phonetic Consonant Transformation (திரிதல்):** Traces trailing dental and nasal sound shifts to map words back to their radical dictionary infinitives (e.g., tracking `அறம்` down to its core action root `அறு`).

---

## 📈 Roadmap & Next Iterations
* [ ] **JSON Lexicon System:** Move raw database lookups into external tables (`akhyata_roots.json`, `nama_bases.json`, `uri_modifiers.json`).
* [ ] **Tense Infix Matrix (*Idainilai*):** Implement an engine state lookup to parse past (`த்`, `ட்`), present (`கிறு`, `கின்று`), and future (`ப்`, `வ்`) indicators.
* [ ] **Person-Number-Gender (PNG) Decoding:** Add suffix checks to map grammatical subjects like `-aan` (masculine singular) or `-aal` (feminine singular).
