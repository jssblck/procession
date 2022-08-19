use std::collections::VecDeque;

use delegate::delegate;

/// Create a bounded vector.
///
/// Invariant: The bound must be greater than or equal to the number of items.
/// Panics if this invariant is not upheld.
#[macro_export]
macro_rules! boundedvec {
    ($bound:expr; $item:expr) => {{
        let mut bv = BoundedVec::new($bound);
        bv.push($item);
        bv
    }};
    ($bound:expr; $( $item:expr ),*) => {{
        let mut bv = BoundedVec::new($bound);
        $(
            bv.push($item);
        )*
        bv
    }};
}

pub use boundedvec;

/// A `Vec` that will not grow beyond the size set at creation time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedVec<T> {
    bound: usize,
    inner: VecDeque<T>,
}

impl<T> BoundedVec<T> {
    /// Create a new `BoundedVec` instance with the specified bound.
    pub fn new(bound: usize) -> Self {
        assert!(bound >= 1, "Bound must be greater than zero");
        Self {
            bound,
            inner: VecDeque::with_capacity(bound),
        }
    }

    /// Push a new item into `BoundedVec`.
    /// New items are pushed to the front of the inner vec, and as the bound is reached
    /// old items are dropped from the back.
    pub fn push(&mut self, item: T) {
        while self.inner.len() >= self.bound {
            self.inner.pop_front();
        }
        self.inner.push_back(item);
    }
}

impl<T> BoundedVec<T> {
    delegate! {
        to self.inner {
            pub fn iter(&self) -> impl Iterator<Item = &T>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_vec_macro_one() {
        let expected = {
            let mut bv = BoundedVec::new(10);
            bv.push(());
            bv
        };
        let created = boundedvec![10; ()];
        assert_eq!(created, expected);
    }

    #[test]
    fn bounded_vec_macro_many() {
        let expected = {
            let mut bv = BoundedVec::new(10);
            bv.push(());
            bv.push(());
            bv.push(());
            bv
        };
        let created = boundedvec![10; (), (), ()];
        assert_eq!(created, expected);
    }

    #[test]
    fn bounded_vec_macro_many_wrapping() {
        let expected = {
            let mut bv = BoundedVec::new(2);
            bv.push(2);
            bv.push(3);
            bv
        };
        let created = boundedvec![2; 1, 2, 3];
        assert_eq!(created, expected);
    }

    #[test]
    #[should_panic]
    fn bounded_vec_macro_invariant() {
        boundedvec![0; (), (), ()];
    }

    #[test]
    #[should_panic]
    fn bounded_vec_invariant() {
        let _: BoundedVec<()> = BoundedVec::new(0);
    }

    #[test]
    fn bounded_vec_iter() {
        let bv = boundedvec![3; 1, 2, 3, 4];
        let expected = vec![2, 3, 4];
        let iterated = bv.iter().cloned().collect::<Vec<_>>();
        assert_eq!(iterated, expected);
    }
}
