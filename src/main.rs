use k_board::{keyboard::Keyboard, keys::Keys};
use rand::seq::SliceRandom;

struct Fruit {
    current: usize,
    possibilities: Vec<(usize, usize, usize)>,
}

struct Snake {
    body: Vec<(usize, usize)>, // body x, y
    head: (usize, usize),
    head_char: char,
    body_char: char,
}

struct Board {
    // snake: Snake,
    width: usize,
    heigth: usize,
    padding: usize,
    board_vec: Vec<usize>,
    vert_wall_char: char,
    hor_wall_char: char,
}

struct Game {
    board: Board,
    snake: Snake,
    fruit: Fruit,
    over: bool,
}

fn main() {
    // let one_second = std::time::Duration::from_millis(100);
    let mut game: Game = init();

    while !game.over {
        update(&mut game);
        // draw(&game);
        draw2(&game);
        // std::thread::sleep(one_second);
    }
}

fn init() -> Game {
    let snake: Snake = Snake {
        head: (10, 10),
        body: vec![(9, 10), (8, 10), (7, 10), (6, 10)],
        head_char: '@',
        body_char: '%',
    };
    let board: Board = Board {
        // snake,
        width: 50,
        heigth: 30,
        padding: 5,
        board_vec: vec![0],
        vert_wall_char: '|',
        hor_wall_char: '~',
    };
    let mut fruits: Vec<(usize, usize, usize)> = (1..board.width)
        .flat_map(|x| (1..board.heigth).map(move |y| (x, y, y * board.width + x)))
        .collect();
    fruits.shuffle(&mut rand::rng());
    let fruit: Fruit = Fruit {
        current: 0,
        possibilities: fruits,
    };
    let game: Game = Game {
        board,
        snake,
        fruit,
        over: false,
    };
    game
}

fn update(game: &mut Game) {
    if let Some(key) = Keyboard::new().next() {
        let (nhp, movement) = match key {
            Keys::Up | Keys::Char('k') => (new_head_pos(&game.snake, Movements::Up), Movements::Up),
            Keys::Down | Keys::Char('j') => {
                (new_head_pos(&game.snake, Movements::Down), Movements::Down)
            }
            Keys::Left | Keys::Char('h') => {
                (new_head_pos(&game.snake, Movements::Left), Movements::Left)
            }
            Keys::Right | Keys::Char('l') => (
                new_head_pos(&game.snake, Movements::Right),
                Movements::Right,
            ),
            _ => ((0, 0), Movements::Invalid),
        };
        if movement != Movements::Invalid {
            if movement_to_body_direction(&game.snake, &movement) {
                return;
            }
            if snake_bit_itself(&game.snake, nhp) {
                println!("Morreu");
                game.over = true;
                return;
            }
            game.snake.body.insert(0, game.snake.head);
            game.snake.head = match movement {
                Movements::Up => (game.snake.head.0, game.snake.head.1 - 1),
                Movements::Down => (game.snake.head.0, game.snake.head.1 + 1),
                Movements::Left => (game.snake.head.0 - 1, game.snake.head.1),
                Movements::Right => (game.snake.head.0 + 1, game.snake.head.1),
                Movements::Invalid => todo!(),
            };
            if !snake_ate_fruit(game, nhp) {
                game.snake.body.pop();
            } else {
                fruit_new_position(game);
            }
        }
    }
}

#[derive(PartialEq)]
enum Movements {
    Invalid,
    Up,
    Down,
    Left,
    Right,
}

fn movement_to_body_direction(snake: &Snake, movement: &Movements) -> bool {
    snake.body[0]
        == match movement {
            Movements::Up => (snake.head.0, snake.head.1 - 1),
            Movements::Down => (snake.head.0, snake.head.1 + 1),
            Movements::Left => (snake.head.0 - 1, snake.head.1),
            Movements::Right => (snake.head.0 + 1, snake.head.1),
            Movements::Invalid => (0, 0),
        }
}

fn new_head_pos(snake: &Snake, mov: Movements) -> (usize, usize) {
    match mov {
        Movements::Invalid => (0, 0),
        Movements::Up => (snake.head.0, snake.head.1 - 1),
        Movements::Down => (snake.head.0, snake.head.1 + 1),
        Movements::Left => (snake.head.0 - 1, snake.head.1),
        Movements::Right => (snake.head.0 + 1, snake.head.1),
    }
}

fn snake_bit_itself(snake: &Snake, new_head_pos: (usize, usize)) -> bool {
    snake.body.contains(&new_head_pos)
}

fn snake_ate_fruit(game: &Game, (x, y): (usize, usize)) -> bool {
    get_current_fruit_pos(game) == (x, y)
}

fn get_current_fruit_pos(game: &Game) -> (usize, usize) {
    let current = game
        .fruit
        .possibilities
        .get(game.fruit.current)
        .expect("snake_ate_fruit current position should be within possibilities range.");
    (current.0, current.1)
}

fn fruit_new_position(game: &mut Game) {
    loop {
        if game.fruit.current >= game.fruit.possibilities.len() {
            game.fruit.current = 0;
        }
        if get_current_fruit_pos(game) == game.snake.head
            || game.snake.body.contains(&get_current_fruit_pos(game))
        {
            game.fruit.current += 1;
        } else {
            break;
        }
    }
}

fn draw(game: &Game) {
    // clearscreen::clear().expect("failed to clear screen");
    std::process::Command::new("clear").status().unwrap();

    // Print title bar
    println!("Snake Game\n");

    // Print top board wall
    println!(
        "{}{}",
        " ".repeat(game.board.padding + 1),
        game.board
            .hor_wall_char
            .to_string()
            .repeat(game.board.width)
    );

    // Print board
    for line in 0..game.board.heigth {
        // Find all the body points that are on the same Y as the current line
        let points_in_y: Vec<usize> = game
            .snake
            .body
            .iter()
            .filter_map(|&(x, y)| if y == line { Some(x) } else { None })
            .collect();
        let mut board_string = String::new();

        // Create the current line string, choosing between head, body or empty char
        for i in 0..game.board.width {
            if line == game.snake.head.1 && i == game.snake.head.0 {
                board_string.push(game.snake.head_char);
            } else if points_in_y.contains(&i) {
                board_string.push(game.snake.body_char);
            } else if line == get_current_fruit_pos(game).1 && i == get_current_fruit_pos(game).0 {
                board_string.push('*')
            } else {
                board_string.push(' ');
            }
        }

        // Print the current board line
        println!(
            "{}{}{}{}",
            " ".repeat(game.board.padding),
            game.board.vert_wall_char,
            board_string,
            game.board.vert_wall_char
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
        " ".repeat(game.board.padding),
        game.board
            .hor_wall_char
            .to_string()
            .repeat(game.board.width + 1)
    );
}

#[derive(Clone, Debug)]
enum Characteres {
    Board,
    SnakeBody,
    SnakeHead,
    Fruit,
    Blank,
}

fn character_compressor(c1: &Characteres, c2: &Characteres) -> char {
    match (c1, c2) {
        (Characteres::Board, Characteres::Board) => '|',
        (Characteres::Board, Characteres::SnakeBody) => '=',
        (Characteres::Board, Characteres::SnakeHead) => ';',
        (Characteres::Board, Characteres::Fruit) => '+',
        (Characteres::Board, Characteres::Blank) => '^',
        (Characteres::SnakeBody, Characteres::Board) => '6',
        (Characteres::SnakeBody, Characteres::SnakeBody) => '8',
        (Characteres::SnakeBody, Characteres::SnakeHead) => '!',
        (Characteres::SnakeBody, Characteres::Fruit) => ':',
        (Characteres::SnakeBody, Characteres::Blank) => '*',
        (Characteres::SnakeHead, Characteres::Board) => '2',
        (Characteres::SnakeHead, Characteres::SnakeBody) => '1',
        (Characteres::SnakeHead, Characteres::SnakeHead) => 'E',
        (Characteres::SnakeHead, Characteres::Fruit) => '?',
        (Characteres::SnakeHead, Characteres::Blank) => '"',
        (Characteres::Fruit, Characteres::Board) => 'L',
        (Characteres::Fruit, Characteres::SnakeBody) => '2',
        (Characteres::Fruit, Characteres::SnakeHead) => '3',
        (Characteres::Fruit, Characteres::Fruit) => 'E',
        (Characteres::Fruit, Characteres::Blank) => '"',
        (Characteres::Blank, Characteres::Board) => '_',
        (Characteres::Blank, Characteres::SnakeBody) => '~',
        (Characteres::Blank, Characteres::SnakeHead) => ',',
        (Characteres::Blank, Characteres::Fruit) => '.',
        (Characteres::Blank, Characteres::Blank) => ' ',
    }
}

fn draw2(game: &Game) {
    std::process::Command::new("clear").status().unwrap();

    // Print title bar
    println!("Snake Game\n");

    assert!(game.board.heigth % 2 == 0);

    let mut canva = create_board_canva(&game.board);
    insert_snake_canva(&mut canva, &game.snake, &game.board);
    insert_fruit_canva(&mut canva, &game.fruit);

    // println!("{:?}", canva);

    for line in (0..game.board.heigth).step_by(2) {
        for column in 0..game.board.width {
            let index = line * game.board.width + column;

            print!(
                "{}",
                character_compressor(&canva[index], &canva[index + game.board.width])
            );
        }
        println!();
    }
}

fn create_board_canva(board: &Board) -> Vec<Characteres> {
    let mut vec: Vec<Characteres> = vec![Characteres::Blank; board.width * board.heigth];

    for line in 0..board.heigth {
        for column in 0..board.width {
            let index = line * board.width + column;

            if line == 0 || line == board.heigth - 1 || column == 0 || column == board.width - 1 {
                vec[index] = Characteres::Board;
            }
        }
    }

    vec
}

fn position_to_index(board: &Board, (x, y): &(usize, usize)) -> usize {
    y * board.width + x
}

fn insert_snake_canva(canva: &mut [Characteres], snake: &Snake, board: &Board) {
    for item in &snake.body {
        canva[position_to_index(board, item)] = Characteres::SnakeBody;
    }
    canva[position_to_index(board, &snake.head)] = Characteres::SnakeHead;
}

fn insert_fruit_canva(canva: &mut [Characteres], fruit: &Fruit) {
    canva[fruit.possibilities[fruit.current].2] = Characteres::Fruit;
    // canva[position_to_index(&game.board, &get_current_fruit_pos(&game))] = Characteres::Fruit;
}
