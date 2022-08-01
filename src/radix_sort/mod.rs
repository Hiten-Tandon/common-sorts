//!Radix Sort is a sorting algorithm, particulalry useful when sorting, large sets of data. 
//!It is an extension of the counting sort algorithm, and is a stable sorting algorithm.
//!Radix sort has many implementations, that vary based on the type[ LSD / MSD ], Radix of the sort
//!and the implementation method, i.e. counting sort or bucket sort.
//!The one used here is LSD base - 255 radix sort, using counting sort. 
mod vec_implementations;
mod slice_implementations;
mod lib;


const RADIX : usize = 256;
pub trait RadixSort {
    fn u8_sorting_routine(arr : &mut [u8], len : usize){ 
        let mut counts : [usize; RADIX] = [0; RADIX];
        arr.iter().for_each(|&i| counts[i as usize] += 1);
        let mut i : usize = 0;
        let mut j : usize = 0;
        while i < len {
            while counts[j] > 0{
                arr[i] = j as u8;
                i += 1;
                counts[j] -= 1;
            }
            j += 1;
        }
    }
    
    fn u16_sorting_routine(arr : &mut [u16], len : usize, mut max_ele : u16) {
        let mut exp : u8 = 0;
       
        while max_ele > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<u16> = vec![0; len];
            arr.iter()
                .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
                temp[*idx] = arr[i];
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr.copy_from_slice(&temp);
            max_ele = max_ele.wrapping_shr(8);
            exp += 1;
        }
    }

    fn u32_sorting_routine(arr : &mut [u32], len : usize, mut max_ele : u32) {

        let mut exp : u8 = 0;
       
        while max_ele > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<u32> = vec![0; len];
            arr.iter()
                .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
                temp[*idx] = arr[i];
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr.copy_from_slice(&temp);
            max_ele = max_ele.wrapping_shr(8);
            exp += 1;
        }
    }

    fn u64_sorting_routine(arr : &mut [u64], len : usize, mut max_ele : u64) {
        
        let mut exp : u8 = 0;
       
        while max_ele > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<u64> = vec![0; len];
            arr.iter()
                .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
                temp[*idx] = arr[i];
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr.copy_from_slice(&temp);
            max_ele = max_ele.wrapping_shr(8);
            exp += 1;
        }
    }

    fn u128_sorting_routine(arr : &mut [u128], len : usize, mut max_ele : u128){

        let mut exp : u8 = 0;
       
        while max_ele > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<u128> = vec![0; len];
            arr.iter()
                .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
                temp[*idx] = arr[i];
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr.copy_from_slice(&temp);
            max_ele = max_ele.wrapping_shr(8);
            exp += 1;
        }
    }

    fn usize_sorting_routine(arr : &mut[usize], len : usize, mut max_ele : usize){

        let mut exp : u8 = 0;
       
        while max_ele > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<usize> = vec![0; len];
            arr.iter()
                .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
                temp[*idx] = arr[i];
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr.copy_from_slice(&temp);
            max_ele = max_ele.wrapping_shr(8);
            exp += 1;
        }
    }

    fn char_sorting_routine(arr : &mut[char], len : usize, max_ele : char){
        let max_ele : u32 = max_ele as u32;
        let mut arr_cpy = arr.iter().map(|&c| c as u32).collect::<Vec<u32>>();
        Self::u32_sorting_routine(&mut arr_cpy, len, max_ele);
        arr.clone_from_slice(&arr_cpy.into_iter().map(|c| char::from_u32(c).unwrap()).collect::<Vec<char>>());
    }
    
    ///RADIX 256 is overkill for two elements, so radix 2 has been used instead
    fn bool_sorting_routine(arr : &mut[bool]){
        let mut false_count : usize = 0;
        let mut true_count  : usize = 0;

        arr.iter().for_each(|&b| if b {true_count += 1} else {false_count += 1});

        let mut i = 0;
        while false_count > 0 {
            arr[i] = false;
            i += 1;
            false_count -= 1;
        }

        while true_count > 0 { 
            arr[i] = true;
            i += 1;
            true_count -= 1;
        }
    }
    
    ///Though the option is provided, use the standard sort for sorting strings, pretty much
    ///whenever you can, because, radix sort is really bad for sorting strings. Radix Sort has a
    ///time complexity of O(n), many people share this misbelief. it is NOT possible to sort an
    ///array in O(n), The closest thing to that is counting sort, but even that is O(2n). The
    ///actual time complexity of  radix sort is, O(kn + kr), where, n is the length of array, r is
    ///the radix(256 here), and k = ceil(logr(max_element)) or, here, since the radix is 256, the
    ///expression can be simplified to k = byte_count(max_element). Which basically means, if you
    ///were to have a name of say 20 ASCII characters, as the max_element in an array of say 1024
    ///names, here, the time complexity would come to be 20*1024 + 20*256, compared to
    ///10*1024(worst case of tim sort / standard sort for rust).
    fn string_sorting_routine(arr : &mut [String], len : usize, mut max_str_len : usize){
        let mut arr_cpy : Vec<Vec<u8>> = arr.iter().map(|ele| ele.bytes().collect()).collect();
        while max_str_len > 0 {
            let mut count : [usize; RADIX] = [0; RADIX];
            let mut temp  : Vec<Vec<u8>> = vec![vec![]; len];
            arr_cpy.iter()
                .for_each(|x| count[*(x.get(max_str_len - 1).unwrap_or(&0)) as usize] += 1);//counting each byte's occurence 

            count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

            for i in 1..RADIX {
                count[i] = count[i].wrapping_add(count[i - 1]);//performing the prefix-sum or rolling sum of the array
            }

            (0..len).rev().for_each(|i| {
                let idx = &mut count[*(arr_cpy[i].get(max_str_len - 1).unwrap_or(&0)) as usize];
                std::mem::swap(&mut temp[*idx], &mut arr_cpy[i]);
                if *idx > 0 {
                    *idx -= 1
                };
            });

            arr_cpy.clone_from_slice(&temp);
            max_str_len -= 1; 
        }
        let arr_cpy : Vec<String> = arr_cpy.into_iter().map(|ele| String::from_utf8(ele).expect("Invalid UTF-8 character")).collect();
        arr.clone_from_slice(&arr_cpy);
    }

    fn radix_sort(&mut self);
}



