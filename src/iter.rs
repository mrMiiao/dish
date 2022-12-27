 pub use core::iter::*;

pub trait Iter: Iterator
where Self: Sized,
{
    /// Minimal and maximal values.
    fn min_max(self) -> Option<(Self::Item, Self::Item)>
    where <Self as Iterator>::Item: Ord,
          Self: Clone,
    {
        match self.clone().min() {
            None => None,
            Some(min) => match self.max() {
                None => None,
                Some(max) => Some((min, max))
            }
        }
    }

    /// Pushes min_max to given function.
    fn min_max_pipe<T, F: Fn(Self::Item, Self::Item) -> T>(self, f: F) -> Option<T>
    where <Self as Iterator>::Item: Ord,
          Self: Clone,
    {
        let (min, max) = self.min_max()?;
        Some(f(min, max))
    }

    /// Collects iterator elements into an array of constant size.
    /// UB if N > _.count().
    unsafe fn collect_array<const N: usize>(self) -> [Self::Item; N] {
        unsafe {
            let mut arr = core::mem::MaybeUninit::zeroed().assume_init();
            self.collect_into_array(&mut arr);
            arr
        }
    }

    /// Collects elements into an array provided by a mutable reference.
    fn collect_into_array<const N: usize>(self, array: &mut [Self::Item; N]) {
        self.enumerate()
            .take_while(|(i, _)| *i < N)
            .for_each(|(i, j)| array[i] = j);
    }
}

impl<T: Iterator> Iter for T {}