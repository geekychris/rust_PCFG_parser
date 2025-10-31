# Probabilistic CYK Parser

A Rust implementation of the probabilistic CYK (Cocke-Younger-Kasami) parsing algorithm for Probabilistic Context-Free Grammars (PCFGs).

## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Building and Running](#building-and-running)
- [Design Overview](#design-overview)
- [Usage Guide](#usage-guide)
- [Extending the Parser](#extending-the-parser)
- [Algorithm Details](#algorithm-details)
- [Examples](#examples)
- [Publishing and Hosting](#publishing-and-hosting-the-library)
- [Contributing](#contributing)

## Features

- Efficient probabilistic parsing using the CYK algorithm
- Support for Probabilistic Context-Free Grammars (PCFGs)
- Parse tree reconstruction with probability tracking
- **Part-of-Speech (POS) tagging** for sentence analysis
- **Expanded vocabulary support** with automatic lexicon loading
- **Morphological heuristics** for unknown word tagging
- **Detailed parse diagnostics** to explain failures
- **Dependency tree extraction** from constituency parses
- **Interactive sentence parser** with full pipeline demonstration
- **Automatic grammar rule generation** from POS tagger
- Clean API for defining grammars and parsing sentences
- Optimized rule lookup using hash maps
- Comprehensive test coverage

## Quick Start

### Basic Parsing

```rust
use cyk_parser::{CYKParser, Rule, PCFG};

// Create a simple grammar
let mut grammar = PCFG::new("S");
grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));
grammar.add_rule(Rule::new_terminal("NP", "she", 0.4));
grammar.add_rule(Rule::new_terminal("VP", "sleeps", 0.3));

// Parse a sentence
let parser = CYKParser::new(grammar);
let result = parser.parse(&["she", "sleeps"]);

if result.success {
    println!("Probability: {}", result.probability);
    println!("{}", result.parse_tree.unwrap());
}
```

### Full Pipeline with POS Tagging and Dependency Trees

```rust
use cyk_parser::{CYKParser, DependencyConverter, POSTagger, Rule, PCFG};

// Create a POS tagger
let tagger = POSTagger::new();

// Tag a sentence
let words = vec!["the", "cat", "sleeps"];
let pos_tags = tagger.tag(&words);

// Parse with grammar
let parser = CYKParser::new(grammar);
let result = parser.parse(&words);

// Extract dependency tree
if let Some(tree) = result.parse_tree {
    let dep_converter = DependencyConverter::new();
    if let Some(dep_tree) = dep_converter.convert(&tree) {
        println!("Root: {}", dep_tree.root);
        for dep in &dep_tree.dependencies {
            println!("{}", dep);
        }
    }
}
```

## Building and Running

### Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs))
- Cargo (comes with Rust)

### Building the Library

```bash
# Clone or navigate to the project directory
cd rust_cyk_parser

# Build the library in debug mode (faster compilation, slower runtime)
cargo build

# Build with optimizations (for production use)
cargo build --release

# Build documentation
cargo doc --open

# Check the code without building
cargo check
```

The release build will be significantly faster at runtime (typically 10-100x) but takes longer to compile. Use debug builds during development and release builds for production.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_simple_parse
```

### Running Examples

```bash
# Run the simple example
cargo run --example simple

# Run the comprehensive demo
cargo run --example demo

# Run the expanded vocabulary demo (handles "the quick brown fox jumped...")
cargo run --example expanded_vocabulary

# Run the full pipeline demo (POS tagging + parsing + dependencies)
cargo run --example full_pipeline

# Run the interactive parser (enter your own sentences)
cargo run --example interactive
```

### Using as a Library in Your Projects

There are three ways to use this library in your Rust projects:

#### Option 1: Local Path Dependency (for development)

Add to your `Cargo.toml`:

```toml
[dependencies]
cyk_parser = { path = "../rust_cyk_parser" }
```

This is useful when you're actively developing both the library and your application.

#### Option 2: Git Dependency (recommended before publishing)

Add to your `Cargo.toml`:

```toml
[dependencies]
cyk_parser = { git = "https://github.com/yourusername/rust_cyk_parser", branch = "master" }
```

Or pin to a specific commit or tag:

```toml
[dependencies]
# Using a specific tag
cyk_parser = { git = "https://github.com/yourusername/rust_cyk_parser", tag = "v0.1.0" }

# Using a specific commit
cyk_parser = { git = "https://github.com/yourusername/rust_cyk_parser", rev = "abc123def" }
```

#### Option 3: Published Crate (once published to crates.io)

Add to your `Cargo.toml`:

```toml
[dependencies]
cyk_parser = "0.1.0"

# Or with features (if you add them later)
cyk_parser = { version = "0.1.0", features = ["serde"] }
```

#### Basic Usage in Your Project

```rust
// In your main.rs or lib.rs
use cyk_parser::{CYKParser, PCFG, Rule, POSTagger};

fn main() {
    // Create your grammar
    let mut grammar = PCFG::new("S");
    grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));
    // ... add more rules

    // Parse sentences
    let parser = CYKParser::new(grammar);
    let result = parser.parse(&["the", "cat", "sleeps"]);

    if result.success {
        println!("Parse succeeded with probability: {}", result.probability);
    }
}
```

## New Features

### Expanding Vocabulary

The parser now supports **three ways** to expand vocabulary:

#### 1. Expanded Built-in Lexicon

Use the expanded tagger with more vocabulary out of the box:

```rust
use cyk_parser::POSTagger;

// Create tagger with expanded vocabulary
let tagger = POSTagger::new_expanded();

// Now handles words like: jumped, brown, fox, over, lazy, dogs, etc.
let tags = tagger.tag(&["the", "quick", "brown", "fox"]);
```

#### 2. Load Vocabulary from Files

Load thousands of words from external lexicon files:

```rust
let mut tagger = POSTagger::new_expanded();

// Load from file (format: "word TAG probability")
tagger.load_from_file("vocabulary.txt").unwrap();

// Example vocabulary.txt:
// jumped V 1.0
// beautiful Adj 1.0
// elephant N 1.0
```

See `examples/vocabulary.txt` for a sample file format.

#### 3. Automatic Grammar Generation

Automatically generate grammar rules from the tagger's lexicon:

```rust
use cyk_parser::{PCFG, POSTagger};

let tagger = POSTagger::new_expanded();
let mut grammar = PCFG::new("S");

// Add structural rules manually
grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));
// ... more structural rules ...

// Automatically add ALL terminal rules from tagger
grammar.add_lexicon_from_tagger(&tagger);

// Now the grammar has hundreds of words!
```

Run `cargo run --example expanded_vocabulary` to see this in action.

### Morphological Heuristics

Unknown words are automatically tagged using morphological patterns:

```rust
// Words ending in "ed" or "ing" → Verb
"jumped"  → V
"running" → V

// Words ending in "ly" → Adverb
"quickly" → Adv

// Words ending in "ous", "ive", "ful" → Adjective
"beautiful" → Adj
"creative"  → Adj

// Words ending in "tion", "ness", "ment" → Noun
"celebration" → N
"happiness"   → N
```

This significantly improves coverage for words not in the lexicon.

### Detailed Parse Diagnostics

When parsing fails, get clear explanations:

```rust
let result = parser.parse(&["the", "unknown", "word"]);

if !result.success {
    println!("{}", result.diagnostics.error_message());
}
```

Output shows:
- **Unknown words** not in the grammar
- **Recognized words** and their POS tags
- **Partial parses** that succeeded
- **Why the parse failed** (grammar structure issues vs. vocabulary)

Example output:
```
Parse failed:

Unknown words (not in grammar):
  Position 3: 'fox'
  Position 4: 'jumped'

All words were recognized, but no valid parse tree could be formed.
Partial parses found:
  Span [0, 3): [NP]
  Span [4, 7): [VP]

The grammar rules don't allow these partial parses to combine into a complete sentence.
```

### Part-of-Speech (POS) Tagging

The POS tagger provides dictionary-based tagging for sentence analysis:

```rust
use cyk_parser::POSTagger;

let tagger = POSTagger::new();
let tags = tagger.tag(&["the", "cat", "sleeps"]);

for tag in tags {
    println!("{} -> {}", tag.word, tag.tag);
}
// Output:
// the -> Det
// cat -> N
// sleeps -> V
```

The tagger includes a built-in lexicon with common words and can be extended:

```rust
let mut tagger = POSTagger::new();
tagger.add_word("running", "V", 1.0);
```

### Dependency Tree Extraction

Convert constituency parse trees to dependency trees:

```rust
use cyk_parser::DependencyConverter;

let dep_converter = DependencyConverter::new();

// After parsing...
if let Some(tree) = result.parse_tree {
    if let Some(dep_tree) = dep_converter.convert(&tree) {
        println!("Root: {}", dep_tree.root);

        for dep in &dep_tree.dependencies {
            println!("{} --[{}]--> {}", dep.head, dep.relation, dep.dependent);
        }
    }
}
```

The dependency converter uses head-finding rules to identify grammatical relationships:
- `nsubj`: Nominal subject
- `dobj`: Direct object
- `det`: Determiner
- `amod`: Adjectival modifier
- `prep`: Prepositional modifier
- `pobj`: Object of preposition

### Interactive Parser

Run the interactive example to parse your own sentences:

```bash
cargo run --example interactive
```

This launches an interactive session where you can:
1. Enter sentences in natural language
2. See part-of-speech tags for each word
3. View the constituency parse tree
4. Examine the dependency tree structure

## Design Overview

### Architecture

The parser is built around four main components:

```
┌─────────────────────────────────────────────────────┐
│                    CYKParser                         │
│  - Orchestrates the parsing algorithm               │
│  - Manages the dynamic programming table            │
│  - Reconstructs parse trees                         │
└────────────────┬────────────────────────────────────┘
                 │ uses
                 ▼
┌─────────────────────────────────────────────────────┐
│                      PCFG                            │
│  - Stores grammar rules                             │
│  - Indexes rules for efficient lookup               │
│  - Maps terminals → rules                           │
│  - Maps (B, C) pairs → rules                        │
└────────────────┬────────────────────────────────────┘
                 │ contains
                 ▼
┌─────────────────────────────────────────────────────┐
│                      Rule                            │
│  - Represents a single production rule              │
│  - Stores LHS, RHS, and probability                 │
└─────────────────────────────────────────────────────┘
```

### Core Data Structures

#### 1. **PCFG (Probabilistic Context-Free Grammar)**

Located in `src/lib.rs:66-122`

```rust
pub struct PCFG {
    pub start_symbol: String,
    rules: Vec<Rule>,
    terminal_rules: HashMap<String, Vec<usize>>,
    binary_rules: HashMap<(String, String), Vec<usize>>,
}
```

**Design decisions:**
- Uses **index-based storage** for rules to avoid cloning
- **Hash maps** for O(1) rule lookup by terminal or non-terminal pair
- Separates terminal and binary rules for efficient querying

#### 2. **Rule (Production Rule)**

Located in `src/lib.rs:40-63`

```rust
pub struct Rule {
    pub lhs: String,              // Left-hand side non-terminal
    pub production: Production,   // Right-hand side
    pub probability: f64,         // Probability of this production
}

pub enum Production {
    Terminal(String),             // A → "word"
    Binary(String, String),       // A → B C
}
```

**Design decisions:**
- Uses enum for type-safe representation of CNF rules
- Stores probability directly with each rule
- Provides convenient constructors `new_terminal` and `new_binary`

#### 3. **CYKParser (Parser Engine)**

Located in `src/lib.rs:188-322`

```rust
pub struct CYKParser {
    grammar: PCFG,
}
```

**Key methods:**
- `parse(&self, words: &[&str]) -> ParseResult`: Main parsing function
- `reconstruct_tree(...)`: Builds parse tree from backpointers

**Internal structure:**
- Uses a 3D dynamic programming table: `Vec<Vec<HashMap<String, TableEntry>>>`
- `table[i][j]` contains all non-terminals that can derive `words[i..i+j+1]`
- Each entry tracks: symbol, probability, and backpointer for reconstruction

#### 4. **ParseTree (Parse Result)**

Located in `src/lib.rs:146-185`

```rust
pub enum ParseTree {
    Terminal { symbol: String, word: String },
    NonTerminal {
        symbol: String,
        probability: f64,
        children: Vec<ParseTree>
    },
}
```

**Design decisions:**
- Recursive structure for easy traversal
- Stores probability at each non-terminal node
- Implements `Display` trait for pretty-printing

### Algorithm Flow

```
1. Input: words = ["the", "cat", "sleeps"]

2. Initialize table (n×n where n = len(words))

3. Base case: Fill diagonal with terminal productions
   table[0][0] ← {Det: 0.6}  (the)
   table[1][0] ← {N: 0.3}     (cat)
   table[2][0] ← {VP: 0.1}    (sleeps)

4. Fill table bottom-up by increasing span length
   For each span [i, i+length]:
     For each split point k:
       For each (B, C) in left×right spans:
         Find rules A → B C
         Calculate prob = P(A→BC) × P(B) × P(C)
         Keep max probability for each A

5. Check if start symbol is in table[0][n-1]

6. Reconstruct parse tree using backpointers
```

### Key Design Decisions

1. **Chomsky Normal Form Requirement**: Simplifies the algorithm and guarantees O(n³) complexity

2. **Probability Tracking**: Uses floating-point multiplication; may underflow for very long sentences (could be improved with log probabilities)

3. **Greedy Selection**: Keeps only the highest probability parse for each non-terminal at each span (could be extended to track k-best parses)

4. **Memory Management**: Collects entries into a vector before inserting to avoid borrow checker issues (trade-off between clarity and performance)

5. **Indexed Grammar Rules**: Stores rules once and uses indices to avoid cloning during parsing

## Usage Guide

### Creating a Grammar

Grammars must be in **Chomsky Normal Form (CNF)**:

```rust
use cyk_parser::{PCFG, Rule};

let mut grammar = PCFG::new("S"); // Start symbol

// Binary productions: A → B C
grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));
grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.6));

// Terminal productions: A → "word"
grammar.add_rule(Rule::new_terminal("Det", "the", 1.0));
grammar.add_rule(Rule::new_terminal("N", "cat", 0.5));
grammar.add_rule(Rule::new_terminal("N", "dog", 0.5));
```

**Important:** Probabilities for all productions from the same LHS should sum to ≤ 1.0

### Parsing Sentences

```rust
use cyk_parser::CYKParser;

let parser = CYKParser::new(grammar);
let result = parser.parse(&["the", "cat"]);

match result.success {
    true => {
        println!("Parse succeeded!");
        println!("Probability: {:.6}", result.probability);

        if let Some(tree) = result.parse_tree {
            println!("Parse tree:\n{}", tree);
        }
    }
    false => {
        println!("No valid parse found");
    }
}
```

### Working with Parse Trees

```rust
use cyk_parser::ParseTree;

fn count_terminals(tree: &ParseTree) -> usize {
    match tree {
        ParseTree::Terminal { .. } => 1,
        ParseTree::NonTerminal { children, .. } => {
            children.iter().map(|c| count_terminals(c)).sum()
        }
    }
}

fn get_probability(tree: &ParseTree) -> f64 {
    match tree {
        ParseTree::Terminal { .. } => 1.0,
        ParseTree::NonTerminal { probability, .. } => *probability,
    }
}
```

## Extending the Parser

### Adding Support for Unary Productions

The current implementation only supports CNF. To add unary productions (A → B):

1. **Add to Production enum** (`src/lib.rs:31-37`):

```rust
pub enum Production {
    Terminal(String),
    Binary(String, String),
    Unary(String),  // Add this
}
```

2. **Update Rule constructor** (`src/lib.rs:47-63`):

```rust
impl Rule {
    pub fn new_unary(lhs: &str, rhs: &str, probability: f64) -> Self {
        Rule {
            lhs: lhs.to_string(),
            production: Production::Unary(rhs.to_string()),
            probability,
        }
    }
}
```

3. **Add indexing in PCFG** (`src/lib.rs:66-105`):

```rust
pub struct PCFG {
    // ... existing fields ...
    unary_rules: HashMap<String, Vec<usize>>,
}
```

4. **Update parse algorithm** (`src/lib.rs:232-277`):

Add a closure step after filling each cell to handle unary chains.

### Adding k-Best Parsing

To track multiple parse trees instead of just the best:

1. **Modify TableEntry** (`src/lib.rs:125-135`):

```rust
struct TableEntry {
    symbol: String,
    parses: Vec<(f64, Option<(usize, String, String)>)>, // k-best
}
```

2. **Update table insertion logic** to maintain a priority queue of k-best parses

3. **Modify reconstruct_tree** to accept a parse index

### Adding Lexical Probabilities

To handle unknown words with probabilistic lexicon:

1. **Add a lexicon struct**:

```rust
pub struct Lexicon {
    word_probs: HashMap<String, HashMap<String, f64>>,
    unknown_word_handler: Box<dyn Fn(&str) -> Vec<(String, f64)>>,
}
```

2. **Modify parse method** to query lexicon for unknown words

3. **Use smoothing** techniques (add-k, Kneser-Ney) for robustness

### Adding Semantic Actions

To compute semantics during parsing:

1. **Add semantic values to ParseTree**:

```rust
pub enum ParseTree {
    Terminal {
        symbol: String,
        word: String,
        semantics: Value,
    },
    NonTerminal {
        symbol: String,
        probability: f64,
        children: Vec<ParseTree>,
        semantics: Value,
    },
}
```

2. **Add semantic rules** to each grammar production

3. **Compute semantics bottom-up** during reconstruction

### Optimizing for Performance

**Current bottlenecks:**
- String cloning in table operations
- HashMap lookups in inner loops

**Optimization strategies:**

1. **Use string interning**:
```rust
use string_cache::DefaultAtom as Atom;
// Replace String with Atom throughout
```

2. **Replace HashMap with perfect hashing** for static grammars

3. **Use parallel processing** for independent cells:
```rust
use rayon::prelude::*;
// Parallelize outer loops
```

4. **Switch to log probabilities** to avoid underflow:
```rust
let log_prob = rule.log_probability +
               left_entry.log_probability +
               right_entry.log_probability;
```

### Adding Grammar File Format

To load grammars from files:

1. **Define a grammar format** (e.g., simple text format):
```
S -> NP VP [1.0]
NP -> Det N [0.6]
Det -> "the" [1.0]
```

2. **Implement a parser**:

```rust
pub fn load_grammar(path: &str) -> Result<PCFG, ParseError> {
    let content = std::fs::read_to_string(path)?;
    let mut grammar = PCFG::new("S");

    for line in content.lines() {
        let rule = parse_rule_line(line)?;
        grammar.add_rule(rule);
    }

    Ok(grammar)
}
```

3. **Add error handling** for malformed grammars

### Adding Interactive Mode

Create a REPL for testing:

```rust
// examples/repl.rs
use std::io::{self, Write};

fn main() {
    let grammar = load_default_grammar();
    let parser = CYKParser::new(grammar);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let words: Vec<&str> = input.trim().split_whitespace().collect();
        let result = parser.parse(&words);

        // Display result...
    }
}
```

## Algorithm Details

### The CYK Algorithm

The CYK (Cocke-Younger-Kasami) algorithm is a dynamic programming algorithm for parsing strings according to a context-free grammar in Chomsky Normal Form.

**Time Complexity:** O(n³ × |G|)
- `n` = length of input sentence
- `|G|` = number of grammar rules

**Space Complexity:** O(n² × |N|)
- `|N|` = number of non-terminal symbols

### Probabilistic Extension

The probabilistic variant tracks the probability of each parse:

1. **Base case:** Terminal productions use their rule probability
2. **Recursive case:** For A → B C:
   ```
   P(A derives words[i..j]) = P(A → B C) × P(B) × P(C)
   ```
3. **Ambiguity:** Keep only the maximum probability parse for each non-terminal

### Chomsky Normal Form

All rules must be in one of two forms:

1. **Binary:** A → B C (two non-terminals)
2. **Terminal:** A → a (single terminal)

**Converting to CNF:**
- Long productions (A → B C D) require new non-terminals
- Unary productions (A → B) can be eliminated via closure
- Mixed productions (A → B "word") must be split

Example conversion:
```
S → NP VP PP       =>    S → NP X1
                         X1 → VP PP

VP → V "quickly"   =>    VP → V Q
                         Q → "quickly"
```

## Examples

### Example 1: Simple Grammar

See `examples/simple.rs` for a basic example with:
- 2 non-terminals (NP, VP)
- Simple sentence structures
- ~10 grammar rules

Run with: `cargo run --example simple`

### Example 2: Comprehensive Demo

See `examples/demo.rs` for an extended example with:
- 8 non-terminals (S, NP, VP, Det, N, V, Adj, PP)
- Complex sentence structures with adjectives and PPs
- ~20 grammar rules
- Multiple test sentences

Run with: `cargo run --example demo`

### Example 3: Expanded Vocabulary

See `examples/expanded_vocabulary.rs` for vocabulary expansion with:
- POSTagger with expanded built-in vocabulary
- Morphological heuristics for unknown words
- Automatic grammar rule generation from lexicon
- Support for complex sentences like "the quick brown fox jumped over the lazy dogs"
- Detailed diagnostic messages for parse failures

Run with: `cargo run --example expanded_vocabulary`

Sample output:
```
Parsing: "the quick brown fox jumped over the lazy dogs"
============================================================

POS Tags:
  0: the -> Det
  1: quick -> Adj
  2: brown -> Adj
  3: fox -> N
  4: jumped -> V
  5: over -> P
  6: the -> Det
  7: lazy -> Adj
  8: dogs -> N

Parsing...
✓ SUCCESS (probability: 0.002250)

Parse Tree:
(S [p=0.0022]
  (NP [p=0.0750]
    (Det 'the')
    (AdjP [p=0.2500]
      (Adj 'quick')
      (AdjP [p=0.5000]
        (Adj 'brown')
        (N 'fox'))))
  (VP [p=0.0300]
    (V 'jumped')
    (PP [p=0.1500]
      (P 'over')
      (NP [p=0.1500]
        (Det 'the')
        (AdjP [p=0.5000]
          (Adj 'lazy')
          (N 'dogs'))))))
```

### Example 4: Full Pipeline

See `examples/full_pipeline.rs` for a complete demonstration of:
- POS tagging
- Constituency parsing with CYK
- Dependency tree extraction
- Step-by-step output showing each stage

Run with: `cargo run --example full_pipeline`

Sample output:
```
1. Part-of-Speech Tagging:
   the → Det
   big → Adj
   cat → N
   chases → V
   the → Det
   mouse → N

2. Constituency Parsing (CYK):
   Parse: SUCCESS
   Probability: 0.021000

3. Constituency Parse Tree:
(S [p=0.0210]
  (NP [p=0.1000]
    (Det 'the')
    (AN [p=0.5000]
      (Adj 'big')
      (N 'cat')))
  (VP [p=0.2100]
    (V 'chases')
    (NP [p=0.3000]
      (Det 'the')
      (N 'mouse'))))

4. Dependency Tree:
   Root: chases
   Dependencies:
     chases --[nsubj]--> cat
     cat --[det]--> the
     cat --[amod]--> big
     chases --[dobj]--> mouse
     mouse --[det]--> the
```

### Example 5: Interactive Parser

See `examples/interactive.rs` for an interactive command-line interface:
- Enter your own sentences
- Real-time POS tagging, parsing, and dependency extraction
- Visual dependency tree display

Run with: `cargo run --example interactive`

### Example 6: Custom Grammar (Exercise)

Try creating a grammar for mathematical expressions:

```rust
let mut grammar = PCFG::new("Expr");

// Expr → Expr Op Expr
grammar.add_rule(Rule::new_binary("Expr", "Expr", "OpExpr", 0.6));
grammar.add_rule(Rule::new_binary("OpExpr", "Op", "Expr", 1.0));

// Expr → num
grammar.add_rule(Rule::new_terminal("Expr", "1", 0.1));
grammar.add_rule(Rule::new_terminal("Expr", "2", 0.1));
// ... more numbers ...

// Op → +|-|*|/
grammar.add_rule(Rule::new_terminal("Op", "+", 0.25));
grammar.add_rule(Rule::new_terminal("Op", "-", 0.25));
grammar.add_rule(Rule::new_terminal("Op", "*", 0.25));
grammar.add_rule(Rule::new_terminal("Op", "/", 0.25));
```

## Testing

### Unit Tests

Located in `src/lib.rs:324-386`

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture --test-threads=1
```

**Test coverage:**
- Basic 2-word sentences
- Complex multi-word sentences
- Invalid sentence rejection
- Probability calculation

### Adding New Tests

```rust
#[test]
fn test_ambiguous_parse() {
    let mut grammar = PCFG::new("S");

    // Create an ambiguous grammar
    grammar.add_rule(Rule::new_binary("S", "S", "S", 0.5));
    grammar.add_rule(Rule::new_terminal("S", "a", 0.5));

    let parser = CYKParser::new(grammar);
    let result = parser.parse(&["a", "a", "a"]);

    assert!(result.success);
    // Should pick most probable parse
}
```

## Troubleshooting

### Common Issues

**Issue:** Parser returns `success: false` for valid sentence
- **Check:** Grammar is in CNF
- **Check:** All words in sentence have terminal productions
- **Check:** Start symbol can derive the sentence structure

**Issue:** Very low probabilities (near 0)
- **Cause:** Probability underflow from multiplication
- **Solution:** Consider using log probabilities

**Issue:** Compilation errors with lifetimes
- **Cause:** Borrowing table while trying to mutate it
- **Solution:** Collect entries first, then insert (as implemented)

### Debugging Tips

1. **Enable debug output** in parse method:
```rust
println!("table[{}][{}]: {:?}", i, length, table[i][length]);
```

2. **Verify grammar rules**:
```rust
println!("Terminal rules for 'the': {:?}",
         grammar.get_terminal_rules("the"));
```

3. **Check parse table** after parsing

## Publishing and Hosting the Library

If you want to share this library with the Rust community, here are your options:

### Option 1: Publishing to crates.io (Recommended)

**crates.io** is the official Rust package registry. Publishing here makes your library discoverable and easy to use.

#### Prerequisites

1. Create an account at [crates.io](https://crates.io/)
2. Get your API token from [crates.io/me](https://crates.io/me)
3. Login via cargo:

```bash
cargo login <your-api-token>
```

#### Before Publishing

1. **Update Cargo.toml** with complete metadata:

```toml
[package]
name = "cyk_parser"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "A probabilistic CYK parser for Probabilistic Context-Free Grammars"
documentation = "https://docs.rs/cyk_parser"
homepage = "https://github.com/yourusername/rust_cyk_parser"
repository = "https://github.com/yourusername/rust_cyk_parser"
readme = "README.md"
keywords = ["parser", "cyk", "pcfg", "nlp", "parsing"]
categories = ["parsing", "algorithms", "text-processing"]

[dependencies]
# Your dependencies here
```

2. **Choose a License**: Add a LICENSE file. Common choices:
   - MIT License (permissive)
   - Apache-2.0 (permissive with patent protection)
   - MIT OR Apache-2.0 (both, user's choice - recommended for Rust libraries)

```bash
# Add license files (choose one or both)
# For MIT: create LICENSE-MIT
# For Apache-2.0: create LICENSE-APACHE
```

3. **Verify the Package**:

```bash
# Check that everything is in order
cargo package --list

# Do a dry-run publish
cargo publish --dry-run
```

#### Publishing

```bash
# Publish to crates.io
cargo publish
```

#### Version Management

Follow [Semantic Versioning](https://semver.org/):
- `0.1.0` - Initial development release
- `0.1.1` - Backward-compatible bug fixes
- `0.2.0` - New features (backward-compatible)
- `1.0.0` - First stable release

Update versions in `Cargo.toml` for each release:

```bash
# After making changes
cargo build --release
cargo test
# Update version in Cargo.toml
cargo publish
```

### Option 2: GitHub Releases

Host your library on GitHub for visibility and collaboration:

1. **Create a GitHub Repository**:

```bash
# Initialize git (if not already)
git init
git add .
git commit -m "Initial commit: Probabilistic CYK Parser"

# Add remote and push
git remote add origin https://github.com/yourusername/rust_cyk_parser.git
git branch -M main
git push -u origin main
```

2. **Create Tagged Releases**:

```bash
# Tag a release
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

3. **GitHub Release Page**:
   - Go to your repository on GitHub
   - Click "Releases" → "Create a new release"
   - Select your tag
   - Add release notes describing changes
   - Attach build artifacts if needed

### Option 3: Private/Corporate Registry

For private or corporate use:

1. **Cargo Registry**: Set up your own private registry (e.g., using Cloudsmith, Artifactory)
2. **Git Repository**: Use private Git repositories with authentication

```toml
# In dependent projects
[dependencies]
cyk_parser = { git = "https://github.com/yourorg/rust_cyk_parser", branch = "main" }
```

### Documentation Hosting

Rust automatically generates documentation for crates published to crates.io:

- Docs appear at: `https://docs.rs/cyk_parser`
- Build locally with: `cargo doc --open`
- Host custom docs on GitHub Pages:

```bash
# Build docs
cargo doc --no-deps

# Copy to docs folder for GitHub Pages
cp -r target/doc docs

# Enable GitHub Pages in repository settings
```

### Best Practices for Publishing

1. **Comprehensive README**: Include examples, API overview, and getting started guide (✓ already done)
2. **Documentation Comments**: Use `///` for public API documentation
3. **Examples**: Provide runnable examples in `examples/` directory (✓ already done)
4. **Tests**: Include thorough unit and integration tests (✓ already done)
5. **CHANGELOG**: Maintain a CHANGELOG.md file tracking changes between versions
6. **CI/CD**: Set up GitHub Actions for automated testing:

```yaml
# .github/workflows/rust.yml
name: Rust

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
```

### Maintenance and Updates

After publishing, maintain your library:

```bash
# Regular workflow
git pull origin main

# Make changes and test
cargo test
cargo clippy -- -D warnings
cargo fmt

# Update version in Cargo.toml
# Update CHANGELOG.md

# Commit and publish
git commit -am "Release v0.1.1: Fix parsing bug"
git tag v0.1.1
git push origin main --tags
cargo publish
```

## Contributing

Potential improvements:
- [ ] Support for epsilon productions
- [ ] k-best parsing
- [ ] Grammar file format and loader
- [ ] Visualization of parse trees
- [ ] Log probabilities for numerical stability
- [ ] Parallel parsing for long sentences
- [ ] Grammar normalization utilities
- [ ] Serde support for serialization
- [ ] WASM compilation for web use
- [ ] Benchmarking suite

## License

This project is open source and available under your choice of license.

**Recommended**: Use dual MIT/Apache-2.0 licensing, which is standard for Rust projects:

- MIT License: Simple and permissive
- Apache-2.0: Provides additional patent protection

Add both LICENSE-MIT and LICENSE-APACHE files to your repository, and specify in Cargo.toml:

```toml
license = "MIT OR Apache-2.0"
```

## References

- Jurafsky, D., & Martin, J. H. (2023). *Speech and Language Processing* (3rd ed.). Chapter 17: Context-Free Grammars and Constituency Parsing.
- Younger, D. H. (1967). Recognition and parsing of context-free languages in time n³.
- Kasami, T. (1966). An efficient recognition and syntax-analysis algorithm for context-free languages.
