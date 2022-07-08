use std::collections::HashMap;

use super::Graph;

impl Graph {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        // create adjacent list
        for courses in prerequisites.iter() {
            let prev_course = courses[1];
            let curr_course = courses[0];

            match adjacency_list.get_mut(&prev_course) {
                Some(course) => {
                    course.push(curr_course);
                },
                None => {
                    adjacency_list.insert(prev_course, vec![curr_course]);
                }
            }
        }

        let mut visited = vec![false;num_courses as usize];
        let mut checked = vec![false;num_courses as usize];

        for curr_course in 0..num_courses {
            if Graph::is_cyclic(&adjacency_list, curr_course, &mut visited, &mut checked) {
                return false;
            }
        }

        true
    }

    fn is_cyclic(adjacency_list: &HashMap<i32, Vec<i32>>, curr_course: i32, visited: &mut Vec<bool>, checked: &mut Vec<bool>) -> bool {
        if checked[curr_course as usize] {
            return false;
        }
        
        if visited[curr_course as usize] {
            return true;
        }

        if !adjacency_list.contains_key(&curr_course) {
            return false;
        }

        visited[curr_course as usize] = true;
        let mut result = false;

        for &next_course in adjacency_list.get(&curr_course).unwrap().iter() {
            result = Graph::is_cyclic(adjacency_list, next_course, visited, checked);
            if result {
                break;
            }
        }

        visited[curr_course as usize] = false;
        checked[curr_course as usize] = true;
        return result;
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
