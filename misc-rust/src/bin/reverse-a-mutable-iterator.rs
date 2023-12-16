fn main() {
    println!("Run tests instead");
}

// I just wanted to check that you can indeed reverse a mutable iterator in any order of iterating/reversing.
#[cfg(test)]
mod test {
    fn only_ones(x: &(usize, &usize)) -> bool {
        *x.1 == 1
    }

    #[test]
    fn reverse_after() {
        let list: Vec<usize> = vec![3, 2, 1, 2, 3, 2, 1, 2, 3];
        let mut iter = list.iter().enumerate();
        assert_eq!(iter.find(only_ones), Some((2, &1)));
        let mut iter = iter.rev();
        assert_eq!(iter.find(only_ones), Some((6, &1)));
        assert_eq!(iter.find(only_ones), None);
    }

    #[test]
    fn reverse_before() {
        let list: Vec<usize> = vec![3, 2, 1, 2, 3, 2, 1, 2, 3];
        let mut iter = list.iter().enumerate().rev();
        assert_eq!(iter.find(only_ones), Some((6, &1)));
        assert_eq!(iter.find(only_ones), Some((2, &1)));
        assert_eq!(iter.find(only_ones), None);
    }

    #[test]
    fn reverse_before_and_after() {
        let list: Vec<usize> = vec![3, 2, 1, 2, 3, 2, 1, 2, 3];
        let mut iter = list.iter().enumerate().rev();
        assert_eq!(iter.find(only_ones), Some((6, &1)));
        let mut iter = iter.rev();
        assert_eq!(iter.find(only_ones), Some((2, &1)));
        assert_eq!(iter.find(only_ones), None);
    }
}
