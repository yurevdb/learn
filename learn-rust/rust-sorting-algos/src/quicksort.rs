use super::Sorter;

pub struct QuickSort;

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // Allocating => Not good
        quicksort(slice)
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    // Trivial cases first
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while right != usize::MAX && left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            right = right.wrapping_sub(1);
        } else {
            // move element to the right side
            rest.swap(left, right);
            right = right.wrapping_sub(1);
        }
    }

    // re-align left and right to account for the pivot at 0
    let left = left + 1;

    // place the pivot at its final location
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}

#[test]
fn quicksort_works() {
    let mut things = vec![4, 2, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
