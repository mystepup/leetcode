use crate::array::Array;

impl Array {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let height:Vec<usize> = height.iter().map(|&x| x as usize).collect();

        let mut start = 0;
        let mut end = height.len() - 1;
        let mut max = 0;

        while start < end {
            let candid = (end - start) * if height[start] > height[end] { height[end] } else { height[start] };

            max = if max > candid { max } else { candid };

            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        max as i32
    }
}

