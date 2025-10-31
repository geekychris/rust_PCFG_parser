use cyk_parser::{CYKParser, Rule, PCFG};

fn create_english_grammar() -> PCFG {
    let mut grammar = PCFG::new("S");

    // Sentence structure rules
    grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));

    // Noun Phrase rules
    grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.5));
    grammar.add_rule(Rule::new_binary("NP", "Det", "AN", 0.3));
    grammar.add_rule(Rule::new_terminal("NP", "she", 0.1));
    grammar.add_rule(Rule::new_terminal("NP", "he", 0.1));

    // Adjective + Noun
    grammar.add_rule(Rule::new_binary("AN", "Adj", "N", 1.0));

    // Verb Phrase rules
    grammar.add_rule(Rule::new_binary("VP", "V", "NP", 0.7));
    grammar.add_rule(Rule::new_binary("VP", "V", "PP", 0.2));
    grammar.add_rule(Rule::new_terminal("VP", "sleeps", 0.1));

    // Prepositional Phrase
    grammar.add_rule(Rule::new_binary("PP", "P", "NP", 1.0));

    // Determiners
    grammar.add_rule(Rule::new_terminal("Det", "the", 0.6));
    grammar.add_rule(Rule::new_terminal("Det", "a", 0.4));

    // Nouns
    grammar.add_rule(Rule::new_terminal("N", "cat", 0.3));
    grammar.add_rule(Rule::new_terminal("N", "dog", 0.3));
    grammar.add_rule(Rule::new_terminal("N", "mouse", 0.2));
    grammar.add_rule(Rule::new_terminal("N", "cheese", 0.2));

    // Adjectives
    grammar.add_rule(Rule::new_terminal("Adj", "big", 0.4));
    grammar.add_rule(Rule::new_terminal("Adj", "small", 0.3));
    grammar.add_rule(Rule::new_terminal("Adj", "lazy", 0.3));

    // Verbs
    grammar.add_rule(Rule::new_terminal("V", "chases", 0.4));
    grammar.add_rule(Rule::new_terminal("V", "sees", 0.3));
    grammar.add_rule(Rule::new_terminal("V", "eats", 0.3));

    // Prepositions
    grammar.add_rule(Rule::new_terminal("P", "with", 0.5));
    grammar.add_rule(Rule::new_terminal("P", "in", 0.5));

    grammar
}

fn main() {
    println!("=== Probabilistic CYK Parser Demo ===\n");

    let grammar = create_english_grammar();
    let parser = CYKParser::new(grammar);

    // Test sentences
    let test_sentences = vec![
        vec!["the", "cat", "chases", "the", "mouse"],
        vec!["she", "sees", "the", "big", "dog"],
        vec!["a", "dog", "eats", "the", "cheese"],
        vec!["he", "sleeps"],
        vec!["the", "big", "cat", "chases", "a", "small", "mouse"],
    ];

    for (i, sentence) in test_sentences.iter().enumerate() {
        println!("Sentence {}: \"{}\"", i + 1, sentence.join(" "));
        println!("{}", "-".repeat(60));

        let result = parser.parse(sentence);

        if result.success {
            println!("Parse: SUCCESS");
            println!("Probability: {:.6}", result.probability);
            println!("\nParse Tree:");
            if let Some(tree) = result.parse_tree {
                print!("{}", tree);
            }
        } else {
            println!("Parse: FAILED");
            println!("The sentence does not conform to the grammar.");
        }

        println!("\n");
    }

    // Test a sentence that should fail
    println!("Testing invalid sentence:");
    println!("Sentence: \"cat dog mouse cheese\"");
    println!("{}", "-".repeat(60));

    let invalid = vec!["cat", "dog", "mouse", "cheese"];
    let result = parser.parse(&invalid);

    if result.success {
        println!("Parse: SUCCESS (unexpected!)");
        println!("Probability: {:.6}", result.probability);
    } else {
        println!("Parse: FAILED (as expected)");
        println!("The sentence does not conform to the grammar.");
    }
}
