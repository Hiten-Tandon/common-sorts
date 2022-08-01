use crate::radix_sort::RadixSort;

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