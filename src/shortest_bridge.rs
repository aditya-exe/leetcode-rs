use std::collections::{HashSet, VecDeque};

pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    let moves = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let start_point = find_start(&grid);
    let mut start_water = HashSet::<((usize, usize), i32)>::new();
    find_start_water(&mut start_water, &mut grid, &moves, start_point);
    return find_shortest_bridge(&mut grid, &start_water, &moves)
}

fn find_start_water(start_water: &mut HashSet<((usize, usize), i32)>, grid: &mut Vec<Vec<i32>>, moves: &Vec<(i32, i32)>, start_point: (usize, usize)) {
    grid[start_point.0][start_point.1] = -1;
    for m in moves.iter() {
        let next_x = (start_point.0 as i32 + m.0) as usize;
        let next_y = (start_point.1 as i32 + m.1) as usize;
        if next_x >= grid.len() || next_y >= grid[0].len() {
            continue;
        }

        match grid[next_x][next_y] {
            0 => {
                start_water.insert(((next_x, next_y), 0));
            },
            1 => find_start_water(start_water, grid, moves, (next_x, next_y)),
            _ => (),
        }
    }
}
fn find_shortest_bridge(grid: &mut Vec<Vec<i32>>, start_water: &HashSet<((usize, usize), i32)>, moves: &Vec<(i32, i32)>) -> i32 {
    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::from_iter(start_water.into_iter().map(|&x| x).collect::<Vec<_>>());

    while !queue.is_empty() {
        let (point, value) = queue.pop_front().unwrap();

        for m in moves.iter() {
            let next_x = (point.0 as i32 + m.0) as usize;
            let next_y = (point.1 as i32 + m.1) as usize;
            if next_x >= grid.len() || next_y >= grid[0].len() {
                continue;
            }

            match grid[next_x][next_y] {
                0 => {
                    grid[next_x][next_y] = -1;
                    queue.push_back(((next_x, next_y), value + 1));
                }
                1 => {
                    return value + 1;
                }
                _ => (),
            }
        }
    }

    return 0;
}

fn find_start(grid: &Vec<Vec<i32>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 1 {
                return (i, j);
            }
        }
    }

    return (0, 0);
}
