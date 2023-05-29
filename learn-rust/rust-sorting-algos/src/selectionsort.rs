use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest)
            }
        }
    }
}

#[test]
fn selectionsort_works() {
    let mut things = vec![4, 2, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
