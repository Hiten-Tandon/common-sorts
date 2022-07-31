mod radix_sort;

#[cfg(test)]
mod tests {
    use crate::radix_sort::RadixSort;
    use rand::Rng;

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
        let mut vec : Vec<f32> = (0..SIZE).map(|_| rand::random::<f32>()).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }

    #[test]
    fn f64_radix_sort_test() {
        let mut vec : Vec<f64> = (0..SIZE).map(|_| rand::random::<f64>()).collect();
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
        let mut vec : Vec<String> = (0..10_000).map(|_| {
            let iter = 0..=rand::thread_rng().gen_range(0..=100);
            iter.map(|_|rand::random::<char>()).collect()
        }).collect();
        let mut vec_clone = vec.clone();
        vec.radix_sort();
        vec_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec_clone);
    }
}
