impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: i32, j: i32, rows: i32, cols: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
            if i < 0 || j < 0 || i >= rows || j >= cols || grid[i as usize][j as usize] == 0 {
                    return 0;
            }

            grid[i as usize][j as usize] = 0;

            return 1 + dfs(i+1,j,rows,cols,grid)+dfs(i-1,j,rows,cols,grid)+dfs(i,j+1,rows,cols,grid)+dfs(i,j-1,rows,cols,grid);
        }

        let mut ans = 0;
        let mut grid = grid;
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        for i in 0..rows {
            for j in 0..cols {
                ans = max(ans, dfs(i,j,rows,cols,&mut grid));
            }
        }

        ans
    }
}
