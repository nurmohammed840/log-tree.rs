#![doc = include_str!("../README.md")]

use std::{fmt::Display, slice::Iter};

pub trait Compact<K: Display>: Sized {
    const PREFIX: &'static str = "";

    fn key(&self, lvl: u8) -> K;
    fn childs(&self) -> Option<Iter<Self>>;
    fn decorators(&self, _lvl: u8) -> [&str; 4] {
        ["├──", "│  ", "└──", "   "]
    }

    fn fmt(&self) -> String {
        fn fmt<T: Display>(
            mut prefix: String,
            is_last: bool,
            tree: &impl Compact<T>,
            out: &mut String,
            lvl: u8,
        ) {
            let decorators = tree.decorators(lvl);
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
            out.push_str(&tree.key(lvl).to_string());
            out.push('\n');
            if let Some(childs) = tree.childs() {
                let len = childs.len();
                for (i, child) in childs.enumerate() {
                    fmt(prefix.clone(), i == len - 1, child, out, lvl + 1);
                }
            }
        }
        let mut out = format!("{}\n", self.key(0));
        if let Some(childs) = self.childs() {
            let len = childs.len();
            for (i, child) in childs.enumerate() {
                fmt(Self::PREFIX.to_string(), i == len - 1, child, &mut out, 0);
            }
        }
        out
    }
}

