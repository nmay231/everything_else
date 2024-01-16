enum OneOrBoth<A, B> {
    A(A),
    B(B),
    AB(A, B),
}

impl<A, B> OneOrBoth<A, B> {
    fn try_new(a: Option<A>, b: Option<B>) -> Option<Self> {
        match (a, b) {
            (None, None) => None,
            (None, Some(b)) => Some(OneOrBoth::B(b)),
            (Some(a), None) => Some(OneOrBoth::A(a)),
            (Some(a), Some(b)) => Some(OneOrBoth::AB(a, b)),
        }
    }
}

impl<'a, I: 'a> OneOrBoth<I, I> {
    fn prefer_first<T>(&'a self) -> &'a T
    where
        &'a T: From<&'a I>,
    {
        match self {
            OneOrBoth::A(a) => a.into(),
            OneOrBoth::B(b) => b.into(),
            OneOrBoth::AB(a, _) => a.into(),
        }
    }

    fn prefer_second<T>(&'a self) -> &'a T
    where
        &'a T: From<&'a I>,
    {
        match self {
            OneOrBoth::A(a) => a.into(),
            OneOrBoth::B(b) => b.into(),
            OneOrBoth::AB(_, b) => b.into(),
        }
    }
}

// TODO: I bet there's some macro magic that allows you to easily access nested
// versions of this: `OneOrBoth<A, OneOrBoth<B, OneOrBoth<C, ...>>>`
//
// I know that to prefer C, you could do
// `val.prefer_second().prefer_second().prefer_first()`, but that doesn't allow
// setting preference of any of the other options. Eh. That sounds too complex anyways, honestly...
fn main() {}
