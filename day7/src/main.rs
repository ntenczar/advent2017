extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: Option<usize>,
    children: Vec<Node>
}

static PC_RE_STR: &str = r"([a-z]+) \((\d+)\) -> (.*)";
static WEIGHT_RE_STR: &str = r"([a-z]+) \((\d+)\)";

impl Node {
    fn from_string(s: String) -> Node {
        let (name, weight, children_str) = destructure_pc(&s);
        let replaced = children_str.replace(",", "");
        let child_strs = replaced.split_whitespace();

        let children = child_strs.map(|s| {
            return <Node>::from_string_no_meta(String::from(s));
        }).collect();

        return Node {
            name: name,
            weight: Some(weight),
            children: children
        };
    }

    fn from_string_no_meta(s: String) -> Node {
        return Node {
            name: s,
            weight: None,
            children: vec![]
        };
    }

    fn find(&self, s: &String) -> bool {
        if self.name == *s {
            return true;
        }
        return self.children.iter().fold(false, |acc, child| {
            return acc || child.find(s);
        });
    }

    fn insert_weight(tree: Node, name: String, weight: usize) -> Node {
        if tree.name == name {
            return Node {
                name: name,
                weight: Some(weight),
                children: tree.children.clone()
            };
        } else {
            let children = tree.children.iter().map(|c| {
                return <Node>::insert_weight(c.clone(), name.clone(), weight);
            }).collect();
            return Node {
                name: tree.name.clone(),
                weight: tree.weight,
                children: children
            };
        }
    }

    fn insert(tree: Node, n: Node) -> Node {
        if tree.name == n.name {
            return n.clone();
        } else {
            let children = tree.children.iter().map(|c| {
                return <Node>::insert(c.clone(), n.clone());
            }).collect();
            return Node {
                name: tree.name.clone(),
                weight: tree.weight,
                children: children
            };
        }
    }

    fn node_weights(&self) -> Vec<usize> {
        return self.children.iter().map(|c| c.total_weight()).collect();
    }

    fn total_weight(&self) -> usize {
        let self_weight = self.weight.unwrap();
        return self.children.iter().fold(self_weight, |acc, c| {
            return acc + c.total_weight();
        });
    }

    fn balance(tree: Node, desired_val: usize) -> Option<usize> {
        let subweights = tree.node_weights();
        if subweights.len() == 1 && desired_val != subweights[0] {
            return Some(desired_val);
        }
        if subweights.len() == 2 && subweights[0] != subweights[1] {
            return tree.children.iter().fold(None, |acc, c| {
                match <Node>::balance(c.clone(), desired_val) {
                    Some(w) => return Some(w),
                    None => return acc
                }
            });
        }
        let mut balance_val = 0;
        if subweights[0] == subweights[1] {
            balance_val = subweights[0];
        }
        if subweights[1] == subweights[2] {
            balance_val = subweights[1];
        }
        tree.children.iter().fold(None, |acc, c| {

        });
        return None;
    }
}

fn main() {
    let stdin = io::stdin();
    let parent_child_re: Regex = Regex::new(PC_RE_STR).unwrap();

    let mut relationships: Vec<String> = vec![];
    let mut others: Vec<String> = vec![];
    for res in stdin.lock().lines() {
        match res {
            Ok(line) => {
                if parent_child_re.is_match(&line) {
                    relationships.push(line.clone());
                } else {
                    others.push(line.clone());
                }
            },
            Err(_) => {}
        }
    }

    let mut rel_nodes: Vec<Node> = relationships.iter().map(|r| {
        Node::from_string(r.clone())
    }).collect();

    let (root, root_idx) = find_root(rel_nodes.clone()).unwrap();

    rel_nodes.remove(root_idx);

    let mut tree = root;
    println!("Building tree..");
    while rel_nodes.len() > 0 {
        let idx = rel_nodes.clone().iter().enumerate().fold(0, |acc, (i,n)| {
            if tree.find(&n.name) {
                return i;
            }
            return acc;
        });
        let node = rel_nodes[idx].clone();
        tree = <Node>::insert(tree, node);
        rel_nodes.remove(idx);
    }
    println!("Adding weights..");

    let weight_re: Regex = Regex::new(WEIGHT_RE_STR).unwrap();
    for other_line in others.iter() {
        let captures = weight_re.captures(other_line).unwrap();
        let name = String::from(captures.get(1).unwrap().as_str());
        let weight_str = captures.get(2).unwrap().as_str();
        let weight = weight_str.parse::<usize>().unwrap();
        tree = <Node>::insert_weight(tree, name, weight);
    }

    println!("{:?}", <Node>::balance(tree, 62853));
}

fn find_root(nodes: Vec<Node>) -> Option<(Node, usize)> {
    let mut found: Option<(Node, usize)> = None;

    for (i, n) in nodes.iter().enumerate() {
        let mut is_found: bool = false;
        for other in nodes.iter() {
            for c in other.clone().children {
                if c.name == n.name {
                    is_found = true;
                }
            }
        }
        if !is_found {
            found = Some((n.clone(), i));
            break;
        }
    }

    return found;
}

fn destructure_pc(s: &str) -> (String, usize, String) {
    let parent_child_re: Regex =
        Regex::new(PC_RE_STR).unwrap();

    let captures = parent_child_re.captures(s).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let weight_str = captures.get(2).unwrap().as_str();
    let child_str = captures.get(3).unwrap().as_str();
    let weight = weight_str.parse::<usize>().unwrap();
    return (String::from(name), weight, String::from(child_str));
}
