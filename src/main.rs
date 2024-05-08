extern crate crossterm;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
// use k_board::{keyboard::Keyboard, keys::Keys};
use std::io::{self, stdout};

struct Snake {
    body: Vec<(usize, usize)>,
    head_x: usize,
    head_y: usize,
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
        let _ = update(&mut board);
        draw(&board);
        // std::thread::sleep(one_second);
    }
}

fn init() -> Board {
    let snake: Snake = Snake {
        head_x: 10,
        head_y: 10,
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

fn update(board: &mut Board) -> io::Result<()> {
    enable_raw_mode()?;

    // let mut stdout = stdout();

    if let Event::Key(KeyEvent {
        code,
        modifiers,
        kind,
        state,
    }) = crossterm::event::read()?
    {
        match code {
            KeyCode::Up => {
                if board.snake.head_y > 0 {
                    board
                        .snake
                        .body
                        .insert(0, (board.snake.head_x, board.snake.head_y));
                    board.snake.body.pop();
                    board.snake.head_y -= 1;
                }
            }
            KeyCode::Down => {
                if board.snake.head_y < board.heigth - 1 {
                    board
                        .snake
                        .body
                        .insert(0, (board.snake.head_x, board.snake.head_y));
                    board.snake.body.pop();
                    board.snake.head_y += 1;
                }
            }
            KeyCode::Left => {
                if board.snake.head_x > 0 {
                    board
                        .snake
                        .body
                        .insert(0, (board.snake.head_x, board.snake.head_y));
                    board.snake.body.pop();
                    board.snake.head_x -= 1;
                }
            }
            KeyCode::Right => {
                if board.snake.head_x < board.width - 1 {
                    board
                        .snake
                        .body
                        .insert(0, (board.snake.head_x, board.snake.head_y));
                    board.snake.body.pop();
                    board.snake.head_x += 1;
                }
            }
            KeyCode::Char('q') => {}
            _ => {}
        }
    }
    disable_raw_mode()?;
    Ok(())
    // if let Some(key) = Keyboard::new().next() {
    //     match key {
    //         Keys::Up => {
    //             if board.snake.head_y > 0 {
    //                 board
    //                     .snake
    //                     .body
    //                     .insert(0, (board.snake.head_x, board.snake.head_y));
    //                 board.snake.body.pop();
    //                 board.snake.head_y -= 1;
    //             }
    //         }
    //         Keys::Down => {
    //             if board.snake.head_y < board.heigth - 1 {
    //                 board
    //                     .snake
    //                     .body
    //                     .insert(0, (board.snake.head_x, board.snake.head_y));
    //                 board.snake.body.pop();
    //                 board.snake.head_y += 1;
    //             }
    //         }
    //         Keys::Left => {
    //             if board.snake.head_x > 0 {
    //                 board
    //                     .snake
    //                     .body
    //                     .insert(0, (board.snake.head_x, board.snake.head_y));
    //                 board.snake.body.pop();
    //                 board.snake.head_x -= 1;
    //             }
    //         }
    //         Keys::Right => {
    //             if board.snake.head_x < board.width - 1 {
    //                 board
    //                     .snake
    //                     .body
    //                     .insert(0, (board.snake.head_x, board.snake.head_y));
    //                 board.snake.body.pop();
    //                 board.snake.head_x += 1;
    //             }
    //         }
    //         _ => {}
    //     }
    // }
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
            if line == board.snake.head_y && i == board.snake.head_x {
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
