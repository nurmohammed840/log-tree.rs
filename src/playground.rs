#![allow(warnings)]
use std::{fmt::Display, fs::write};

type Ptr = Option<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dec {
    Trunk,
    SpaceLeft,
    SpaceRight,
    Space,
}
impl Dec {
    fn to_string(&self) -> String {
        match *self {
            Dec::Trunk => "····│".to_string(),
            Dec::SpaceRight => "····╭─╼".to_string(),
            Dec::SpaceLeft => "····╰─╼".to_string(),
            Dec::Space => "····".to_string(),
        }
    }
}
use Dec::*;
#[derive(Debug, Clone, Copy)]
struct Node<K> {
    value: K,
    left: Ptr,
    right: Ptr,
}
#[derive(Debug)]
struct Tree<K> {
    root: Ptr,
    store: Vec<Node<K>>,
}

impl<K: Copy + Display> Tree<K> {
    pub fn get_node(&self, np: Ptr) -> Option<&Node<K>> {
        self.store.get(np?)
    }
    // Prints the tree with root p.  The idea is to do an in-order traversal
    // (reverse in-order in this case, where right is on top), and print nodes as they
    // are visited, one per line. Each invocation of display() gets its own copy
    // of the display element vector e, which is grown with either whitespace or
    // a trunk element, then modified in its last and possibly second-to-last
    // characters in context.
    fn display(
        &self,
        p: Ptr,
        side: bool,
        mut prefix: Vec<String>,
        out: &mut String,
        d: String,
    ) -> Option<()> {
        let node = self.get_node(p)?;
        prefix.push(d);
        write("rsx.txt", prefix.join(""));
        self.display(node.right, true, prefix.clone(), out, Space.to_string());
        let (last_dec, tail) = match side {
            true => (SpaceRight, Trunk),
            false => (SpaceLeft, Space),
        };
        *prefix.last_mut()? = last_dec.to_string();
        write("rsx.txt", prefix.join(""));
        // Visit node => print accumulated elements. Each node gets a line and each line gets a node.
        out.push_str(&format!("{} {}\n", prefix.join(""), node.value));
        write("r.txt", &out);
        // Overwrite last element before continuing traversal
        *prefix.last_mut()? = tail.to_string();
        write("rsx.txt", prefix.join(""));
        self.display(node.left, false, prefix.clone(), out, Trunk.to_string())
    }

    pub fn display_tree(&self) -> String {
        let mut f = String::new();
        self.display(self.root, false, vec![], &mut f, Trunk.to_string());
        f
    }
}

/// Decodes and prints a previously generated tree.
#[test]
fn maian() {
    let tree: Tree<f32> = Tree {
        root: Some(0),
        store: vec![
            Node {
                value: 0.45,
                left: Some(1),
                right: Some(3),
            },
            Node {
                value: -0.94,
                left: Some(7),
                right: Some(2),
            },
            Node {
                value: 0.15,
                left: Some(8),
                right: None,
            },
            Node {
                value: -0.29,
                left: Some(4),
                right: Some(9),
            },
            Node {
                value: 0.8,
                left: Some(5),
                right: None,
            },
            Node {
                value: -0.85,
                left: Some(6),
                right: None,
            },
            Node {
                value: -0.46,
                left: None,
                right: None,
            },
            Node {
                value: -0.85,
                left: None,
                right: Some(13),
            },
            Node {
                value: -0.42,
                left: None,
                right: Some(10),
            },
            Node {
                value: 0.63,
                left: Some(12),
                right: None,
            },
            Node {
                value: -0.83,
                left: None,
                right: Some(11),
            },
            Node {
                value: 0.75,
                left: None,
                right: None,
            },
            Node {
                value: -0.48,
                left: None,
                right: None,
            },
            Node {
                value: 0.53,
                left: None,
                right: None,
            },
        ],
    };
    println!("{:#?}", tree);
    println!("{}", tree.display_tree());
}
