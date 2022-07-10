use super::Graph;

impl Graph {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let mut visited:Vec<Vec<(Option<bool>, Option<bool>)>> = vec![vec![(None, None);n];n];

        vec![vec![0;1]]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}