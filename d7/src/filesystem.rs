use std::collections::VecDeque;

use regex::Regex;

use crate::{directory::Directory, file::File};

pub struct FileSystem {
    directories: VecDeque<Directory>,
    files: VecDeque<File>,
}

fn get_first_chunk<'a>(line: &'a str, split: &'a str) -> &'a str {
    let chunks: Vec<&str> = line.split(split).collect();
    chunks[1]
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

    pub fn from_string(input: &str) -> FileSystem {
        let mut fs = FileSystem::new();
        let mut index = 0;
        let file = Regex::new(r"([0-9]+)\s(\w.*)").unwrap();
        let cd = "$ cd ";
        let dir_ls = "dir ";

        for line in input.lines() {
            if line.contains(cd) {
                let dir = get_first_chunk(line, cd);
                if dir == "/" {
                    index = 0;
                } else if dir == ".." {
                    index = fs.get_dir(index).parent;
                } else {
                    index = fs.get_child_dir_by_name(index, dir);
                }
            } else if line.contains(dir_ls) {
                let dir = get_first_chunk(line, dir_ls);
                fs.add_directory(index, Directory::new(dir.to_string(), 0, index));
            } else if let Some(captures) = file.captures(line) {
                let size = &captures[1];
                let file = &captures[2];
                fs.add_file(
                    index,
                    File::new(file.to_string(), size.to_string().trim().parse().unwrap()),
                );
            }
        }

        fs
    }

    pub fn get_child_dir_by_name(&self, index: usize, child_name: &str) -> usize {
        for child_i in &self.directories[index].child_directories {
            if self.directories[*child_i].name == child_name.to_string() {
                return *child_i;
            }
        }

        0
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

    pub fn contains_file_or_dir(&mut self, dir_index: usize, name: &String) -> bool {
        let dir_length = self.directories.len();
        let file_length = self.files.len();

        for child in self.directories[dir_index].child_files.clone().into_iter() {
            if child >= file_length {
                continue;
            }
            if self.files[child].name == *name {
                return true;
            }
        }

        for child in self.directories[dir_index]
            .child_directories
            .clone()
            .into_iter()
        {
            if child >= dir_length {
                continue;
            }
            if self.directories[child].name == *name {
                return true;
            }
        }

        false
    }

    pub fn add_file(&mut self, dir_index: usize, file: File) {
        if self.contains_file_or_dir(dir_index, &file.name) {
            return;
        }

        self.add_size_to_dir(dir_index, file.size);
        if let Some(dir) = self.directories.get_mut(dir_index) {
            dir.child_files.push_back(self.files.len());
        }
        self.files.push_back(file);
    }

    pub fn add_directory(&mut self, dir_index: usize, new_dir: Directory) {
        if self.contains_file_or_dir(dir_index, &new_dir.name) {
            return;
        }

        self.add_size_to_dir(dir_index, new_dir.size);
        let len = self.directories.len();
        if let Some(dir) = self.directories.get_mut(dir_index) {
            dir.child_directories.push_back(len);
        }
        self.directories.push_back(new_dir);
    }

    pub fn get_sum_of_sizes(&self, max_size: u128) -> u128 {
        let mut sum = 0;
        for dir in &self.directories {
            if dir.size <= max_size {
                sum += dir.size;
            }
        }
        sum
    }
    pub fn get_size_of_smallest_deletable_dir(&self, total: u128, needed: u128) -> u128 {
        let unused  = total - self.get_top_dir().size;
        let to_free_at_least = needed - unused;
        let mut min = total;        
        for dir in &self.directories {
            if dir.size >= to_free_at_least && dir.size < min {
                min = dir.size;
            }
        }

        min
    }


}

#[cfg(test)]
mod tests {
    use crate::directory::Directory;
    use crate::file::File;
    use crate::filesystem::FileSystem;
    use indoc::indoc;

    #[test]
    fn from_string_tests() {
        let mut fs = FileSystem::from_string(indoc! {"
                        $ cd /
                        $ ls
                        dir a
                        14848514 b.txt
                        8504156 c.dat
                        dir d
                        $ cd a
                        $ ls
                        dir e
                        29116 f
                        2557 g
                        62596 h.lst
                        $ cd e
                        $ ls
                        584 i
                        $ cd ..
                        $ cd ..
                        $ cd d
                        $ ls
                        4060174 j
                        8033020 d.log
                        5626152 d.ext
                        7214296 k
                     "
        });

        {
            let top_dir = fs.get_top_dir();
            assert_eq!(top_dir.size, 48381165);
            assert_eq!(fs.get_sum_of_sizes(100000), 95437);
            assert_eq!(fs.get_size_of_smallest_deletable_dir(70000000,30000000), 24933642);
        }
    }

    #[test]
    fn adding_tests() {
        let mut file_system = FileSystem::new();
        let file = File::new("file1".to_string(), 100);
        let file2 = File::new("file2".to_string(), 100);
        let file3 = File::new("file3".to_string(), 100);
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

        file_system.add_directory(0, Directory::new("/abcc".to_string(), 0, 0));
        file_system.add_file(2, file2.clone());

        {
            let third_dir = file_system.get_dir(2);
            assert_eq!(third_dir.size, 100);
        }
        {
            let top_dir = file_system.get_top_dir();
            assert_eq!(top_dir.size, 300);
        }

        file_system.add_directory(2, Directory::new("/abcc".to_string(), 0, 2));
        file_system.add_file(3, file.clone());
        file_system.add_file(3, file2.clone());
        file_system.add_file(3, file3.clone());

        {
            let third_dir = file_system.get_dir(2);
            assert_eq!(third_dir.size, 400);
        }
        {
            let top_dir = file_system.get_top_dir();
            assert_eq!(top_dir.size, 600);
        }
    }
}
