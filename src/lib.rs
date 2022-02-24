#![doc = include_str!("../README.md")]

pub type Childs<'a, T> = Box<dyn AsRef<[T]> + 'a>;


pub trait LogTree: Sized {
    /// Add a node to the tree.
    /// 
    /// - Returns `None` if the node should not be added.
    /// - Returns `Some((key, childs))` if the node should be added.
    /// - where `childs` can be empty, if the node is a leaf node.
    /// 
    /// The `lvl` parameter is the depth of the node in the tree.
    fn add_node(&self, lvl: u8) -> Option<(String, Childs<Self>)>;

    ///  Decorators is an array of 4 strings, used to decorate the tree.
    fn decorators(&self, lvl: u8) -> [&str; 4] {
        match lvl {
            0 => ["├─╼ ", "│  ", "└─╼ ", "   "],
            _ => ["├─> ", "│  ", "╰─> ", "   "],
        }
    }

    /// format the tree, and return the formatted string.
    fn fmt_tree(&self) -> String {
        let mut out = String::new();
        if let Some((key, childs)) = self.add_node(0) {
            out.push_str(&key);
            let childs = childs.as_ref().as_ref();
            let len = childs.len();
            for (i, child) in childs.iter().enumerate() {
                fmt_tree(String::new(), i == len - 1, child, &mut out, 0);
            }
        }
        out
    }

    /// Same as fmt_tree, but return the formatted string as child of tree.
    fn fmt_tree_node(&self, is_last: bool) -> String {
        let mut out = String::new();
        fmt_tree(String::new(), is_last, self, &mut out, 0);
        out
    }
}

fn fmt_tree(
    mut prefix: String,
    is_last: bool,
    tree: &impl LogTree,
    out: &mut String,
    lvl: u8,
) {
    let decorators = tree.decorators(lvl);
    out.push('\n');
    out.push_str(&prefix);
    prefix.push_str(match is_last {
        false => {
            out.push_str(decorators[0]);
            decorators[1]
        }
        true => {
            out.push_str(decorators[2]);
            decorators[3]
        }
    });
    if let Some((key, childs)) = tree.add_node(lvl) {
        out.push_str(&key);
        let childs = childs.as_ref().as_ref();
        let len = childs.len();
        for (i, child) in childs.iter().enumerate() {
            fmt_tree(prefix.clone(), i == len - 1, child, out, lvl + 1);
        }
    }
}
