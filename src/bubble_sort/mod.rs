use std::default::Default;

//!This trait implements Bubble Sort for all the types it can
//!Bubble Sort is a very simple sorting algorithm, which is genrally taught in intro to DSA to
//!introduce algorithms, as such it's a very slow algorithm, with O(N²) time complexity.
//!This trait is for educational purposes only and is not meant to be used for production, which is
//!why bubble_sort method issues a warning
pub trait BubbleSort {
    fn bubble_sort(&mut self) {}
}

impl<T: PartialOrd + PartialEq + Default> BubbleSort for [T] {
    fn bubble_sort(&mut self) {
        warn!("Bubble sort is a slow sorting algorithm, it should not be used over tim sort which is the standard sorting algorithm");
        let len = self.len();

        for i in 0..(len - 1) {
            for j in 0..(len - i - 1) {
                if self[j] > self[j + 1] {
                    let mut temp: T = std::mem::take(&mut self[j]);
                    self[j] = std::mem::take(&mut self[j + 1]);
                    self[j + 1] = std::mem::take(&mut temp);
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn bs_test() {
    let mut vec = vec![10, -5, 6, 3, -5, 7, 8, 50, 12, -15, -82, -75];
    vec.bubble_sort();

    assert!(vec.windows(2).all(|window| window[0] <= window[1]));
}
