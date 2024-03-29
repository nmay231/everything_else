use std::fs::File;
use std::io::{self, BufRead};

// TODO: Should I ever have an enum struct variant or should it always be a tuple variant containing a boring struct?
// TODO: Is it okay to mix the two and switch between them when rapidly developing, or is that detrimental?
struct Directory<'a> {
    name: String,
    parent: Option<&'a mut Directory<'a>>,
    contents: Vec<INode<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &str, parent: Option<&'a mut Self>) -> Self {
        Self {
            name: name.into(),
            parent,
            contents: vec![],
        }
    }

    fn get_parent(&'a mut self) -> Option<&'a mut Self> {
        self.parent
    }

    // fn get_rc(&self) -> Rc<Self> {
    //     match self.get_parent() {
    //         None => Rc::new(self),
    //         Some(parent) => {
    //             parent
    //                 .contents
    //                 .iter()
    //                 .filter_map(|child| match child.as_ref() {
    //                     INode::Directory(dir) => Some(dir),
    //                     INode::File { .. } => None,
    //                 })
    //                 .find(|dir| dir.name == name);
    //         }
    //     }
    // }
}

enum INode<'a> {
    Directory(Directory<'a>),
    File {
        name: String,
        parent: Option<Directory<'a>>,
        bytes: usize,
    },
}

impl<'a> INode<'a> {
    fn new_file(name: &str, parent: Option<Directory<'a>>, bytes: usize) -> Self {
        Self::File {
            name: name.into(),
            parent,
            bytes,
        }
    }

    fn new_dir(name: &str, parent: Option<&'a Directory>) -> Self {
        Self::Directory(Directory::new(name, parent))
    }

    pub fn get_parent(&'a self) -> Option<&'a Directory<'a>> {
        match self {
            INode::Directory(dir) => dir.parent,
            INode::File { parent, .. } => parent.as_ref(),
        }
    }
    pub fn walk_depth_first(&self, func: impl FnMut(&INode) -> ()) -> () {
        self._walk_depth_first(func);
    }

    fn _walk_depth_first<F>(&self, mut func: F) -> F
    where
        F: FnMut(&INode) -> (),
    {
        match self {
            INode::File { .. } => func(self),
            INode::Directory(dir) => {
                func(self);
                for inode in dir.contents.iter() {
                    // I don't think this is the best way to satisfy the borrow checker, but it works *shrug*
                    // It's either that or somehow box the function closure on the heap or something
                    func = inode._walk_depth_first(func);
                }
            }
        };
        func
    }
}

enum ParseMode {
    Command,
    LS,
}

fn main() -> Result<(), &'static str> {
    let f = File::open("./assets/day6.txt").or(Err("File missing or unreadable"))?;
    let lines = io::BufReader::new(f).lines();

    let root = Directory::new("/", None);
    let mut current_dir = &mut root;
    let mut mode = ParseMode::Command;

    for (line_num, line) in lines.enumerate() {
        let line = line.expect(format!("Error reading file on line {}", line_num).as_str());
        match (&mode, &line[..4]) {
            (ParseMode::Command, "$ ls") => {
                if current_dir.contents.len() > 0 {
                    // Detected duplicates would be annoying...
                    panic!("Listed a directory more than once!");
                }
                mode = ParseMode::LS;
            }
            (ParseMode::Command, "$ cd") => {
                let name = line[5..].trim();
                if name == ".." {
                    current_dir = current_dir.get_parent().ok_or("Tried cd-ing out of root")?;
                } else {
                    let target = current_dir
                        .contents
                        .iter()
                        .filter_map(|child| match child {
                            INode::Directory(dir) => Some(dir),
                            INode::File { .. } => None,
                        })
                        .find(|dir| dir.name == name);

                    let target = target.expect("Unable to find target directory");
                    current_dir = target;
                }
            }
            (ParseMode::LS, _) if !line.starts_with("$") => {
                let mut split = line.split(" ");
                let error = format!("Parse error line:{line_num}");
                let desc = split.nth(0).expect(error.as_str());
                let name = split.nth(0).expect(error.as_str());
                if desc == "dir" {
                    current_dir
                        .contents
                        .push(INode::new_dir(name, Some(current_dir)));
                    // INode::Directory(Directory::new(
                    //     name,
                    //     Some(Rc::new(current_dir)),
                    // ))
                }
            }
            (_, _) => {
                panic!("Unhandled line {line_num}: {line}")
            }
        }
    }
    Ok(())
}
