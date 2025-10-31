use cyk_parser::{CYKParser, DependencyConverter, DependencyTree, POSTagger, Rule, PCFG};
use std::io::{self, Write};

fn create_english_grammar(tagger: &POSTagger) -> PCFG {
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

    // Automatically add all terminal rules from the tagger's expanded lexicon
    grammar.add_lexicon_from_tagger(tagger);

    grammar
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║         Interactive CYK Parser with Dependency Trees         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("This parser demonstrates:");
    println!("  1. Part-of-Speech (POS) tagging");
    println!("  2. Constituency parsing using CYK algorithm");
    println!("  3. Dependency tree extraction\n");

    println!("Example sentences you can try:");
    println!("  - the cat chases the mouse");
    println!("  - she sees the big dog");
    println!("  - the quick brown fox jumped over the lazy dogs");
    println!("  - he sleeps");
    println!("  - the big cat chases a small mouse\n");

    println!("Type 'quit' or 'exit' to quit.\n");

    // Initialize components with expanded vocabulary
    let tagger = POSTagger::new_expanded();
    let grammar = create_english_grammar(&tagger);
    let parser = CYKParser::new(grammar);
    let dep_converter = DependencyConverter::new();

    loop {
        print!("Enter sentence: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                // Check for quit command
                if input.is_empty() {
                    continue;
                }
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    println!("\nGoodbye!");
                    break;
                }

                // Process the sentence
                println!("\n{}", "─".repeat(70));
                process_sentence(input, &tagger, &parser, &dep_converter);
                println!("{}\n", "─".repeat(70));
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn process_sentence(
    sentence: &str,
    tagger: &POSTagger,
    parser: &CYKParser,
    dep_converter: &DependencyConverter,
) {
    println!("Input: \"{}\"", sentence);
    println!();

    // Split into words
    let words: Vec<&str> = sentence.split_whitespace().collect();

    if words.is_empty() {
        println!("Error: Empty sentence");
        return;
    }

    // Step 1: POS Tagging
    println!("Step 1: Part-of-Speech Tagging");
    println!("{}", "-".repeat(40));
    let pos_tags = tagger.tag(&words);

    for tag in &pos_tags {
        println!("  {} → {}", tag.word, tag.tag);
    }
    println!();

    // Step 2: Parsing
    println!("Step 2: Constituency Parsing (CYK Algorithm)");
    println!("{}", "-".repeat(40));
    let result = parser.parse(&words);

    if !result.success {
        println!("  Parse FAILED");
        println!("  The sentence does not conform to the grammar.");
        println!("\n  Possible issues:");
        println!("  - Unknown words not in the lexicon");
        println!("  - Invalid sentence structure");
        println!("  - Grammar does not cover this construction");
        return;
    }

    println!("  Parse: SUCCESS");
    println!("  Probability: {:.6}", result.probability);
    println!();

    // Display parse tree
    if let Some(ref tree) = result.parse_tree {
        println!("  Constituency Parse Tree:");
        println!("  {}", "-".repeat(38));
        for line in format!("{}", tree).lines() {
            println!("  {}", line);
        }
        println!();

        // Step 3: Dependency Tree
        println!("Step 3: Dependency Tree Extraction");
        println!("{}", "-".repeat(40));

        match dep_converter.convert(tree) {
            Some(dep_tree) => {
                println!("  Root: {}", dep_tree.root);
                println!();
                println!("  Dependencies:");

                if dep_tree.dependencies.is_empty() {
                    println!("    (none - single word sentence)");
                } else {
                    for dep in &dep_tree.dependencies {
                        println!("    {} --[{}]--> {}", dep.head, dep.relation, dep.dependent);
                    }
                }
                println!();

                // Visual representation
                println!("  Dependency Visualization:");
                println!("  {}", "-".repeat(38));
                visualize_dependencies(&dep_tree);
            }
            None => {
                println!("  Error: Could not extract dependency tree");
            }
        }
    }
}

fn visualize_dependencies(dep_tree: &DependencyTree) {
    use std::collections::HashMap;

    // Build a map of head -> [(relation, dependent)]
    let mut head_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for dep in &dep_tree.dependencies {
        head_map
            .entry(dep.head.clone())
            .or_insert_with(Vec::new)
            .push((dep.relation.clone(), dep.dependent.clone()));
    }

    // Print tree starting from root
    print_dep_node(&dep_tree.root, &head_map, 0);
}

fn print_dep_node(
    word: &str,
    head_map: &std::collections::HashMap<String, Vec<(String, String)>>,
    indent: usize,
) {
    let spaces = "  ".repeat(indent);
    println!("  {}{}", spaces, word);

    if let Some(dependents) = head_map.get(word) {
        for (relation, dependent) in dependents {
            let spaces = "  ".repeat(indent + 1);
            println!("  {}└─[{}]─ {}", spaces, relation, dependent);

            // Recursively print dependents of this dependent
            print_dep_node(dependent, head_map, indent + 2);
        }
    }
}
