type DSU struct {
	parent []int
	size []int
}

func (dsu *DSU) Union(a, b int) bool {
	pa := dsu.FindParent(a)
	pb := dsu.FindParent(b)

	if pa == pb {
		return false
	}

	if dsu.size[pa] >= dsu.size[pb] {
		dsu.size[pa] += dsu.size[pb]
		dsu.parent[pb] = pa
	} else {
		dsu.size[pb] += dsu.size[pa]
		dsu.parent[pa] = pb
	}
	return true

}

func (dsu *DSU) Size(node int) int {
	p := dsu.FindParent(node)
	return dsu.size[p]
}

func (dsu *DSU) FindParent(node int) int {
	if dsu.parent[node] != node {
		dsu.parent[node] = dsu.FindParent(dsu.parent[node])
	}
	return dsu.parent[node]
}

func maxAreaOfIsland(grid [][]int) int {
    rows, cols := len(grid), len(grid[0])

	dsu := DSU{parent: make([]int, rows*cols+1), size:  make([]int, rows*cols+1)}

	for i := 0; i <= rows*cols ; i ++ {
		dsu.parent[i] = i
		dsu.size[i] = 1
	}
	
	directions := [][]int{{1,0}, {0,1}, {0,-1}, {-1,0}}

	ans := 0

	index := func (r, c int) int {
	return r*cols + c
	}

	for i := range rows {
		for j := range cols {
			if grid[i][j] == 1 {
				for _, dir := range directions {
					x, y:= i+dir[0],j+dir[1]

					if x < 0 || y < 0 || x >=rows || y >= cols || grid[x][y] == 0 {
						continue
					}
					dsu.Union(index(i,j), index(x, y))
				}
				ans = max(ans, dsu.Size(index(i, j)))
			}
		}
	}

	return ans
}

