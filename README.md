# Example

```rust
struct Tree { key: &'static str, childs: Vec<Tree> }

macro_rules! t { 
    ($key:tt, $($childs:expr),*) => { Tree { key: stringify!($key), childs: vec![$($childs),*] } }; 
}

impl log_tree::Compact<&'static str> for Tree {
    fn key(&self, lvl: u8) -> &'static str {
        self.key
    }
    fn childs(&self) -> Option<std::slice::Iter<Self>> {
        Some(self.childs.iter())
    }
}

let tree = t!(Root, t!(C1, t!(C11,), t!(C12,)), t!(C2,));

println!("{}", tree.fmt());
// Root
// ├──C1
// │  ├──C11
// │  └──C12
// └──C2
```