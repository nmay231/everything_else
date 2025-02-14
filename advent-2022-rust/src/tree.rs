use std::rc::Rc;

pub enum Tree<B, L> {
    Branch(Rc<TreeBranch<B, L>>),
    Leaf(Rc<TreeLeaf<L>>),
}

pub struct TreeBranch<B, L> {
    pub data: B,
    children: Vec<Tree<B, L>>,
}

pub struct TreeLeaf<L> {
    pub data: L,
}

impl<L> TreeLeaf<L> {
    pub fn new(data: L) -> Self {
        Self { data }
    }
}

impl<B, L> TreeBranch<B, L> {
    pub fn new(data: B) -> Self {
        Self {
            data,
            children: vec![],
        }
    }

    pub fn add_branch(&mut self, data: B) -> () {
        self.children
            .push(Tree::Branch(Rc::new(TreeBranch::new(data))));
    }

    pub fn add_leaf(&mut self, data: L) -> () {
        self.children.push(Tree::Leaf(Rc::new(TreeLeaf::new(data))));
    }

    // TODO: Too lazy to copy all vector operations one by one
    pub fn children<'a>(&'a mut self) -> &'a mut Vec<Tree<B, L>> {
        &mut self.children
    }

    // TODO: I could make this behind a &reference, but I hate copying lifetimes everywhere
    pub fn depth_first(self) -> TreeBranchDepthFirst<B, L> {
        let tmp = Rc::new(self);
        TreeBranchDepthFirst {
            root: Rc::clone(&tmp),
            previous: vec![],
            branch: tmp,
            index: 0,
        }
    }
}

// Inspiration: https://stackoverflow.com/a/36168919/4373214
pub struct TreeBranchDepthFirst<B, L> {
    pub root: Rc<TreeBranch<B, L>>,
    previous: Vec<(Rc<TreeBranch<B, L>>, usize)>,
    pub branch: Rc<TreeBranch<B, L>>,
    index: usize,
}

impl<B, L> Iterator for TreeBranchDepthFirst<B, L> {
    type Item = Tree<B, L>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index + 1 >= self.branch.children.len() {
            let (branch, index) = self.previous.pop()?;
            (self.branch, self.index) = (branch, index);
        }
        self.index += 1;
        match &self.branch.children[self.index] {
            Tree::Leaf(ref leaf) => Some(Tree::Leaf(Rc::clone(leaf))),
            Tree::Branch(branch) => {
                self.previous.push((Rc::clone(&self.branch), self.index));
                (self.branch, self.index) = (Rc::clone(branch), 0);
                Some(Tree::Branch(Rc::clone(&self.branch)))
            }
        }
    }
}
