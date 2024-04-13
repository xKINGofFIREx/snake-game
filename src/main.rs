use console::Term;
use rand::{self, Rng};
use std::{thread::sleep, time::Duration};

fn main() {
    let stdout = Term::buffered_stdout();

    let mut snake: Vec<Point> = Vec::new();
    snake.push(Point { x: 5, y: 5 });

    let mut apple: (usize, usize) = (8, 5);

    let mut dir = Direction::RIGHT;
    'game: loop {
        //function to clear console
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");

        //takes last element as head position in vector
        let head_pos = snake.len() - 1;

        //draws field with objects
        draw_field(&snake, apple, head_pos);

        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => dir = Direction::UP,
                's' => dir = Direction::DOWN,
                'a' => dir = Direction::LEFT,
                'd' => dir = Direction::RIGHT,
                _ => dir = Direction::UP,
            }
        }

        if snake[head_pos].x == apple.0 && snake[head_pos].y == apple.1 {
            snake.push(Point {
                x: apple.0,
                y: apple.1,
            });

            apple = gen_apple();
        } else {
            direction_move(&mut snake, head_pos, &dir);
            for (i, body) in snake.iter().enumerate() {
                if i != head_pos && snake[head_pos].x == body.x && snake[head_pos].y == body.y {
                    break 'game;
                }
            }
        }
    }
    println!("GAME OVER");
    sleep(Duration::from_millis(5000));
}

fn draw_field(snake: &Vec<Point>, apple: (usize, usize), head_pos: usize) {
    let line = [' '; 11];
    let mut field = [line; 11];

    field[apple.1][apple.0] = '*';

    for i in snake {
        field[i.y][i.x] = '#';
    }

    field[snake[head_pos].y][snake[head_pos].x] = '@';

    println!("_____________");
    for y in 0..11 {
        print!("|");
        for x in 0..11 {
            print!("{}", field[y][x]);
        }
        println!("|");
    }
    println!("_____________");
}

fn gen_apple() -> (usize, usize) {
    (
        rand::thread_rng().gen_range(0..11),
        rand::thread_rng().gen_range(0..11),
    )
}

fn direction_move(snake: &mut Vec<Point>, head_pos: usize, dir: &Direction) {
    match &dir {
        Direction::DOWN => {
            if snake[head_pos].y + 1 < 11 {
                snake.push(Point {
                    x: snake[head_pos].x,
                    y: snake[head_pos].y + 1,
                });

                snake.remove(0);
            } else {
                snake.push(Point {
                    x: snake[head_pos].x,
                    y: 0,
                });

                snake.remove(0);
            }
        }
        Direction::UP => {
            if snake[head_pos].y == 0 {
                snake.push(Point {
                    x: snake[head_pos].x,
                    y: 10,
                });

                snake.remove(0);
            } else if snake[head_pos].y - 1 > 0 {
                snake.push(Point {
                    x: snake[head_pos].x,
                    y: snake[head_pos].y - 1,
                });

                snake.remove(0);
            } else if snake[head_pos].y == 1 {
                snake.push(Point {
                    x: snake[head_pos].x,
                    y: 0,
                });

                snake.remove(0);
            }
        }
        Direction::RIGHT => {
            if snake[head_pos].x + 1 < 11 {
                snake.push(Point {
                    x: snake[head_pos].x + 1,
                    y: snake[head_pos].y,
                });

                snake.remove(0);
            } else {
                snake.push(Point {
                    x: 0,
                    y: snake[head_pos].y,
                });

                snake.remove(0);
            }
        }
        Direction::LEFT => {
            if snake[head_pos].x == 0 {
                snake.push(Point {
                    x: 10,
                    y: snake[head_pos].y,
                });

                snake.remove(0);
            } else if snake[head_pos].x - 1 > 0 {
                snake.push(Point {
                    x: snake[head_pos].x - 1,
                    y: snake[head_pos].y,
                });

                snake.remove(0);
            } else if snake[head_pos].x == 1 {
                snake.push(Point {
                    x: 0,
                    y: snake[head_pos].y,
                });
                snake.remove(0);
            }
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
