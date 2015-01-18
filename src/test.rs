#![cfg(test)]

use super::{execute};

#[test]
fn use_it() {
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    execute(&mut [
        &mut || left = 22,
        &mut || right = 44
    ]);
    assert_eq!(left, 22);
    assert_eq!(right, 44);
}

#[cfg(test)]
fn quicksort(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }

    let pivot_value = v[0]; // simplest possible thing...
    let mid = partition(pivot_value, v);
    let (left, right) = v.split_at_mut(mid);
    execute(&mut [
        &mut || quicksort(left),
        &mut || quicksort(right)
    ]);

    fn partition(pivot_value: i32,
                 v: &mut [i32])
                 -> usize
    {
        // Invariant:
        //     .. l ==> less than or equal to pivot
        //     r .. ==> greater than pivot
        let mut l = 0;
        let mut r = v.len() - 1;
        while l <= r {
            if v[l] > pivot_value {
                v.swap(l, r);
                r -= 1;
            } else if v[r] <= pivot_value {
                v.swap(l, r);
                l += 1;
            } else {
                l += 1;
                r -= 1;
            }
        }
        return l;
    }
}

#[test]
fn call_quicksort() {
    let mut v = [55, 12, 86, 8, 3, 5];
    quicksort(v.as_mut_slice());
    let mut bound = 0;
    for &elem in v.iter() {
        assert!(elem >= bound);
        bound = elem;
    }
}

// #[test]
// fn use_it_bad() {
//     let mut left: i32 = 0;
//     let mut right: i32 = 0;
//     execute(&mut [
//         &mut || left = 22,
//         &mut || left = 44  //~ cannot borrow `left` as mutable more than once at a time
//     ]);
//     assert_eq!(left, 22);
//     assert_eq!(right, 44);
// }
