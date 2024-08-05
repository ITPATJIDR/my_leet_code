use std::cmp;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
          // Initialize distance matrix with infinity
        let mut distance = vec![vec![i32::MAX / 2; n as usize]; n as usize];
        
        for i in 0..n as usize {
            distance[i][i] = 0;  // Distance to itself is 0
        }
        
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            distance[u][v] = w;
            distance[v][u] = w;
        }
        
        // Floyd-Warshall algorithm to find all-pairs shortest path
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    distance[i][j] = cmp::min(distance[i][j], distance[i][k] + distance[k][j]);
                }
            }
        }
        
        let mut ans = -1;
        let mut mini = i32::MAX;
        
        for i in 0..n as usize {
            let mut count = 0;
            for j in 0..n as usize {
                if i != j && distance[i][j] <= distance_threshold {
                    count += 1;
                }
            }
            if count <= mini {
                mini = count;
                ans = i as i32;
            }
        }
        
        ans
    }
}