use std::{cell::RefCell, collections::VecDeque, rc::Weak};

use crate::{file::File, filesystem::FileSystem};

pub struct Directory {
    pub name: String,
    pub size: u128,
    pub child_directories: VecDeque<usize>,
    pub child_files: VecDeque<usize>,
    pub parent: usize,
}

impl Directory {
    pub fn new(name: String, size: u128, parent: usize) -> Directory {
        Directory {
            name: name,
            size: size,
            child_directories: VecDeque::new(),
            child_files: VecDeque::new(),
            parent,
        }
    }
}
