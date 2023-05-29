pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

// Stupid sorts
mod bubblesort;
mod insertionsort;
mod selectionsort;

// Not so stupid sort
mod quicksort;

pub use bubblesort::Bubblesort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl<T> Sorter<T> for StdSorter {
        fn sort(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn standard_sort() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
