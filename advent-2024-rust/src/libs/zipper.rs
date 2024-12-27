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
pub trait Zipper<'creation>: Sized {
    type Source;
    type Index;

    /// Convert the source type into a zipper view
    fn new(root: &'creation mut Self::Source) -> Self;

    /// Get a mutable reference to the current root of the source tree
    fn source(&mut self) -> &mut Self::Source;

    /// Get the child at index as a zipper, or return the current zipper if index
    /// does not exist
    fn child(&mut self, index: Self::Index) -> Result<(), ()>;

    /// Get the parent as a zipper, or return the current zipper if `self` is
    /// the root
    fn parent(&mut self) -> Result<(), ()>;

    /// Traverse up to the parent node and return, see `unzip` for a method that
    /// unwraps the zipper type after traversing to the root node
    fn to_root(&mut self) {
        while self.parent().is_ok() {}
    }
}
