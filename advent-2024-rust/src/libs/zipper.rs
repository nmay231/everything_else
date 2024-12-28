/// Implementing this trait allows you to construct a Zipper wrapper type that
/// allows mutable access of a recursive Source type without falling back to Rc
/// or similar types.
pub trait ZipperTrait<'a>: Sized {
    type Index;

    /// Return the wrapper zipper type.
    fn zipper(&'a mut self) -> Zipper<'a, Self> {
        Zipper {
            parents: vec![],
            root: self,
        }
    }

    /// Remove and return the child at the specified index.
    fn pop_child(&mut self, index: &Self::Index) -> Option<Self>;

    /// Insert a child at the specified index.
    fn insert_child(&mut self, index: Self::Index, child: Self);
}

pub struct Zipper<'a, Source: ZipperTrait<'a>> {
    parents: Vec<(Source::Index, Source)>,
    root: &'a mut Source,
}

impl<'a, Source: ZipperTrait<'a>> Zipper<'a, Source> {
    /// Point to the child at this index. The return Result is Err() if the
    /// child at that index doesn't exist.
    pub fn to_child(&mut self, index: Source::Index) -> Result<(), ()> {
        match self.root.pop_child(&index) {
            None => Err(()),
            Some(child) => {
                self.parents
                    .push((index, std::mem::replace(self.root, child)));
                Ok(())
            }
        }
    }

    /// Point to the parent. The return Result is Err() if the current node has
    /// no parent.
    pub fn to_parent(&mut self) -> Result<(), ()> {
        match self.parents.pop() {
            None => Err(()),
            Some((index, mut parent)) => {
                std::mem::swap(self.root, &mut parent);
                self.root.insert_child(index, parent);
                Ok(())
            }
        }
    }

    /// Point to the root of the source type
    pub fn to_root(&mut self) {
        while self.to_parent().is_ok() {}
    }

    /// Get a mutable reference to the current node of the source type.
    pub fn source(&mut self) -> &mut Source {
        self.root
    }
}

impl<'a, Source: ZipperTrait<'a>> Zipper<'a, Source>
where
    <Source as ZipperTrait<'a>>::Index: Clone,
{
    /// Runs self.source().insert_child() followed by self.to_child().
    pub fn insert_child_then_to(&mut self, index: Source::Index, child: Source) {
        self.source().insert_child(Clone::clone(&index), child);
        self.to_child(index)
            .expect("Source type::insert_child() has been incorrectly implemented");
    }
}

#[cfg(test)]
mod test_zipper {
    use std::assert_matches::assert_matches;

    use itertools::Either;

    use super::ZipperTrait;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Binary<T> {
        left: Option<Box<Binary<T>>>,
        right: Option<Box<Binary<T>>>,
        data: T,
    }

    impl<T> Binary<T> {
        fn new(data: T) -> Self {
            Self {
                left: None,
                right: None,
                data,
            }
        }
    }

    impl<'a> ZipperTrait<'a> for Binary<usize> {
        type Index = Either<(), ()>;

        fn pop_child(&mut self, index: &Self::Index) -> Option<Self> {
            match index {
                Either::Left(_) => self.left.take().and_then(|child| Some(*child)),
                Either::Right(_) => self.right.take().and_then(|child| Some(*child)),
            }
        }

        fn insert_child(&mut self, index: Self::Index, child: Self) {
            match index {
                Either::Left(_) => self.left = Some(Box::new(child)),
                Either::Right(_) => self.right = Some(Box::new(child)),
            }
        }
    }

    #[test]
    fn do_nothing() {
        let mut tree = Binary::new(1_usize);
        let mut _zipper = tree.zipper();

        assert_matches!(
            tree,
            Binary {
                data: 1,
                left: None,
                right: None,
            }
        );
    }

    #[test]
    fn shallow_left_right() {
        let mut tree = Binary::new(1_usize);
        let mut zipper = tree.zipper();

        zipper
            .source()
            .insert_child(Either::Left(()), Binary::new(3));
        zipper
            .source()
            .insert_child(Either::Right(()), Binary::new(5));

        assert_matches!(
            tree,
            Binary {
                data: 1,
                left: Some(box Binary {
                    data: 3,
                    left: None,
                    right: None,
                }),
                right: Some(box Binary {
                    data: 5,
                    left: None,
                    right: None,
                }),
            }
        );
    }

    #[test]
    fn left_left_right_right() {
        let mut tree = Binary::new(1_usize);
        let mut zipper = tree.zipper();

        zipper.insert_child_then_to(Either::Left(()), Binary::new(2));
        zipper.insert_child_then_to(Either::Left(()), Binary::new(3));
        zipper.to_root();
        zipper.insert_child_then_to(Either::Right(()), Binary::new(4));
        zipper.insert_child_then_to(Either::Right(()), Binary::new(5));
        zipper.to_root();

        assert_matches!(
            tree,
            Binary {
                data: 1,
                left: Some(box Binary {
                    data: 2,
                    left: Some(box Binary { left: None, right: None, data: 3 }),
                    right: None,
                }),
                right: Some(box Binary {
                    data: 4,
                    left: None,
                    right: Some(box Binary { left: None, right: None, data: 5 }),
                }),
            }
        );
    }
}
