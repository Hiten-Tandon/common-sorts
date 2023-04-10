///This trait implements Bubble Sort for all the types it can
///Bubble Sort is a very simple sorting algorithm, which is genrally taught in intro to DSA to
///introduce algorithms, as such it's a very slow algorithm, with O(N²) time complexity.
///This trait is for educational purposes only and is not meant to be used for production, which is
///why bubble_sort method issues a warning
pub trait BubbleSort {
    fn bubble_sort(&mut self) {}
}

impl<T: PartialOrd> BubbleSort for [T] {
    #[cold]
    fn bubble_sort(&mut self) {
        eprintln!("Bubble sort is a slow sorting algorithm, it should not be used over tim sort which is the standard sorting algorithm");
        let len: usize = self.len();

        for i in 0..(len - 1) {
            for j in 0..(len - i - 1) {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn bs_test() {
    let mut vec: Vec<i32> = (0..10_000).map(|_| rand::random::<i32>()).collect();
    vec.bubble_sort();

    assert!(vec.windows(2).all(|win| win[0] <= win[1]));
}
