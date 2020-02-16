fn can_place(r: i32, c: i32, n: i32, board: &Vec<Vec<i32>>) -> bool {
    let mut can_place: bool = false;
    if !board[r as usize].contains(&n) {
        can_place = true;
        for i in 0..board.len() {
            if board[i][c as usize] == n {
                can_place = false;
                break
            }
        }
    }
    if can_place {
        let mut grid_x = 0;
        let mut grid_y = 0;
        if c < 3 {
            grid_x = 0
        } else if c < 6 {
            grid_x = 1
        } else {
            grid_x = 2
        }
        if r < 3 {
            grid_y = 0
        } else if r < 6 {
            grid_y = 1
        } else {
            grid_y = 2
        }
        for y in grid_y * 3..grid_y * 3 + 3 {
            for x in grid_x * 3..grid_x * 3 + 3 {
                if board[y][x] == n {
                    can_place = false;
                    break
                }
            }
            if !can_place {
                break
            }
        }
        if can_place {
            return true
        }
    }

    false
}

fn solve(mut r:i32, mut c:i32, mut board: &mut Vec<Vec<i32>>) -> bool {
    if c as usize == board[0].len() {
        c = 0;
        r += 1;
        if r as usize == board.len() {
            return true;
        }
    }

    if board[r as usize][c as usize] != -1 {
        return solve(r, c+1, board)
    }

    for n in 1..10 {
        if can_place(r, c, n, &board) {
            board[r as usize][c as usize] = n;
            if solve(r, c+1, board) {
                return true;
            }
            board[r as usize][c as usize] = -1;
        }
    }

    false
}

fn main() {
    let mut board: Vec<Vec<i32>> = vec![
        vec![5, 3, -1,  -1, 7, -1,  -1, -1, -1],
        vec![6, -1, -1,  1, 9, 5,   -1, -1, -1],
        vec![-1, 9, 8,  -1, -1, -1,  -1, 6, -1],

        vec![8, -1, -1,  -1, 6, -1,  -1, -1, 3],
        vec![4, -1, -1,  8, -1, 3,   -1, -1, 1],
        vec![7, -1, -1,  -1, 2, -1,  -1, -1, 6],

        vec![-1, 6, -1,  -1, -1, -1,  2, 8, -1],
        vec![-1, -1, -1,  4, 1, 9,    -1, -1, 5],
        vec![-1, -1, -1,  -1, 8, -1,  -1, 7, 9]
    ];

    solve(0, 0, &mut board);

    board.iter().for_each(|it| {
        println!("{:?}", it);
    })
}