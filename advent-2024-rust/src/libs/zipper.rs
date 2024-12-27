/// A trait to allow any tree-like structure to become mutable. Basically, you
/// will convert the source type into one that implements Zipper, and then use
/// the parent and child methods to traverse up and down the tree. The important
/// distinction is that these are fully owned values, which is only possible
/// (without Rc and the like) because getting a child element pops it from the
/// tree while remembering the steps needed to re-add it in the correct spot.
///
/// A simple example would be a binary tree as the source type while the zipper
/// type keeps track of the current root of the tree alongside a stack of
/// parents "left or right" enums/bools showing where the current node would be
/// reinserted when traversing up the tree.
pub trait Zipper: Sized {
    type Source;
    type Index;

    /// Convert the source type into a zipper view
    fn new(root: Self::Source) -> Self;

    /// Get a mutable reference to the current root of the source tree
    fn source(&mut self) -> &mut Self::Source;

    /// Get the child at index as a zipper, or return the current zipper if index
    /// does not exist
    fn child(self, index: Self::Index) -> Result<Self, Self>;

    /// Get the parent as a zipper, or return the current zipper if `self` is
    /// the root
    fn parent(self) -> Result<Self, Self>;

    /// Traverse up to the parent node and return, see `unzip` for a method that
    /// unwraps the zipper type after traversing to the root node
    fn to_root(mut self) -> Self {
        loop {
            match self.parent() {
                Ok(parent) => self = parent,
                Err(root) => return root,
            }
        }
    }

    /// Extract the current source type consuming `self`. Use `.unzip()` if you
    /// need to get the root of the source type
    fn unwrap_source(self) -> Self::Source;

    /// Convert a zipper view into the source type
    fn unzip(self) -> Self::Source {
        self.to_root().unwrap_source()
    }
}
