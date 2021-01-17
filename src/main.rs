// use urubu::advanced::perft::Perft;
// use urubu::advanced::board::Board;
// use std::time::Instant;
// use urubu::types::board_move::BoardMove;
// use urubu::types::square::Square;
//
// fn main() {
//     println!("Hello, world!");
//     let mut board = Board::default();
//     let before_time = Instant::now();
//     let nodes = Perft::new().perft(&mut board, 7);
//     let after_time = Instant::now();
//     let dur = after_time.duration_since(before_time).as_millis() as u64;
//     println!("perft result {}", nodes);
//     println!("Time taken {} ms", dur);
//     println!("nps {}", (nodes * 1000 /dur));
// }
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut goto_x = x;
        let mut goto_y = y;
        let mut humans = Vec::<Point>::new();
        let mut zombies = Vec::<Point>::new();
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            humans.push(Point {x : human_x, y: human_y })
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);

            zombies.push(Point { x : zombie_xnext, y : zombie_ynext })
        }

        let mut goto = None;
        let mut best_distance = -1.0;
        for human in &humans {
            eprintln!("h {} {}", human.x, human.y);
            let mut current_distance = 100000.0;
            for zombie in &zombies {
                eprintln!("z {} {}", zombie.x, zombie.y);
                let zombie_distance = distance(human, zombie);
                eprintln!("zd {}", zombie_distance);
                if zombie_distance < current_distance {
                    current_distance = zombie_distance;
                }
            }
            eprintln!("bd {} {}", current_distance, best_distance);
            if best_distance < current_distance {
                goto = Some(human);
                best_distance = current_distance;
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{} {}", goto.unwrap().x, goto.unwrap().y); // Your destination coordinates
    }
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x).pow(2) as f32 + (p2.y - p1.y).pow(2) as f32).sqrt()
}

#[derive(Copy, Clone, Debug, Display)]
struct Point {
    x: i32,
    y: i32,
}
