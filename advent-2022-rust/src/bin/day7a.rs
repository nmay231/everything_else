use std::fs::File;
use std::io::{self, BufRead};

use lib::tree::{Tree, TreeBranch};

// TODO: Should I ever have an enum struct variant or should it always be a tuple variant containing a boring struct?
// TODO: Is it okay to mix the two and switch between them when rapidly developing, or is that detrimental?
// struct Directory {
//     name: String,
//     parent: Option<Box<RefCell<Directory>>>,
//     contents: Vec<RefCell<INode>>,
// }

// impl Directory {
//     fn new(name: &str, parent: Option<Box<RefCell<Directory>>>) -> Self {
//         Self {
//             name: name.into(),
//             parent,
//             contents: vec![],
//         }
//     }

//     fn get_parent(&self) -> Option<Box<RefCell<Directory>>> {
//         self.parent
//     }
// }

// enum INode {
//     Directory(Directory),
//     File {
//         name: String,
//         parent: Option<Box<RefCell<Directory>>>,
//         bytes: usize,
//     },
// }

// impl INode {
//     fn new_file(name: &str, parent: Option<Box<RefCell<Directory>>>, bytes: usize) -> Self {
//         Self::File {
//             name: name.into(),
//             parent,
//             bytes,
//         }
//     }

//     fn new_dir(name: &str, parent: Option<Box<RefCell<Directory>>>) -> Self {
//         Self::Directory(Directory::new(name, parent))
//     }

//     pub fn get_parent(self) -> Option<Box<RefCell<Directory>>> {
//         match self {
//             INode::Directory(dir) => dir.parent,
//             INode::File { parent, .. } => parent,
//         }
//     }
//     pub fn walk_depth_first(&self, func: impl FnMut(&INode) -> ()) -> () {
//         self._walk_depth_first(func);
//     }

//     fn _walk_depth_first<F>(&self, mut func: F) -> F
//     where
//         F: FnMut(&INode) -> (),
//     {
//         match self {
//             INode::File { .. } => func(self),
//             INode::Directory(dir) => {
//                 func(self);
//                 for inode in dir.contents.iter() {
//                     // I don't think this is the best way to satisfy the borrow checker, but it works *shrug*
//                     // It's either that or somehow box the function closure on the heap or something
//                     func = inode._walk_depth_first(func);
//                 }
//             }
//         };
//         func
//     }
// }

struct IDirectory {
    name: String,
    bytes: usize,
}

struct IFile {
    name: String,
    bytes: usize,
}

enum MyTree<B, L> {
    Branch(B, Vec<MyTree<B, L>>),
    Leaf(L),
}

fn main() -> Result<(), &'static str> {
    let f = File::open("./assets/day6.txt").or(Err("File missing or unreadable"))?;
    // Skip the first line `cd /`
    let lines = io::BufReader::new(f).lines().skip(1);

    let mut stack: Vec<(IDirectory, Vec<MyTree<IDirectory, IFile>>, usize)> = vec![];

    let mut pwd = IDirectory {
        name: "/".into(),
        bytes: 0,
    };
    let mut ls: Vec<MyTree<IDirectory, IFile>> = vec![];

    for (line_num, line) in lines
        .chain(std::iter::once(Ok("$ hi".to_string())))
        .enumerate()
    {
        let line = line.expect(format!("Error reading file on line {}", line_num).as_str());

        if &line[..4] == "$ cd" {
            let name = &line[5..];
            if name == ".." {
                let (new_pwd, mut parent, index) = stack.pop().expect("Tried to cd .. out of root");
                parent.push(MyTree::Branch(pwd, ls));
                // This unnecessarily preserves order, but I want to do it anyways, so there...
                let last = parent.len() - 1;
                parent.swap(index, last);

                (pwd, ls) = (new_pwd, parent);
                continue;
            }

            let mut index = None;
            for (i, inode) in ls.iter().enumerate() {
                if let MyTree::Branch(dir_info, _) = inode {
                    if dir_info.name != name {
                        continue;
                    }
                    index = Some(i);
                    break;
                }
            }
            if let Some(index) = index {
                match ls.swap_remove(index) {
                    MyTree::Branch(dir_info, dir_contents) => {
                        stack.push((pwd, ls, index));
                        pwd = dir_info;
                        ls = dir_contents;
                    }
                    MyTree::Leaf(_) => {
                        panic!("There's gotta be a better way to filter_map an iterator while getting *owned* values of the results...")
                    }
                }
            }
            panic!("Did not find dir to cd into line={line_num}")
        } else if !line.starts_with("$") {
            let mut split = line.split(" ");
            let error = format!("Parse error line:{line_num}");

            let description = split.next().expect(error.as_str());
            let name = split.next().expect(error.as_str()).to_owned();
            assert_eq!(split.next(), None);

            if description == "dir" {
                ls.push(MyTree::Branch(IDirectory { name, bytes: 0 }, vec![]));
            } else {
                let bytes = description
                    .parse()
                    .expect(format!("Could not parse file size on line {line}").as_str());
                ls.push(MyTree::Leaf(IFile { name, bytes }))
            }
        } else if line == "$ hi" {
            
        }
        assert!(line == "$ ls")
    }

    Ok(())
}
