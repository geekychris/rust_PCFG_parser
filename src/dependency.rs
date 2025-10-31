use crate::ParseTree;
use std::fmt;

/// Represents a dependency relation between words
#[derive(Debug, Clone)]
pub struct Dependency {
    /// The head word
    pub head: String,
    /// The dependent word
    pub dependent: String,
    /// The relation type (e.g., "nsubj", "dobj", "det")
    pub relation: String,
}

impl Dependency {
    pub fn new(head: &str, dependent: &str, relation: &str) -> Self {
        Dependency {
            head: head.to_string(),
            dependent: dependent.to_string(),
            relation: relation.to_string(),
        }
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --{}-> {}", self.head, self.relation, self.dependent)
    }
}

/// Represents a dependency tree
#[derive(Debug, Clone)]
pub struct DependencyTree {
    /// The root word (usually the main verb)
    pub root: String,
    /// List of all dependencies
    pub dependencies: Vec<Dependency>,
}

impl DependencyTree {
    pub fn new(root: String) -> Self {
        DependencyTree {
            root,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.push(dep);
    }
}

impl fmt::Display for DependencyTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Root: {}", self.root)?;
        writeln!(f, "\nDependencies:")?;
        for dep in &self.dependencies {
            writeln!(f, "  {}", dep)?;
        }
        Ok(())
    }
}

/// Converts a constituency parse tree to a dependency tree
pub struct DependencyConverter {
    /// Head rules for finding the head of each phrase
    head_rules: HeadRules,
}

impl DependencyConverter {
    pub fn new() -> Self {
        DependencyConverter {
            head_rules: HeadRules::default(),
        }
    }

    /// Convert a parse tree to a dependency tree
    pub fn convert(&self, parse_tree: &ParseTree) -> Option<DependencyTree> {
        // Find the root (head of the sentence)
        let root_word = self.find_head_word(parse_tree)?;
        let mut dep_tree = DependencyTree::new(root_word.clone());

        // Extract dependencies
        self.extract_dependencies(parse_tree, &root_word, &mut dep_tree);

        Some(dep_tree)
    }

    /// Find the head word of a parse tree
    fn find_head_word(&self, tree: &ParseTree) -> Option<String> {
        match tree {
            ParseTree::Terminal { word, .. } => Some(word.clone()),
            ParseTree::NonTerminal { symbol, children, .. } => {
                let head_idx = self.head_rules.get_head_index(symbol, children);
                if let Some(head_child) = children.get(head_idx) {
                    self.find_head_word(head_child)
                } else {
                    None
                }
            }
        }
    }

    /// Extract dependencies from a parse tree
    fn extract_dependencies(
        &self,
        tree: &ParseTree,
        parent_head: &str,
        dep_tree: &mut DependencyTree,
    ) {
        match tree {
            ParseTree::Terminal { .. } => {
                // Terminals don't have children
            }
            ParseTree::NonTerminal { symbol, children, .. } => {
                if children.is_empty() {
                    return;
                }

                // Find which child is the head
                let head_idx = self.head_rules.get_head_index(symbol, children);
                let head_word = self.find_head_word(&children[head_idx]);

                if let Some(head) = head_word {
                    // Process each non-head child
                    for (i, child) in children.iter().enumerate() {
                        if i != head_idx {
                            let dependent_word = self.find_head_word(child);
                            if let Some(dep) = dependent_word {
                                let relation = self.get_relation_type(symbol, &child, i, head_idx);
                                dep_tree.add_dependency(Dependency::new(&head, &dep, &relation));
                            }
                        }
                        // Recursively process children
                        self.extract_dependencies(child, &head, dep_tree);
                    }
                } else {
                    // If we can't find a head, just process children
                    for child in children {
                        self.extract_dependencies(child, parent_head, dep_tree);
                    }
                }
            }
        }
    }

    /// Determine the relation type based on the phrase type and position
    fn get_relation_type(&self, phrase_type: &str, child: &ParseTree, child_idx: usize, head_idx: usize) -> String {
        let child_symbol = match child {
            ParseTree::Terminal { symbol, .. } => symbol.as_str(),
            ParseTree::NonTerminal { symbol, .. } => symbol.as_str(),
        };

        match phrase_type {
            "S" => {
                if child_symbol == "NP" {
                    "nsubj".to_string() // Nominal subject
                } else if child_symbol == "VP" {
                    "root".to_string()
                } else {
                    "dep".to_string()
                }
            }
            "VP" => {
                if child_symbol == "V" {
                    "root".to_string()
                } else if child_symbol == "NP" {
                    "dobj".to_string() // Direct object
                } else if child_symbol == "PP" {
                    "prep".to_string() // Prepositional modifier
                } else {
                    "dep".to_string()
                }
            }
            "NP" => {
                if child_symbol == "Det" {
                    "det".to_string() // Determiner
                } else if child_symbol == "Adj" || child_symbol == "AN" {
                    "amod".to_string() // Adjectival modifier
                } else if child_symbol == "N" {
                    if child_idx < head_idx {
                        "compound".to_string() // Compound noun
                    } else {
                        "dep".to_string()
                    }
                } else {
                    "dep".to_string()
                }
            }
            "PP" => {
                if child_symbol == "P" {
                    "case".to_string() // Case marking
                } else if child_symbol == "NP" {
                    "pobj".to_string() // Object of preposition
                } else {
                    "dep".to_string()
                }
            }
            "AN" => {
                if child_symbol == "Adj" {
                    "amod".to_string()
                } else {
                    "dep".to_string()
                }
            }
            _ => "dep".to_string(), // Generic dependency
        }
    }
}

impl Default for DependencyConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Head rules for determining which child is the head of a phrase
struct HeadRules;

impl HeadRules {
    /// Get the index of the head child in a phrase
    fn get_head_index(&self, phrase_type: &str, children: &[ParseTree]) -> usize {
        match phrase_type {
            "S" => {
                // For sentences, the VP is the head (right child)
                self.find_child_by_symbol(children, "VP").unwrap_or(children.len() - 1)
            }
            "VP" => {
                // For VP, the verb is the head
                self.find_child_by_symbol(children, "V").unwrap_or(0)
            }
            "NP" => {
                // For NP, the noun is the head (usually rightmost)
                self.find_child_by_symbol(children, "N")
                    .or_else(|| self.find_child_by_symbol(children, "AN"))
                    .unwrap_or(children.len() - 1)
            }
            "PP" => {
                // For PP, the NP is the head (right child)
                self.find_child_by_symbol(children, "NP").unwrap_or(children.len() - 1)
            }
            "AN" => {
                // For Adj+Noun, the noun is the head (right child)
                children.len() - 1
            }
            _ => {
                // Default: rightmost child is head
                children.len().saturating_sub(1)
            }
        }
    }

    /// Find the index of a child with a specific symbol
    fn find_child_by_symbol(&self, children: &[ParseTree], symbol: &str) -> Option<usize> {
        children.iter().position(|child| match child {
            ParseTree::Terminal { symbol: s, .. } => s == symbol,
            ParseTree::NonTerminal { symbol: s, .. } => s == symbol,
        })
    }
}

impl Default for HeadRules {
    fn default() -> Self {
        HeadRules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_creation() {
        let dep = Dependency::new("eats", "cat", "nsubj");
        assert_eq!(dep.head, "eats");
        assert_eq!(dep.dependent, "cat");
        assert_eq!(dep.relation, "nsubj");
    }

    #[test]
    fn test_dependency_tree_creation() {
        let mut tree = DependencyTree::new("eats".to_string());
        tree.add_dependency(Dependency::new("eats", "cat", "nsubj"));
        tree.add_dependency(Dependency::new("eats", "mouse", "dobj"));

        assert_eq!(tree.root, "eats");
        assert_eq!(tree.dependencies.len(), 2);
    }
}
