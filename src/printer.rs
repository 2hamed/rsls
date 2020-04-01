use std::fs::DirEntry;

pub trait DirEntryPrinter {
    fn print(&self);
}

pub struct Node {
    dir_entry: DirEntry,
}

impl DirEntryPrinter for Node {
    fn print(&self) {}
}

impl Node {
    pub fn new(dir_entry: DirEntry) -> Node {
        Node {
            dir_entry: dir_entry,
        }
    }
}
