mod radix_sort;

#[cfg(test)]
mod tests {
    use crate::radix_sort::RadixSort;
    use rand::Rng;
    use std::collections::VecDeque;

    const SIZE : usize = 10_000_000;

    #[test]
    fn u8_radix_sort_test() {
        let mut vec : Vec<u8> = (0..SIZE).map(|_| rand::random::<u8>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u16_radix_sort_test() {
        let mut vec : Vec<u16> = (0..SIZE).map(|_| rand::random::<u16>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u32_radix_sort_test() {
        let mut vec : Vec<u32> = (0..SIZE).map(|_| rand::random::<u32>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u64_radix_sort_test() {
        let mut vec : Vec<u64> = (0..SIZE).map(|_| rand::random::<u64>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u128_radix_sort_test() {
        let mut vec : Vec<u128> = (0..SIZE).map(|_| rand::random::<u128>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn usize_radix_sort_test() {
        let mut vec : Vec<usize> = (0..SIZE).map(|_| rand::random::<usize>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i8_radix_sort_test() {
        let mut vec : Vec<i8> = (0..SIZE).map(|_| rand::random::<i8>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i16_radix_sort_test() {
        let mut vec : Vec<i16> = (0..SIZE).map(|_| rand::random::<i16>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i32_radix_sort_test() {
        let mut vec : Vec<i32> = (0..SIZE).map(|_| rand::random::<i32>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i64_radix_sort_test() {
        let mut vec : Vec<i64> = (0..SIZE).map(|_| rand::random::<i64>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i128_radix_sort_test() {
        let mut vec : Vec<i128> = (0..SIZE).map(|_| rand::random::<i128>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn isize_radix_sort_test() {
        let mut vec : Vec<isize> = (0..SIZE).map(|_| rand::random::<isize>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn f32_radix_sort_test() {
        let mut vec : Vec<f32> = (0..SIZE).map(|_| rand::random::<f32>()*rand::random::<i32>() as f32).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn f64_radix_sort_test() {
        let mut vec : Vec<f64> = (0..SIZE).map(|_| rand::random::<f64>()*rand::random::<i64>() as f64).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }


    #[test]
    fn char_radix_sort_test() {
        let mut vec : Vec<char> = (0..SIZE).map(|_| rand::random::<char>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn bool_radix_sort_test() {
        let mut vec : Vec<bool> = (0..SIZE).map(|_| rand::random::<bool>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn string_radix_sort(){
        let mut vec : Vec<String> = (0..100_000).map(|_| {
            let iter = 0..=rand::thread_rng().gen_range(0..=100);
            iter.map(|_|rand::random::<char>()).collect()
        }).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u8_slice_radix_sort_test() {
        let mut vec : VecDeque<u8> = (0..SIZE).map(|_| rand::random::<u8>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u16_slice_radix_sort_test() {
        let mut vec : VecDeque<u16> = (0..SIZE).map(|_| rand::random::<u16>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u32_slice_radix_sort_test() {
        let mut vec : VecDeque<u32> = (0..SIZE).map(|_| rand::random::<u32>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u64_slice_radix_sort_test() {
        let mut vec : VecDeque<u64> = (0..SIZE).map(|_| rand::random::<u64>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn u128_slice_radix_sort_test() {
        let mut vec : VecDeque<u128> = (0..SIZE).map(|_| rand::random::<u128>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn usize_slice_radix_sort_test() {
        let mut vec : VecDeque<usize> = (0..SIZE).map(|_| rand::random::<usize>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i8_slice_radix_sort_test() {
        let mut vec : VecDeque<i8> = (0..SIZE).map(|_| rand::random::<i8>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i16_slice_radix_sort_test() {
        let mut vec : VecDeque<i16> = (0..SIZE).map(|_| rand::random::<i16>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i32_slice_radix_sort_test() {
        let mut vec : VecDeque<i32> = (0..SIZE).map(|_| rand::random::<i32>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i64_slice_radix_sort_test() {
        let mut vec : VecDeque<i64> = (0..SIZE).map(|_| rand::random::<i64>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn i128_slice_radix_sort_test() {
        let mut vec : VecDeque<i128> = (0..SIZE).map(|_| rand::random::<i128>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn isize_slice_radix_sort_test() {
        let mut vec : VecDeque<isize> = (0..SIZE).map(|_| rand::random::<isize>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort();
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn f32_slice_radix_sort_test() {
        let mut vec : VecDeque<f32> = (0..SIZE).map(|_| rand::random::<f32>()*rand::random::<i32>() as f32).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn f64_slice_radix_sort_test() {
        let mut vec : VecDeque<f64> = (0..SIZE).map(|_| rand::random::<f64>()*rand::random::<i64>() as f64).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }


    #[test]
    fn char_slice_radix_sort_test() {
        let mut vec : VecDeque<char> = (0..SIZE).map(|_| rand::random::<char>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn bool_slice_radix_sort_test() {
        let mut vec : VecDeque<bool> = (0..SIZE).map(|_| rand::random::<bool>()).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn string_slice_radix_sort(){
        let mut vec : VecDeque<String> = (0..100_000).map(|_| {
            let iter = 0..=rand::thread_rng().gen_range(0..=100);
            iter.map(|_|rand::random::<char>()).collect()
        }).collect();
        let mut vec_clone = vec.clone();
        vec.make_contiguous().radix_sort();
        vec_clone.make_contiguous().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }
}
