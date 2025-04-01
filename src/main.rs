use k_board::{keyboard::Keyboard, keys::Keys};

struct Snake {
    body: Vec<(usize, usize)>, // body x, y
    head: (usize, usize),
    head_char: char,
    body_char: char,
}

struct Board {
    snake: Snake,
    width: usize,
    heigth: usize,
    padding: usize,
    vert_wall_char: char,
    hor_wall_char: char,
}

fn main() {
    // let one_second = std::time::Duration::from_millis(100);
    let mut board: Board = init();

    loop {
        update(&mut board);
        draw(&board);
        // std::thread::sleep(one_second);
    }
}

fn init() -> Board {
    let snake: Snake = Snake {
        head: (10, 10),
        body: vec![(8, 10), (9, 10)],
        head_char: '@',
        body_char: '%',
    };
    let board: Board = Board {
        snake,
        width: 50,
        heigth: 15,
        padding: 5,
        vert_wall_char: '|',
        hor_wall_char: '~',
    };
    board
}

fn update(board: &mut Board) {
    if let Some(key) = Keyboard::new().next() {
        match key {
            Keys::Up => {
                if board.snake.head.1 > 0 {
                    board
                        .snake
                        .body
                        .insert(0, board.snake.head);
                    board.snake.body.pop();
                    board.snake.head.1 -= 1;
                }
            }
            Keys::Down => {
                if board.snake.head.1 < board.heigth - 1 {
                    board
                        .snake
                        .body
                        .insert(0, board.snake.head);
                    board.snake.body.pop();
                    board.snake.head.1 += 1;
                }
            }
            Keys::Left => {
                if board.snake.head.0 > 0 {
                    board
                        .snake
                        .body
                        .insert(0, board.snake.head);
                    board.snake.body.pop();
                    board.snake.head.0 -= 1;
                }
            }
            Keys::Right => {
                if board.snake.head.0 < board.width - 1 {
                    board
                        .snake
                        .body
                        .insert(0, board.snake.head);
                    board.snake.body.pop();
                    board.snake.head.0 += 1;
                }
            }
            _ => {}
        }
    }
}

fn draw(board: &Board) {
    clearscreen::clear().expect("failed to clear screen");
    // std::process::Command::new("clear").status().unwrap();

    // Print title bar
    println!("Snake Game\n");

    // Print top board wall
    println!(
        "{}{}",
        " ".repeat(board.padding + 1),
        board.hor_wall_char.to_string().repeat(board.width)
    );

    // Print board
    for line in 0..board.heigth {
        // Find all the body points that are on the same Y as the current line
        let points_in_y: Vec<usize> = board
            .snake
            .body
            .iter()
            .filter_map(|&(x, y)| if y == line { Some(x) } else { None })
            .collect();
        let mut board_string = String::new();

        // Create the current line string, choosing between head, body or empty char
        for i in 0..board.width {
            if line == board.snake.head.1 && i == board.snake.head.0 {
                board_string.push(board.snake.head_char);
            } else if points_in_y.contains(&i) {
                board_string.push(board.snake.body_char);
            } else {
                board_string.push(' ');
            }
        }

        // Print the current board line
        println!(
            "{}{}{}{}",
            " ".repeat(board.padding),
            board.vert_wall_char,
            board_string,
            board.vert_wall_char
        )
        // if board.snake.head_y == line {
        //     println!(
        //         "{}{}{}{}",
        //         " ".repeat(board.padding),
        //         board.vert_wall_char,
        //         " ".repeat(board.snake.head_x)
        //             + &board.snake.head_char.to_string()
        //             + &" ".repeat(board.width - board.snake.head_x - 1),
        //         board.vert_wall_char
        //     );
        // } else {
        //     println!(
        //         "{}{}{}{}",
        //         " ".repeat(board.padding),
        //         board.vert_wall_char,
        //         " ".repeat(board.width),
        //         board.vert_wall_char
        //     );
        // }
    }

    // Print bottom board wall
    println!(
        "{}{}",
        " ".repeat(board.padding),
        board.hor_wall_char.to_string().repeat(board.width + 1)
    );
}
