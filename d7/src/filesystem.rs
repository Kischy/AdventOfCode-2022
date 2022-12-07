use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    directory::{self, Directory},
    file::File,
};

pub struct FileSystem {
    directories: VecDeque<Directory>,
    files: VecDeque<File>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let mut dirs = VecDeque::new();
        dirs.push_back(Directory::new("/".to_string(), 0, 0));
        FileSystem {
            directories: dirs,
            files: VecDeque::new(),
        }
    }

    pub fn get_top_dir(&self) -> &Directory {
        &self.directories[0]
    }

    pub fn get_dir(&self, index: usize) -> &Directory {
        &self.directories[index]
    }

    pub fn add_size_to_dir(&mut self, dir_index: usize, size: u128) {
        let mut next_index = 0;
        if let Some(dir) = self.directories.get_mut(dir_index) {
            dir.size += size;
            next_index = dir.parent;
            if next_index == dir_index {
                return;
            }
        }
        self.add_size_to_dir(next_index, size);
    }

    pub fn add_file(&mut self, dir_index: usize, file: File) {
        self.add_size_to_dir(dir_index, file.size);
        if let Some(dir) = self.directories.get_mut(dir_index) {
            dir.child_files.push_back(self.files.len());
        }
        self.files.push_back(file);
    }

    pub fn add_directory(&mut self, dir_index: usize, new_dir: Directory) {
        self.add_size_to_dir(dir_index, new_dir.size);
        let len = self.directories.len();
        if let Some(dir) = self.directories.get_mut(dir_index) {
            dir.child_directories.push_back(len);
        }
        self.directories.push_back(new_dir);
    }
}

#[cfg(test)]
mod tests {
    use crate::directory::Directory;
    use crate::file::File;
    use crate::filesystem::FileSystem;

    #[test]
    fn adding_tests() {
        let mut file_system = FileSystem::new();
        let file = File::new("file1".to_string(), 100);
        file_system.add_file(0, file.clone());

        {
            let top_dir = file_system.get_top_dir();
            assert_eq!(top_dir.size, 100);
        }

        file_system.add_directory(0, Directory::new("/abc".to_string(), 0, 0));
        file_system.add_file(1, file.clone());

        {
            let second_dir = file_system.get_dir(1);
            assert_eq!(second_dir.size, 100);
        }
        {
            let top_dir = file_system.get_top_dir();
            assert_eq!(top_dir.size, 200);
        }
    }
}
