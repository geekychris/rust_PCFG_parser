use cyk_parser::{CYKParser, DependencyConverter, POSTagger, Rule, PCFG};

fn create_english_grammar() -> PCFG {
    let mut grammar = PCFG::new("S");

    // Sentence structure rules
    grammar.add_rule(Rule::new_binary("S", "NP", "VP", 1.0));

    // Noun Phrase rules
    grammar.add_rule(Rule::new_binary("NP", "Det", "N", 0.6));
    grammar.add_rule(Rule::new_binary("NP", "Det", "AN", 0.2));
    grammar.add_rule(Rule::new_terminal("NP", "she", 0.2));

    // Adjective + Noun
    grammar.add_rule(Rule::new_binary("AN", "Adj", "N", 1.0));

    // Verb Phrase rules
    grammar.add_rule(Rule::new_binary("VP", "V", "NP", 0.7));
    grammar.add_rule(Rule::new_terminal("VP", "sleeps", 0.3));

    // Determiners
    grammar.add_rule(Rule::new_terminal("Det", "the", 1.0));

    // Nouns
    grammar.add_rule(Rule::new_terminal("N", "cat", 0.5));
    grammar.add_rule(Rule::new_terminal("N", "mouse", 0.5));

    // Adjectives
    grammar.add_rule(Rule::new_terminal("Adj", "big", 1.0));

    // Verbs
    grammar.add_rule(Rule::new_terminal("V", "chases", 1.0));

    grammar
}

fn main() {
    println!("=== Full Pipeline Demo: POS Tagging + Parsing + Dependencies ===\n");

    // Initialize components
    let tagger = POSTagger::new();
    let grammar = create_english_grammar();
    let parser = CYKParser::new(grammar);
    let dep_converter = DependencyConverter::new();

    // Test sentence
    let sentence = "the big cat chases the mouse";
    let words: Vec<&str> = sentence.split_whitespace().collect();

    println!("Sentence: \"{}\"\n", sentence);

    // Step 1: POS Tagging
    println!("1. Part-of-Speech Tagging:");
    println!("{}", "-".repeat(40));
    let pos_tags = tagger.tag(&words);
    for tag in &pos_tags {
        println!("   {} → {}", tag.word, tag.tag);
    }
    println!();

    // Step 2: Parse the sentence
    println!("2. Constituency Parsing (CYK):");
    println!("{}", "-".repeat(40));
    let result = parser.parse(&words);

    if !result.success {
        println!("   Parse FAILED");
        return;
    }

    println!("   Parse: SUCCESS");
    println!("   Probability: {:.6}", result.probability);
    println!();

    // Display parse tree
    if let Some(ref tree) = result.parse_tree {
        println!("3. Constituency Parse Tree:");
        println!("{}", "-".repeat(40));
        println!("{}", tree);

        // Step 3: Convert to dependency tree
        println!("4. Dependency Tree:");
        println!("{}", "-".repeat(40));
        if let Some(dep_tree) = dep_converter.convert(tree) {
            println!("   Root: {}\n", dep_tree.root);
            println!("   Dependencies:");
            for dep in &dep_tree.dependencies {
                println!("     {} --[{}]--> {}", dep.head, dep.relation, dep.dependent);
            }
        } else {
            println!("   Could not extract dependency tree");
        }
    }
}
