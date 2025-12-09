#[derive(Debug)]
pub struct Result {
    median: Option<i32>,
    mode: Option<i32>,
}

impl Result {
    pub fn get_median(&self) -> Option<i32> {
        self.median
    }
    pub fn get_mode(&self) -> Option<i32> {
        self.mode
    }
}

pub fn get_result(list: &mut Vec<i32>) -> Result {
    list.sort();

    let Some(mode) = list.get(0) else {
        return Result {
            median: None,
            mode: None,
        };
    };
    // Roughly getting the same time of about 11-13ms on both probably because the data is sorted leading to branch predictor working maybe
    // Branchless version
    let mut mode = *mode;
    let mut count = 1;
    let mut max = 1;
    let mut prev_num = mode;
    for num in list.iter().skip(1) {
        let mut reduce = num ^ prev_num;
        reduce |= reduce >> 16;
        reduce |= reduce >> 8;
        reduce |= reduce >> 4;
        reduce |= reduce >> 2;
        reduce |= reduce >> 1;
        reduce &= 1;
        reduce = 1 - reduce;
        count = (count * reduce) + 1;
        // count = count * (*num == prev_num) as i32 + 1;
        prev_num = *num;
        let mask = (max - count) >> 31;
        mode = (num & mask) | (mode & !mask);
        max = (count & mask) | (max & !mask);
        // max = count * (count > max) as i32 + max * (count <= max) as i32;
        // mode = num * (count > max) as i32 + mode * (count <= max) as i32;
    }

    // Branch version
    // let mut mode = *mode;
    // let mut mode = *list.get(1).unwrap();
    // let mut count = 0;
    // let mut max = 0;
    // let mut current = mode;
    // for num in list.iter() {
    //     if current == *num {
    //         count += 1;
    //     } else {
    //         if count > max {
    //             max = count;
    //             mode = *num;
    //         }
    //         current = *num;
    //         count = 1;
    //     }
    // }
    // if count > max {
    //     mode = current;
    // }

    Result {
        median: Some(list[list.len() / 2]),
        mode: Some(mode),
    }
}
