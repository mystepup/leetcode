use std::collections::HashMap;

use super::Graph;

impl Graph {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.len() == 0 {
            return false;
        }

        let mut course_to_prerequisite: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut find_start = vec![false; num_courses as usize];

        for courses in prerequisites.iter() {
            find_start[courses[0] as usize] = true;
            match course_to_prerequisite.get_mut(&courses[1]) {
                Some(c) => {
                    c.push(courses[0]);
                },
                None => {
                    course_to_prerequisite.insert(courses[1], vec![courses[0]]);
                }
            };
        }

        let start = match find_start.into_iter().position(|x| x == false) {
            Some(ok) => return true,
            None => {
                return false;
            }
        };

        let mut visited = vec![false; num_courses as usize];
        let mut stack: Vec<i32> = Vec::new();

        stack.push(start);
        while stack.len() != 0 {
            let v = stack.pop().unwrap();

            if visited[v as usize] != true {
                visited[v as usize] = true;
                match course_to_prerequisite.get(&v) {
                    Some(pre) => {
                        for &p in pre.iter() {
                            stack.push(p);
                        }
                    },
                    None => {} 
                };

            }
        }

        match visited.into_iter().find(|&x| x == false) {
            Some(_) => false,
            None => true
        }

    }
}

// Example 1:

// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: true
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0. So it is possible.

// Example 2:

// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
// Output: false
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.

// Example 3:

// Input: numCourses = 3, prerequisites = [[2,0],[0,1], [1, 0]]
