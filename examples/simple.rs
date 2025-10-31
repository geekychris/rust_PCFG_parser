use cyk_parser::{CYKParser, Rule, PCFG};

fn main() {
    // Create a simple grammar
    let mut grammar = PCFG::new("S");

    // Grammar rules:
    // S → NP VP
    grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));

    // NP → Det N | "she"
    grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.6));
    grammar.add_rule(Rule::new_terminal("NP", "she", 0.4));

    // VP → V NP | "sleeps"
    grammar.add_rule(Rule::new_binary("VP", "V", "NP", 0.7));
    grammar.add_rule(Rule::new_terminal("VP", "sleeps", 0.3));

    // Terminals
    grammar.add_rule(Rule::new_terminal("Det", "the", 1.0));
    grammar.add_rule(Rule::new_terminal("N", "cat", 0.5));
    grammar.add_rule(Rule::new_terminal("N", "dog", 0.5));
    grammar.add_rule(Rule::new_terminal("V", "chases", 1.0));

    // Create parser
    let parser = CYKParser::new(grammar);

    // Parse some sentences
    println!("Parsing: \"she sleeps\"");
    let result = parser.parse(&["she", "sleeps"]);
    println!("Success: {}, Probability: {:.6}", result.success, result.probability);
    if !result.success {
        println!("{}", result.diagnostics.error_message());
    }
    println!();

    println!("Parsing: \"the cat chases the dog\"");
    let result = parser.parse(&["the", "cat", "chases", "the", "dog"]);
    println!("Success: {}, Probability: {:.6}", result.success, result.probability);

    if let Some(tree) = result.parse_tree {
        println!("\nParse tree:");
        println!("{}", tree);
    } else {
        println!("\n{}", result.diagnostics.error_message());
    }

    // Try parsing a sentence with unknown words
    println!("\nParsing: \"the quick brown fox jumped over the lazy dogs\"");
    let result = parser.parse(&["the", "quick", "brown", "fox", "jumped", "over", "the", "lazy", "dogs"]);
    println!("Success: {}, Probability: {:.6}", result.success, result.probability);
    if !result.success {
        println!("\n{}", result.diagnostics.error_message());
    }
}
