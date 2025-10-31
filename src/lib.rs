use std::collections::HashMap;
use std::fmt;

// Public modules
pub mod pos_tagger;
pub mod dependency;

// Re-export commonly used types
pub use pos_tagger::{POSTagger, POSTag};
pub use dependency::{Dependency, DependencyTree, DependencyConverter};

/// Represents a symbol in the grammar (terminal or non-terminal)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    NonTerminal(String),
    Terminal(String),
}

impl Symbol {
    pub fn non_terminal(s: &str) -> Self {
        Symbol::NonTerminal(s.to_string())
    }

    pub fn terminal(s: &str) -> Self {
        Symbol::Terminal(s.to_string())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::NonTerminal(s) => write!(f, "{}", s),
            Symbol::Terminal(s) => write!(f, "'{}'", s),
        }
    }
}

/// Represents the right-hand side of a production rule
#[derive(Debug, Clone, PartialEq)]
pub enum Production {
    /// Terminal production: A -> a
    Terminal(String),
    /// Binary production: A -> B C
    Binary(String, String),
}

/// A single grammar rule with its probability
#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: String,
    pub production: Production,
    pub probability: f64,
}

impl Rule {
    pub fn new_terminal(lhs: &str, terminal: &str, probability: f64) -> Self {
        Rule {
            lhs: lhs.to_string(),
            production: Production::Terminal(terminal.to_string()),
            probability,
        }
    }

    pub fn new_binary(lhs: &str, rhs1: &str, rhs2: &str, probability: f64) -> Self {
        Rule {
            lhs: lhs.to_string(),
            production: Production::Binary(rhs1.to_string(), rhs2.to_string()),
            probability,
        }
    }
}

/// Probabilistic Context-Free Grammar (PCFG)
#[derive(Debug, Clone)]
pub struct PCFG {
    pub start_symbol: String,
    rules: Vec<Rule>,
    /// Maps terminal to rules that produce it
    terminal_rules: HashMap<String, Vec<usize>>,
    /// Maps (B, C) pairs to rules that produce them
    binary_rules: HashMap<(String, String), Vec<usize>>,
}

impl PCFG {
    pub fn new(start_symbol: &str) -> Self {
        PCFG {
            start_symbol: start_symbol.to_string(),
            rules: Vec::new(),
            terminal_rules: HashMap::new(),
            binary_rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        let rule_idx = self.rules.len();

        match &rule.production {
            Production::Terminal(t) => {
                self.terminal_rules
                    .entry(t.clone())
                    .or_insert_with(Vec::new)
                    .push(rule_idx);
            }
            Production::Binary(b, c) => {
                self.binary_rules
                    .entry((b.clone(), c.clone()))
                    .or_insert_with(Vec::new)
                    .push(rule_idx);
            }
        }

        self.rules.push(rule);
    }

    /// Automatically add terminal rules from a POS tagger's lexicon
    pub fn add_lexicon_from_tagger(&mut self, tagger: &pos_tagger::POSTagger) {
        for (word, tags) in tagger.lexicon.iter() {
            for (tag, prob) in tags {
                self.add_rule(Rule::new_terminal(tag, word, *prob));
            }
        }
    }

    /// Get all rules that produce a given terminal
    pub fn get_terminal_rules(&self, terminal: &str) -> Vec<&Rule> {
        self.terminal_rules
            .get(terminal)
            .map(|indices| indices.iter().map(|&i| &self.rules[i]).collect())
            .unwrap_or_default()
    }

    /// Get all rules that produce the pair (B, C)
    pub fn get_binary_rules(&self, b: &str, c: &str) -> Vec<&Rule> {
        self.binary_rules
            .get(&(b.to_string(), c.to_string()))
            .map(|indices| indices.iter().map(|&i| &self.rules[i]).collect())
            .unwrap_or_default()
    }
}

/// Entry in the CYK parse table
#[derive(Debug, Clone)]
struct TableEntry {
    /// Non-terminal symbol
    symbol: String,
    /// Probability of this parse
    probability: f64,
    /// Backpointer for reconstructing parse tree
    /// For terminal: None
    /// For binary: Some((split_point, left_symbol, right_symbol))
    backpointer: Option<(usize, String, String)>,
}

/// Diagnostic information about parse failures
#[derive(Debug, Clone)]
pub struct ParseDiagnostics {
    /// Words that were not recognized by any terminal rule
    pub unknown_words: Vec<(usize, String)>,
    /// Words that were recognized but position info
    pub recognized_words: Vec<(usize, String, Vec<String>)>,
    /// Whether the grammar could parse the full sentence structure
    pub full_parse_attempted: bool,
    /// Partial parses that were possible (span start, span length, non-terminals)
    pub partial_parses: Vec<(usize, usize, Vec<String>)>,
}

impl ParseDiagnostics {
    pub fn new() -> Self {
        ParseDiagnostics {
            unknown_words: Vec::new(),
            recognized_words: Vec::new(),
            full_parse_attempted: false,
            partial_parses: Vec::new(),
        }
    }

    /// Get a human-readable error message
    pub fn error_message(&self) -> String {
        let mut msg = String::from("Parse failed:\n");

        if !self.unknown_words.is_empty() {
            msg.push_str("\nUnknown words (not in grammar):\n");
            for (pos, word) in &self.unknown_words {
                msg.push_str(&format!("  Position {}: '{}'\n", pos, word));
            }
        }

        if !self.recognized_words.is_empty() && self.unknown_words.is_empty() {
            msg.push_str("\nAll words were recognized, but no valid parse tree could be formed.\n");
            msg.push_str("\nWord recognition:\n");
            for (pos, word, symbols) in &self.recognized_words {
                msg.push_str(&format!("  Position {}: '{}' -> [{}]\n",
                    pos, word, symbols.join(", ")));
            }
        }

        if self.full_parse_attempted && !self.partial_parses.is_empty() {
            msg.push_str("\nPartial parses found:\n");
            for (start, len, symbols) in &self.partial_parses {
                msg.push_str(&format!("  Span [{}, {}): [{}]\n",
                    start, start + len + 1, symbols.join(", ")));
            }
            msg.push_str("\nThe grammar rules don't allow these partial parses to combine into a complete sentence.\n");
        }

        msg
    }
}

impl Default for ParseDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of parsing
#[derive(Debug)]
pub struct ParseResult {
    pub success: bool,
    pub probability: f64,
    pub parse_tree: Option<ParseTree>,
    pub diagnostics: ParseDiagnostics,
}

/// Parse tree node
#[derive(Debug, Clone)]
pub enum ParseTree {
    Terminal {
        symbol: String,
        word: String,
    },
    NonTerminal {
        symbol: String,
        probability: f64,
        children: Vec<ParseTree>,
    },
}

impl fmt::Display for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}

impl ParseTree {
    fn fmt_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let spaces = "  ".repeat(indent);
        match self {
            ParseTree::Terminal { symbol, word } => {
                writeln!(f, "{}({} '{}')", spaces, symbol, word)
            }
            ParseTree::NonTerminal {
                symbol,
                probability,
                children,
            } => {
                writeln!(f, "{}({} [p={:.4}]", spaces, symbol, probability)?;
                for child in children {
                    child.fmt_indent(f, indent + 1)?;
                }
                writeln!(f, "{})", spaces)
            }
        }
    }
}

/// Probabilistic CYK Parser
pub struct CYKParser {
    grammar: PCFG,
}

impl CYKParser {
    pub fn new(grammar: PCFG) -> Self {
        CYKParser { grammar }
    }

    /// Parse a sequence of words using the probabilistic CYK algorithm
    pub fn parse(&self, words: &[&str]) -> ParseResult {
        let mut diagnostics = ParseDiagnostics::new();

        if words.is_empty() {
            return ParseResult {
                success: false,
                probability: 0.0,
                parse_tree: None,
                diagnostics,
            };
        }

        let n = words.len();
        // table[i][j] contains possible non-terminals for span from i to i+j+1
        let mut table: Vec<Vec<HashMap<String, TableEntry>>> = vec![vec![HashMap::new(); n]; n];

        // Fill in base case: single words
        for (i, &word) in words.iter().enumerate() {
            let rules = self.grammar.get_terminal_rules(word);

            if rules.is_empty() {
                // Track unknown words
                diagnostics.unknown_words.push((i, word.to_string()));
            } else {
                // Track recognized words and their possible non-terminals
                let symbols: Vec<String> = rules.iter().map(|r| r.lhs.clone()).collect();
                diagnostics.recognized_words.push((i, word.to_string(), symbols));
            }

            for rule in rules {
                let entry = TableEntry {
                    symbol: rule.lhs.clone(),
                    probability: rule.probability,
                    backpointer: None,
                };

                // Keep the highest probability for each non-terminal
                table[i][0]
                    .entry(rule.lhs.clone())
                    .and_modify(|e| {
                        if entry.probability > e.probability {
                            *e = entry.clone();
                        }
                    })
                    .or_insert(entry);
            }
        }

        // Fill in the rest of the table
        for length in 1..n {
            // length is the span size - 1
            for i in 0..n - length {
                // Collect all new entries for this cell first
                let mut new_entries: Vec<TableEntry> = Vec::new();

                // Consider all ways to split the span [i, i+length+1)
                for k in 0..length {
                    // Split at position i+k+1
                    // Left span: [i, i+k+1), Right span: [i+k+1, i+length+1)
                    let left_entries = &table[i][k];
                    let right_entries = &table[i + k + 1][length - k - 1];

                    // Try all combinations of non-terminals from left and right
                    for (b, left_entry) in left_entries {
                        for (c, right_entry) in right_entries {
                            // Find all rules A -> B C
                            for rule in self.grammar.get_binary_rules(b, c) {
                                let prob = rule.probability * left_entry.probability * right_entry.probability;

                                let entry = TableEntry {
                                    symbol: rule.lhs.clone(),
                                    probability: prob,
                                    backpointer: Some((i + k + 1, b.clone(), c.clone())),
                                };

                                new_entries.push(entry);
                            }
                        }
                    }
                }

                // Now insert all the collected entries
                for entry in new_entries {
                    table[i][length]
                        .entry(entry.symbol.clone())
                        .and_modify(|e| {
                            if entry.probability > e.probability {
                                *e = entry.clone();
                            }
                        })
                        .or_insert(entry);
                }
            }
        }

        // Check if we can derive the full sentence from the start symbol
        diagnostics.full_parse_attempted = true;

        if let Some(entry) = table[0][n - 1].get(&self.grammar.start_symbol) {
            let parse_tree = self.reconstruct_tree(&table, words, 0, n - 1, &entry.symbol);
            ParseResult {
                success: true,
                probability: entry.probability,
                parse_tree: Some(parse_tree),
                diagnostics,
            }
        } else {
            // Collect information about partial parses
            // Check what non-terminals we found for the full span
            if !table[0][n - 1].is_empty() {
                let symbols: Vec<String> = table[0][n - 1].keys().cloned().collect();
                diagnostics.partial_parses.push((0, n - 1, symbols));
            }

            // Also collect information about other spans that were successfully parsed
            for length in (1..n).rev() {
                for start in 0..n - length {
                    if !table[start][length].is_empty() {
                        let symbols: Vec<String> = table[start][length].keys().cloned().collect();
                        diagnostics.partial_parses.push((start, length, symbols));
                    }
                }
            }

            ParseResult {
                success: false,
                probability: 0.0,
                parse_tree: None,
                diagnostics,
            }
        }
    }

    /// Reconstruct the parse tree from the CYK table
    fn reconstruct_tree(
        &self,
        table: &[Vec<HashMap<String, TableEntry>>],
        words: &[&str],
        start: usize,
        length: usize,
        symbol: &str,
    ) -> ParseTree {
        let entry = &table[start][length][symbol];

        match &entry.backpointer {
            None => {
                // Terminal production
                ParseTree::Terminal {
                    symbol: symbol.to_string(),
                    word: words[start].to_string(),
                }
            }
            Some((split, left_sym, right_sym)) => {
                // Binary production
                let k = split - start - 1;
                let left_tree = self.reconstruct_tree(table, words, start, k, left_sym);
                let right_tree = self.reconstruct_tree(table, words, *split, length - k - 1, right_sym);

                ParseTree::NonTerminal {
                    symbol: symbol.to_string(),
                    probability: entry.probability,
                    children: vec![left_tree, right_tree],
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simple_grammar() -> PCFG {
        let mut grammar = PCFG::new("S");

        // S -> NP VP [1.0]
        grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));

        // NP -> Det N [0.6]
        grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.6));
        // NP -> "she" [0.4]
        grammar.add_rule(Rule::new_terminal("NP", "she", 0.4));

        // VP -> V NP [0.7]
        grammar.add_rule(Rule::new_binary("VP", "V", "NP", 0.7));
        // VP -> "eats" [0.3]
        grammar.add_rule(Rule::new_terminal("VP", "eats", 0.3));

        // Det -> "the" [1.0]
        grammar.add_rule(Rule::new_terminal("Det", "the", 1.0));

        // N -> "cat" [0.5]
        grammar.add_rule(Rule::new_terminal("N", "cat", 0.5));
        // N -> "dog" [0.5]
        grammar.add_rule(Rule::new_terminal("N", "dog", 0.5));

        // V -> "eats" [1.0]
        grammar.add_rule(Rule::new_terminal("V", "eats", 1.0));

        grammar
    }

    #[test]
    fn test_simple_parse() {
        let grammar = create_simple_grammar();
        let parser = CYKParser::new(grammar);

        let result = parser.parse(&["she", "eats"]);
        assert!(result.success);
        assert!(result.probability > 0.0);
    }

    #[test]
    fn test_longer_parse() {
        let grammar = create_simple_grammar();
        let parser = CYKParser::new(grammar);

        let result = parser.parse(&["she", "eats", "the", "cat"]);
        assert!(result.success);
        assert!(result.probability > 0.0);
    }

    #[test]
    fn test_failed_parse() {
        let grammar = create_simple_grammar();
        let parser = CYKParser::new(grammar);

        let result = parser.parse(&["cat", "cat", "cat"]);
        assert!(!result.success);
    }
}
