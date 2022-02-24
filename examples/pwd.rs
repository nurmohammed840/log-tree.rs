use log_tree::LogTree;
use std::{env, fs::read_dir, path::Path};

struct DirTree {
    path: String,
    is_dir: bool,
}

impl LogTree for DirTree {
    fn add_node(&self, lvl: u8) -> Option<(String, log_tree::Childs<Self>)> {
        let key = Path::new(&self.path)
            .file_name()?
            .to_string_lossy()
            .into_owned();

        let mut childs = Vec::new();

        if let Some(Ok(dir)) = (lvl < 2).then(|| read_dir(&self.path)) {
            childs = dir
                .filter_map(|entry| entry.ok())
                .map(|entry| DirTree {
                    path: entry.path().to_str().unwrap().to_string(),
                    is_dir: entry.file_type().unwrap().is_dir(),
                })
                .collect();
        }
        Some((key, Box::new(childs)))
    }

    fn decorators(&self, _lvl: u8) -> [&str; 4] {
        match self.is_dir {
            true => ["├─╼ ", "│  ", "└─╼ ", "   "],
            false => ["├─> ", "│  ", "╰─> ", "   "],
        }
    }
}

fn main() {
    let path = env::current_dir().unwrap().to_string_lossy().to_string();
    let tree = DirTree { path, is_dir: true };
    println!("{}", tree.fmt_tree_node(true));
}
