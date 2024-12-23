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

    /// Get a reference to the current root of the source tree
    fn source(&self) -> &Self::Source;

    /// Get the child at index as a zipper, or return the current zipper if index
    /// does not exist
    fn child(self, index: Self::Index) -> Result<Self, Self>;

    /// Get the parent as a zipper, or return the current zipper if `self` is
    /// the root
    fn parent(self) -> Result<Self, Self>;

    /// Extract the current source type consuming `self`. Use `.unzip()` if you
    /// need to get the source type of the root.
    fn unwrap_source(self) -> Self::Source;

    /// Convert a zipper view into the source type
    fn unzip(mut self) -> Self::Source {
        loop {
            match self.parent() {
                Ok(parent) => self = parent,
                Err(root) => return root.unwrap_source(),
            }
        }
    }
}
