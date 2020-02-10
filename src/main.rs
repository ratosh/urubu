use crate::input_handler::{handle_input};
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter some text: ");
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Error: Unable to read user input");
        input.trim_newline();
        if !handle_input(&input) {
            break;
        }
    }
}

pub trait Trim {
    fn trim_newline(&mut self);
}

impl Trim for String {
    
    fn trim_newline(&mut self) {
        while self.ends_with('\n') || self.ends_with('\r') {
            self.pop();
        }
    }
}

mod input_handler {
    use std::time::Instant;
    use urubu::abts::ab_search::SearchInfo;
    use urubu::simplified::perft::Perft;
    use urubu::simplified::game::Game;

    const PARAM_SEPARATOR : char = ' ';

    pub fn handle_input(input: &str) -> bool {
        let mut tokens = input.split(PARAM_SEPARATOR);
        match tokens.next() {
            Some("perf") =>  {
                let mut game = Game::default();
                let timer = Instant::now();
                let nodes = Perft::default().perft(&mut game, tokens.next().unwrap().parse().unwrap());
                let duration = timer.elapsed();
                let dur = duration.as_millis() as u64;
                println!("perft result {}", nodes);
                println!("Time taken {} ms", dur);
                println!("nps {}", (nodes * 1000 / dur));
                true
            },
            Some("go") => {
                let mut game = Game::default();
                let mut search_info = SearchInfo::default();
                let search_result = search_info.search(&mut game);
                println!("total moves {}", search_info.node_count);
                println!("search result {}", search_result.0.to_string());
                true
            },
            Some("quit") => {
                false
            },
            _ => {
                println!("Unknown command: {}", input);
                true
            }
        }
    }
}