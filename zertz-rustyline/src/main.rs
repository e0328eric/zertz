mod error;

use rustyline::Editor;

use zertz_core::board::Marble;
use zertz_core::coordinate::Coordinate;
use zertz_core::game;

const PROMPT: &str = ">> ";

fn main() -> error::Result<()> {
    let mut rl = Editor::<()>::new()?;
    let mut zertz = game::Game::new();

    loop {
        println!("{:?}", zertz);

        match zertz.give_main_state() {
            game::GameAction::PutMarble => {
                println!("Input the coordinate where you want to put the marble");
                let put_coord = get_coord(&mut rl)?;
                println!("Input the coordinate where you want to remove the ring");
                let remove_coord = get_coord(&mut rl)?;
                println!("Input the color of the marble");
                let marble = get_marble(&mut rl)?;

                zertz.put_marble(put_coord, remove_coord, marble)?;
            }
            game::GameAction::CatchMarble(mut list) => {
                print!("[");
                for (idx, catch_info) in list.iter().enumerate() {
                    print!(" ({idx}: {catch_info}) ");
                }
                println!("]");

                println!("Input the coordinate where you want to move it?");
                let mut idx = rl.readline(PROMPT)?.trim().parse::<usize>().expect("idx??");

                while let Some(new_list) = zertz.catch_marble(list[idx])? {
                    list = new_list;
                    print!("[");
                    for (idx, catch_info) in list.iter().enumerate() {
                        print!(" ({idx}: {catch_info}) ");
                    }
                    println!("]");

                    println!("Input the coordinate where you want to move it?");
                    idx = rl.readline(PROMPT)?.trim().parse::<usize>().expect("idx??");
                }
            }
        }
    }
}

fn get_coord(rl: &mut Editor<()>) -> error::Result<Coordinate> {
    loop {
        let string = rl.readline(PROMPT)?;
        if let Some(coord) = parse_coord(string.trim()) {
            break Ok(coord);
        }
        println!("Wrong input found. Please write appropriately.");
    }
}

fn parse_coord(string: &str) -> Option<Coordinate> {
    let (_, right) = string.split_once('(')?;
    let (left, _) = right.split_once(')')?;
    let (row, col) = left.split_once(',')?;

    Some(Coordinate::new(
        row.trim().parse().unwrap(),
        col.trim().parse().unwrap(),
    ))
}

fn get_marble(rl: &mut Editor<()>) -> error::Result<Marble> {
    loop {
        let string = rl.readline(PROMPT)?;
        if let Some(marble) = parse_marble(string.trim()) {
            break Ok(marble);
        }
        println!("Wrong input found. Please write appropriately.");
    }
}

fn parse_marble(string: &str) -> Option<Marble> {
    match string.trim() {
        "white" | "White" | "WHITE" => Some(Marble::White),
        "gray" | "Gray" | "GRAY" => Some(Marble::Gray),
        "black" | "Black" | "BLACK" => Some(Marble::Black),
        _ => None,
    }
}
