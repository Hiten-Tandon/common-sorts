const RADIX : usize = 256;
pub trait RadixSort {
    //Radix sort can only sort unsigned integers
    //for u8, it's same as counting sort, so, I've used it, so as to reduce boilerplate
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

impl RadixSort for Vec<u8> {
    fn radix_sort(&mut self){
        let len = self.len();
        Self::u8_sorting_routine(self, len);
    }
}

impl RadixSort for Vec<i8> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
            } else {
                pos_count += 1;
            }
        });

        let mut negatives : Vec<u8> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u8> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u8) } else { positives.push(i as u8) });

        Self::u8_sorting_routine(&mut negatives, neg_count);
        Self::u8_sorting_routine(&mut positives, pos_count);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i8);
    }
}

impl RadixSort for Vec<u16> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u16_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for Vec<i16> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u16 = 0;
        let mut pos_max   : u16 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u16);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u16);
            }
        });

        let mut negatives : Vec<u16> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u16> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u16) } else { positives.push(i as u16) });

        Self::u16_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u16_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i16);
    }
}

impl RadixSort for Vec<u32> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u32_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for Vec<i32> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u32 = 0;
        let mut pos_max   : u32 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u32);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u32);
            }
        });

        let mut negatives : Vec<u32> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u32> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u32) } else { positives.push(i as u32) });

        Self::u32_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u32_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i32);
    }
}

impl RadixSort for Vec<u64> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u64_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for Vec<i64> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u64 = 0;
        let mut pos_max   : u64 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u64);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u64);
            }
        });

        let mut negatives : Vec<u64> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u64> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u64) } else { positives.push(i as u64) });

        Self::u64_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u64_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i64);
    }
}

impl RadixSort for Vec<u128> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u128_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for Vec<i128> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u128 = 0;
        let mut pos_max   : u128 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u128);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u128);
            }
        });

        let mut negatives : Vec<u128> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u128> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u128) } else { positives.push(i as u128) });

        Self::u128_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u128_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i128);
    }
}

impl RadixSort for Vec<usize> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::usize_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for Vec<isize> {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : usize = 0;
        let mut pos_max   : usize = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as usize);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as usize);
            }
        });

        let mut negatives : Vec<usize> = Vec::with_capacity(neg_count);
        let mut positives : Vec<usize> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as usize) } else { positives.push(i as usize) });

        Self::usize_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::usize_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as isize);
    }
}

impl RadixSort for Vec<f32>{
    fn radix_sort(&mut self){
        let mut i32_rep :Vec<i32>;

        unsafe{
            i32_rep = std::mem::transmute::<Vec<f32>, Vec<i32>>(self.clone());
        }
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u32 = 0;
        let mut pos_max   : u32 = 0;

        i32_rep.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u32);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u32);
            }
        });

        let mut negatives : Vec<u32> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u32> = Vec::with_capacity(pos_count);

        i32_rep.iter().for_each(|&i| if i < 0 { negatives.push(!i as u32) } else { positives.push(i as u32) });

        Self::u32_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u32_sorting_routine(&mut positives, pos_count, pos_max);

        i32_rep
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i32);

        unsafe{
             *self = std::mem::transmute::<Vec<i32>, Vec<f32>>(i32_rep.clone());
        }
    }
}

impl RadixSort for Vec<f64>{
    fn radix_sort(&mut self){
        let mut i64_rep :Vec<i64>;

        unsafe{
            i64_rep = std::mem::transmute::<Vec<f64>, Vec<i64>>(self.clone());
        }
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u64 = 0;
        let mut pos_max   : u64 = 0;

        i64_rep.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u64);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u64);
            }
        });

        let mut negatives : Vec<u64> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u64> = Vec::with_capacity(pos_count);

        i64_rep.iter().for_each(|&i| if i < 0 { negatives.push(!i as u64) } else { positives.push(i as u64) });

        Self::u64_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u64_sorting_routine(&mut positives, pos_count, pos_max);

        i64_rep
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i64);

        unsafe{
             *self = std::mem::transmute::<Vec<i64>, Vec<f64>>(i64_rep.clone());
        }
    }
}

impl RadixSort for Vec<bool> {
    fn radix_sort(&mut self){
        Self::bool_sorting_routine(self);
    }
}

impl RadixSort for Vec<char> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::char_sorting_routine(self, len, max_ele);
    }
}
///experimental
impl RadixSort for Vec<String> {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_str_len = self.iter().max_by_key(|s| s.len()).unwrap_or(&String::new()).len();
        Self::string_sorting_routine(self, len, max_str_len);
    }
}

impl RadixSort for [u8] {
    fn radix_sort(&mut self){
        let len = self.len();
        Self::u8_sorting_routine(self, len);
    }
}

impl RadixSort for [i8] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
            } else {
                pos_count += 1;
            }
        });

        let mut negatives : Vec<u8> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u8> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u8) } else { positives.push(i as u8) });

        Self::u8_sorting_routine(&mut negatives, neg_count);
        Self::u8_sorting_routine(&mut positives, pos_count);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i8);
    }
}

impl RadixSort for [u16] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u16_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [i16] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u16 = 0;
        let mut pos_max   : u16 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u16);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u16);
            }
        });

        let mut negatives : Vec<u16> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u16> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u16) } else { positives.push(i as u16) });

        Self::u16_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u16_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i16);
    }
}

impl RadixSort for [u32] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u32_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [i32] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u32 = 0;
        let mut pos_max   : u32 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u32);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u32);
            }
        });

        let mut negatives : Vec<u32> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u32> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u32) } else { positives.push(i as u32) });

        Self::u32_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u32_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i32);
    }
}

impl RadixSort for [u64] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u64_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [i64] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u64 = 0;
        let mut pos_max   : u64 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u64);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u64);
            }
        });

        let mut negatives : Vec<u64> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u64> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u64) } else { positives.push(i as u64) });

        Self::u64_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u64_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i64);
    }
}

impl RadixSort for [u128] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::u128_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [i128] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u128 = 0;
        let mut pos_max   : u128 = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u128);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u128);
            }
        });

        let mut negatives : Vec<u128> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u128> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as u128) } else { positives.push(i as u128) });

        Self::u128_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u128_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i128);
    }
}

impl RadixSort for [usize] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::usize_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [isize] {
    fn radix_sort(&mut self){
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : usize = 0;
        let mut pos_max   : usize = 0;

        self.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as usize);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as usize);
            }
        });

        let mut negatives : Vec<usize> = Vec::with_capacity(neg_count);
        let mut positives : Vec<usize> = Vec::with_capacity(pos_count);

        self.iter().for_each(|&i| if i < 0 { negatives.push(!i as usize) } else { positives.push(i as usize) });

        Self::usize_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::usize_sorting_routine(&mut positives, pos_count, pos_max);
        negatives.reverse();

        self
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as isize);
    }
}

impl RadixSort for [f32]{
    fn radix_sort(&mut self){
        let i32_rep : &mut [i32];

        unsafe{
            i32_rep = std::mem::transmute::<&mut [f32], &mut [i32]>(self);
        }
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u32 = 0;
        let mut pos_max   : u32 = 0;

        i32_rep.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u32);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u32);
            }
        });

        let mut negatives : Vec<u32> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u32> = Vec::with_capacity(pos_count);

        i32_rep.iter().for_each(|&i| if i < 0 { negatives.push(!i as u32) } else { positives.push(i as u32) });

        Self::u32_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u32_sorting_routine(&mut positives, pos_count, pos_max);

        i32_rep
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i32);

        unsafe{
             self.copy_from_slice(std::mem::transmute::<&mut [i32], &mut [f32]>(i32_rep));
        }
    }
}

impl RadixSort for [f64]{
    fn radix_sort(&mut self){
        let i64_rep : &mut [i64];

        unsafe{
            i64_rep = std::mem::transmute::<&mut [f64], &mut [i64]>(self);
        }
        let mut neg_count : usize = 0;
        let mut pos_count : usize = 0;
        let mut neg_max   : u64 = 0;
        let mut pos_max   : u64 = 0;

        i64_rep.iter().for_each(|&i|{
            if i < 0 {
                neg_count += 1;
                neg_max = neg_max.max(!i as u64);
            } else {
                pos_count += 1;
                pos_max = pos_max.max(i as u64);
            }
        });

        let mut negatives : Vec<u64> = Vec::with_capacity(neg_count);
        let mut positives : Vec<u64> = Vec::with_capacity(pos_count);

        i64_rep.iter().for_each(|&i| if i < 0 { negatives.push(!i as u64) } else { positives.push(i as u64) });

        Self::u64_sorting_routine(&mut negatives, neg_count, neg_max);
        Self::u64_sorting_routine(&mut positives, pos_count, pos_max);

        i64_rep
            .iter_mut()
            .zip(
                negatives
                .into_iter()
                .map(|neg| !neg)
                .chain(positives.into_iter()))
            .for_each(|(ele_arr, ele)| *ele_arr = ele as i64);

        unsafe{
             self.copy_from_slice(std::mem::transmute::<&mut [i64], &mut [f64]>(i64_rep));
        }
    }
}
impl RadixSort for [bool] {
    fn radix_sort(&mut self){
        Self::bool_sorting_routine(self);
    }
}

impl RadixSort for [char] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_ele = *self.iter().max().unwrap();

        Self::char_sorting_routine(self, len, max_ele);
    }
}

impl RadixSort for [String] {
    fn radix_sort(&mut self){
        let len = self.len();
        let max_str_len = self.iter().max_by_key(|s| s.len()).unwrap_or(&String::new()).len();
        Self::string_sorting_routine(self, len, max_str_len);
    }
}
