use cyk_parser::{CYKParser, POSTagger, Rule, PCFG};

fn create_grammar_with_expanded_vocabulary() -> (PCFG, POSTagger) {
    // Create expanded tagger with more vocabulary
    let tagger = POSTagger::new_expanded();

    let mut grammar = PCFG::new("S");

    // Core sentence structure
    grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));

    // Noun Phrase rules
    grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.4));
    grammar.add_rule(Rule::new_binary("NP", "Det", "AdjP", 0.3));
    grammar.add_rule(Rule::new_terminal("NP", "she", 0.1));
    grammar.add_rule(Rule::new_terminal("NP", "he", 0.1));
    grammar.add_rule(Rule::new_terminal("NP", "i", 0.1));

    // Adjective Phrase (for multiple adjectives + noun)
    grammar.add_rule(Rule::new_binary("AdjP", "Adj", "N", 0.5));
    grammar.add_rule(Rule::new_binary("AdjP", "Adj", "AdjP", 0.5));

    // Verb Phrase rules
    grammar.add_rule(Rule::new_binary("VP", "V", "NP", 0.4));   // Transitive: "chases the dog"
    grammar.add_rule(Rule::new_binary("VP", "V", "PP", 0.2));    // Verb + prep phrase
    grammar.add_rule(Rule::new_binary("VP", "VT", "PP", 0.1));   // Transitive + prep phrase

    // Intransitive verbs (no object needed)
    grammar.add_rule(Rule::new_terminal("VP", "sleeps", 0.1));
    grammar.add_rule(Rule::new_terminal("VP", "runs", 0.1));
    grammar.add_rule(Rule::new_terminal("VP", "walks", 0.1));

    // Transitive VP (verb with object)
    grammar.add_rule(Rule::new_binary("VT", "V", "NP", 1.0));

    // Prepositional Phrase
    grammar.add_rule(Rule::new_binary("PP", "P", "NP", 1.0));

    // Automatically add all terminal rules from the tagger's lexicon
    grammar.add_lexicon_from_tagger(&tagger);

    (grammar, tagger)
}

fn main() {
    println!("=== Expanded Vocabulary Demo ===\n");

    let (grammar, tagger) = create_grammar_with_expanded_vocabulary();
    let parser = CYKParser::new(grammar);

    // Test sentences
    let test_sentences = vec![
        "the quick brown fox jumped over the lazy dogs",
        "the cat chases the dog",
        "she sleeps",
        "the big cat runs",
    ];

    for sentence in test_sentences {
        println!("Parsing: \"{}\"", sentence);
        println!("{}", "=".repeat(60));

        let words: Vec<&str> = sentence.split_whitespace().collect();

        // Show POS tags
        println!("\nPOS Tags:");
        let pos_tags = tagger.tag(&words);
        for (i, tag) in pos_tags.iter().enumerate() {
            println!("  {}: {} -> {}", i, tag.word, tag.tag);
        }

        // Parse
        println!("\nParsing...");
        let result = parser.parse(&words);

        if result.success {
            println!("✓ SUCCESS (probability: {:.6})", result.probability);

            if let Some(tree) = result.parse_tree {
                println!("\nParse Tree:");
                println!("{}", tree);
            }
        } else {
            println!("✗ FAILED");
            println!("\n{}", result.diagnostics.error_message());
        }

        println!("\n");
    }

    // Demonstrate file loading capability
    println!("\n{}", "=".repeat(60));
    println!("NOTE: You can expand vocabulary by loading from a file:");
    println!("  let mut tagger = POSTagger::new_expanded();");
    println!("  tagger.load_from_file(\"vocabulary.txt\").unwrap();");
    println!("\nFile format (one word per line):");
    println!("  jumped V 1.0");
    println!("  running V 1.0");
    println!("  beautiful Adj 1.0");
    println!("\n{}", "=".repeat(60));
}
