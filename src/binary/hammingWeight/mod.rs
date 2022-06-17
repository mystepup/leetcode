use crate::binary::Binary;

impl Binary {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;
        let mut count = 0;

        while n > 0 {
            if n % 2 == 1 {
                count += 1;
                n = n / 2;
            } else {
                n = n / 2;
            }
        }

        count
    } 
}
