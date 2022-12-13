mod fs {
    pub enum FileSystemNode {
        File {
            name: String,
            size: usize,
        },
        Dir {
            name: String,
            parent: usize,
            content: Vec<usize>,
        },
    }

    pub struct FileSystem {
        fs: Vec<FileSystemNode>,
    }

    impl FileSystem {
        pub fn new() -> Self {
            let mut fs = Vec::new();

            fs.push(FileSystemNode::Dir {
                name: "/".to_string(),
                parent: 0,
                content: Vec::new(),
            });

            FileSystem { fs }
        }

        pub fn root(&self) -> usize {
            return 0;
        }

        fn add_child(&mut self, parent: usize, child: usize) {
            if let FileSystemNode::Dir { content, .. } = &mut self.fs[parent] {
                content.push(child);
            } else {
                panic!("invalid parent (not a dir)");
            }
        }

        pub fn mkdir(&mut self, parent: usize, name: String) -> usize {
            //println!("make dir: {name}");

            let new_id = self.fs.len();
            self.fs.push(FileSystemNode::Dir {
                name: name,
                parent: parent,
                content: Vec::new(),
            });

            self.add_child(parent, new_id);
            new_id
        }

        pub fn mkfile(&mut self, parent: usize, name: String, size: usize) -> usize {
            //println!("make file: {name}");

            let new_id = self.fs.len();
            self.fs.push(FileSystemNode::File { name, size });

            self.add_child(parent, new_id);
            new_id
        }

        pub fn parent(&self, pwd: usize) -> usize {
            if pwd == self.root() {
                self.root()
            } else {
                if let FileSystemNode::Dir { parent, .. } = &self.fs[pwd] {
                    *parent
                } else {
                    panic!("invalid pwd (not a dir)");
                }
            }
        }

        pub fn find_child(&self, pwd: usize, name: &str) -> usize {
            if let FileSystemNode::Dir { content, .. } = &self.fs[pwd] {
                content
                    .iter()
                    .copied()
                    .find(|child_idx| {
                        let child = &self.fs[*child_idx];
                        let child_name = match child {
                            FileSystemNode::Dir { name, .. }
                            | FileSystemNode::File { name, .. } => name,
                        };
                        child_name == name
                    })
                    .expect("child not found")
            } else {
                panic!("invalid pwd (not a dir)");
            }
        }

        pub fn is_dir(&self, pwd: usize) -> bool {
            matches!(self.fs[pwd], FileSystemNode::Dir { .. })
        }

        pub fn children(&self, pwd: usize) -> &[usize] {
            if let FileSystemNode::Dir { content, .. } = &self.fs[pwd] {
                &content
            } else {
                panic!("invalid pwd (not a dir)");
            }
        }

        pub fn file_size(&self, file: usize) -> usize {
            if let FileSystemNode::File { size, .. } = &self.fs[file] {
                *size
            } else {
                panic!("not a file");
            }
        }
    }
}

use fs::FileSystem;

use std::collections::{HashMap, VecDeque};

fn build_fs() -> FileSystem {
    let mut fs = FileSystem::new();
    let mut pwd = fs.root();

    // Build the tree
    for line in INPUT.lines() {
        if line.starts_with("$ ") {
            let command = &line[2..];

            // Parse command
            if command == "ls" {
                // Nothing to do here, will continue with directory listing
            } else if command.starts_with("cd ") {
                let cd = &command[3..];
                //println!("cd: {cd:?}");

                if cd == "/" {
                    pwd = fs.root();
                } else if cd == ".." {
                    pwd = fs.parent(pwd);
                } else {
                    pwd = fs.find_child(pwd, cd);
                    assert!(fs.is_dir(pwd), "cd arg (\"{cd}\") is not a dir");
                }
            } else {
                panic!("unknown command \"{command}\"");
            }
        } else if line.starts_with("dir ") {
            fs.mkdir(pwd, line[4..].to_string());
        } else {
            // Has to be file entry
            let (file_size, file_name) = line.split_once(' ').expect("file entry has wrong format");
            let file_size: usize = file_size.parse().expect("cannot parse file size");
            fs.mkfile(pwd, file_name.to_string(), file_size);
        }
    }

    fs
}

fn compute_dir_sizes(fs: &FileSystem) -> HashMap<usize, usize> {
    let mut dir_sizes = HashMap::<usize, usize>::new();
    let mut queue = VecDeque::new();

    queue.push_back(fs.root());
    while let Some(top) = queue.pop_front() {
        let mut put_back = false;
        let mut dir_size = 0;
        for child in fs.children(top) {
            if fs.is_dir(*child) {
                if let Some(size) = dir_sizes.get(child) {
                    dir_size += size;
                } else {
                    queue.push_back(*child);
                    put_back = true;
                }
            } else {
                dir_size += fs.file_size(*child);
            }
        }

        if put_back {
            queue.push_back(top);
        } else {
            dir_sizes.insert(top, dir_size);
        }
    }

    dir_sizes
}

fn task1_v2() {
    let mut pwd = Vec::new();
    let mut dir_sizes = HashMap::new();

    for line in INPUT.lines() {
        if line == "$ ls" || line.starts_with("dir ") {
            continue;
        }
        if line.starts_with("$ cd") {
            let cd_arg = line.strip_prefix("$ cd ").unwrap();
            if cd_arg == "/" {
                pwd = vec![String::new()];
            } else if cd_arg == ".." {
                pwd.pop();
            } else {
                pwd.push(cd_arg.to_string());
            }
        } else {
            let (size, _) = line.split_once(" ").unwrap();
            let size = size.parse::<usize>().unwrap();

            (0..(pwd.len() + 1))
                .into_iter()
                .map(|i| pwd[0..i].join("/"))
                .for_each(|path| *(dir_sizes.entry(path).or_insert(0)) += size);
        }
    }

    let total_size = dir_sizes
        .values()
        .copied()
        .filter(|size| *size <= 100000)
        .sum::<usize>();
    println!("sum is: {total_size}");
}

fn task1() {
    let fs = build_fs();
    let dir_sizes = compute_dir_sizes(&fs);

    let mut dir_sizes = dir_sizes.into_iter().collect::<Vec<_>>();
    dir_sizes.sort_unstable_by_key(|(_, size)| *size);

    let mut size_sum = 0;
    for (_dir, size) in dir_sizes {
        if (size) <= 100000 {
            size_sum += size;
        } else {
            break;
        }
    }

    println!("sum is: {size_sum}");
}

fn task2() {
    let fs = build_fs();
    let dir_sizes = compute_dir_sizes(&fs);

    let disk_size: usize = 70000000;
    let occupied_space = dir_sizes[&fs.root()];
    let free_space = disk_size - occupied_space;
    let space_to_free = 30000000 - free_space;

    let mut dir_sizes = dir_sizes.into_iter().collect::<Vec<_>>();
    dir_sizes.sort_unstable_by_key(|(_, size)| *size);

    let (_dir, dir_size_delete) = dir_sizes
        .iter()
        .copied()
        .find(|(_, size)| *size > space_to_free)
        .expect("no large enough dir found");

    println!("delete dir size: {dir_size_delete}");
}

fn main() {
    task1_v2();
}

const INPUT: &'static str = "$ cd /
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
7214296 k";
