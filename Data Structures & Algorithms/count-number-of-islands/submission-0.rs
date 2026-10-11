impl Solution {

    pub fn dfs(i: i32, j: i32, rows: i32, cols: i32, grid: &mut Vec<Vec<char>>) {
        if i < 0 || j < 0 || i >= rows || j >= cols || grid[i as usize][j as usize] == '0' {
            return;
        }   

        grid[i as usize][j as usize] = '0';

        Self::dfs(i+1, j, rows, cols, grid);
        Self::dfs(i, j+1, rows, cols, grid);
        Self::dfs(i-1, j, rows, cols, grid);
        Self::dfs(i, j-1, rows, cols, grid);
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();


        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    Self::dfs(i as i32,j as i32,rows as i32,cols as i32,  &mut grid);
                    ans += 1;
                }
            }
        }
        ans
    }
}
