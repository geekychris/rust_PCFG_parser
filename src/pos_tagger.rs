use std::collections::HashMap;

/// Part-of-speech tag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct POSTag {
    pub word: String,
    pub tag: String,
}

impl POSTag {
    pub fn new(word: &str, tag: &str) -> Self {
        POSTag {
            word: word.to_string(),
            tag: tag.to_string(),
        }
    }
}

/// Simple dictionary-based POS tagger
pub struct POSTagger {
    pub lexicon: HashMap<String, Vec<(String, f64)>>,
    default_tag: String,
}

impl POSTagger {
    pub fn new() -> Self {
        let mut lexicon = HashMap::new();

        // Pronouns
        lexicon.insert("she".to_string(), vec![("NP".to_string(), 1.0)]);
        lexicon.insert("he".to_string(), vec![("NP".to_string(), 1.0)]);
        lexicon.insert("i".to_string(), vec![("NP".to_string(), 1.0)]);
        lexicon.insert("they".to_string(), vec![("NP".to_string(), 1.0)]);
        lexicon.insert("we".to_string(), vec![("NP".to_string(), 1.0)]);

        // Determiners
        lexicon.insert("the".to_string(), vec![("Det".to_string(), 1.0)]);
        lexicon.insert("a".to_string(), vec![("Det".to_string(), 1.0)]);
        lexicon.insert("an".to_string(), vec![("Det".to_string(), 1.0)]);

        // Nouns
        lexicon.insert("cat".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("dog".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("mouse".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("cheese".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("ball".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("book".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("house".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("car".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("man".to_string(), vec![("N".to_string(), 1.0)]);
        lexicon.insert("woman".to_string(), vec![("N".to_string(), 1.0)]);

        // Verbs
        lexicon.insert("chases".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("sees".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("eats".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("sleeps".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("runs".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("walks".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("reads".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("writes".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("likes".to_string(), vec![("V".to_string(), 1.0)]);
        lexicon.insert("loves".to_string(), vec![("V".to_string(), 1.0)]);

        // Adjectives
        lexicon.insert("big".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("small".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("lazy".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("happy".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("sad".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("quick".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("slow".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("red".to_string(), vec![("Adj".to_string(), 1.0)]);
        lexicon.insert("blue".to_string(), vec![("Adj".to_string(), 1.0)]);

        // Prepositions
        lexicon.insert("with".to_string(), vec![("P".to_string(), 1.0)]);
        lexicon.insert("in".to_string(), vec![("P".to_string(), 1.0)]);
        lexicon.insert("on".to_string(), vec![("P".to_string(), 1.0)]);
        lexicon.insert("at".to_string(), vec![("P".to_string(), 1.0)]);
        lexicon.insert("to".to_string(), vec![("P".to_string(), 1.0)]);
        lexicon.insert("from".to_string(), vec![("P".to_string(), 1.0)]);

        POSTagger {
            lexicon,
            default_tag: "N".to_string(), // Default to noun for unknown words
        }
    }

    /// Tag a sentence (list of words)
    pub fn tag(&self, words: &[&str]) -> Vec<POSTag> {
        words
            .iter()
            .map(|&word| {
                let word_lower = word.to_lowercase();
                let tag = self.lexicon
                    .get(&word_lower)
                    .and_then(|tags| tags.first())
                    .map(|(tag, _)| tag.clone())
                    .unwrap_or_else(|| self.default_tag.clone());

                POSTag::new(word, &tag)
            })
            .collect()
    }

    /// Get the most likely tag for a word
    pub fn get_tag(&self, word: &str) -> String {
        let word_lower = word.to_lowercase();
        self.lexicon
            .get(&word_lower)
            .and_then(|tags| tags.first())
            .map(|(tag, _)| tag.clone())
            .unwrap_or_else(|| self.guess_tag_from_morphology(&word_lower))
    }

    /// Guess POS tag based on morphological patterns
    fn guess_tag_from_morphology(&self, word: &str) -> String {
        // Verb patterns
        if word.ends_with("ed") || word.ends_with("ing") {
            return "V".to_string();
        }
        if word.ends_with('s') && word.len() > 2 {
            // Could be plural noun or third-person verb
            // Simple heuristic: if it ends in common verb patterns, tag as verb
            if word.ends_with("es") || word.ends_with("ies") {
                return "V".to_string();
            }
            // Otherwise assume plural noun
            return "N".to_string();
        }

        // Adjective patterns
        if word.ends_with("ly") {
            return "Adv".to_string();
        }
        if word.ends_with("ous") || word.ends_with("ive") || word.ends_with("ful") || word.ends_with("less") {
            return "Adj".to_string();
        }

        // Noun patterns
        if word.ends_with("tion") || word.ends_with("ness") || word.ends_with("ment") || word.ends_with("ity") {
            return "N".to_string();
        }

        // Default to noun for unknown words
        self.default_tag.clone()
    }

    /// Add a word to the lexicon
    pub fn add_word(&mut self, word: &str, tag: &str, probability: f64) {
        self.lexicon
            .entry(word.to_lowercase())
            .or_insert_with(Vec::new)
            .push((tag.to_string(), probability));
    }

    /// Load words from a text file (format: "word TAG probability" per line)
    /// Example: "jumped V 1.0"
    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<usize> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            if parts.len() >= 2 {
                let word = parts[0];
                let tag = parts[1];
                let prob = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1.0);

                self.add_word(word, tag, prob);
                count += 1;
            }
        }

        Ok(count)
    }

    /// Create an expanded tagger with more vocabulary
    pub fn new_expanded() -> Self {
        let mut tagger = Self::new();

        // Add more verbs (various tenses)
        let verbs = vec![
            ("jumped", "V"), ("jump", "V"), ("jumps", "V"), ("jumping", "V"),
            ("ran", "V"), ("run", "V"), ("runs", "V"), ("running", "V"),
            ("walked", "V"), ("walk", "V"), ("walks", "V"), ("walking", "V"),
            ("chased", "V"), ("chase", "V"), ("chases", "V"), ("chasing", "V"),
        ];
        for (word, tag) in verbs {
            tagger.add_word(word, tag, 1.0);
        }

        // Add more nouns
        let nouns = vec![
            ("fox", "N"), ("foxes", "N"), ("dogs", "N"), ("cats", "N"),
            ("bird", "N"), ("birds", "N"), ("tree", "N"), ("trees", "N"),
            ("person", "N"), ("people", "N"), ("child", "N"), ("children", "N"),
        ];
        for (word, tag) in nouns {
            tagger.add_word(word, tag, 1.0);
        }

        // Add more adjectives
        let adjectives = vec![
            ("brown", "Adj"), ("black", "Adj"), ("white", "Adj"),
            ("green", "Adj"), ("yellow", "Adj"), ("old", "Adj"),
            ("young", "Adj"), ("new", "Adj"), ("good", "Adj"),
        ];
        for (word, tag) in adjectives {
            tagger.add_word(word, tag, 1.0);
        }

        // Add more prepositions
        let prepositions = vec![
            ("over", "P"), ("under", "P"), ("above", "P"), ("below", "P"),
            ("near", "P"), ("far", "P"), ("between", "P"), ("through", "P"),
        ];
        for (word, tag) in prepositions {
            tagger.add_word(word, tag, 1.0);
        }

        tagger
    }
}

impl Default for POSTagger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tagging() {
        let tagger = POSTagger::new();
        let tags = tagger.tag(&["the", "cat", "sleeps"]);

        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].tag, "Det");
        assert_eq!(tags[1].tag, "N");
        assert_eq!(tags[2].tag, "V");
    }

    #[test]
    fn test_unknown_word() {
        let tagger = POSTagger::new();
        let tag = tagger.get_tag("unknown");

        assert_eq!(tag, "N"); // Default tag
    }
}
