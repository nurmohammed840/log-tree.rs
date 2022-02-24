use log_tree::LogTree;

struct Tree {
    key: String,
    childs: Vec<Tree>,
}
macro_rules! t {
    ($key:tt, $($childs:expr),*) => {
        Tree { key: stringify!($key).to_string(), childs: vec![$($childs),*] }
    };
}

impl LogTree for Tree {
    fn add_node(&self, _: u8) -> Option<(String, log_tree::Childs<Self>)> {
        Some((self.key.clone(), Box::new(&self.childs)))
    }
}

fn main() {
    let tree = t!(
        A1,
        t!(B1, t!(C1,), t!(C2,)),
        t!(B2, t!(C1, t!(D1,), t!(D2,)))
    );
    println!("{}", tree.fmt_tree());
}
