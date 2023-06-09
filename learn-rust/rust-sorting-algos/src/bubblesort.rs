use super::Sorter;

pub struct Bubblesort;

impl<T> Sorter<T> for Bubblesort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut things = vec![4, 2, 3, 1];
    Bubblesort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
