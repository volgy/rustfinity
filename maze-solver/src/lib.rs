use std::collections::VecDeque;

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let size_x = maze[0].len() as isize;
    let size_y = maze.len() as isize;

    let mut paths = VecDeque::new();
    paths.push_back(vec![start]);

    while let Some(path) = paths.pop_front() {
        let curr = *path.last().unwrap();
        if curr == end {
            return path;
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (y, x) = (curr.0 as isize + dx, curr.1 as isize + dy);
            if (0..size_x).contains(&x) && (0..size_y).contains(&y) {
                let next = (y as usize, x as usize);
                if maze[next.0][next.1] != '#' && !path.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    paths.push_back(new_path);
                }
            }
        }
    }

    vec![]
}
