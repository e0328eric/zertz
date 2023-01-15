mod error;

use rustyline::Editor;

use zertz_core::board::{BoardKind, Marble};
use zertz_core::coordinate::Coordinate;
use zertz_core::game::{self, GameInputData};

const PROMPT: &str = ">> ";

fn main() -> error::Result<()> {
    let mut rl = Editor::<()>::new()?;

    let board_kind = loop {
        println!("What kind of board do you want?");
        println!("Possible boards: [37, 40, 43, 44, 48, 61]");
        let board_kind_str = rl.readline("Input the number: ")?;
        let Ok(board_kind_num) = board_kind_str.parse::<u8>() else {
            println!("This is not a number. Please input the number\n");
            continue;
        };
        if let Ok(kind) = BoardKind::try_from(board_kind_num) {
            break kind;
        } else {
            println!("This is not a valid board size.");
            continue;
        }
    };

    let mut zertz = game::Game::new(board_kind);

    'main: loop {
        match zertz.get_game_state() {
            game::GameState::CheckIsCatchable => {
                zertz.play(GameInputData::default())?;
            }
            game::GameState::PutMarble => {
                println!("{:?}", zertz);
                println!("Input the coordinate where you want to put the marble");
                let put_coord = get_coord(&mut rl)?;
                println!("Input the coordinate where you want to remove the ring");
                let remove_coord = get_coord(&mut rl)?;
                println!("Input the color of the marble");
                let marble = get_marble(&mut rl)?;

                if let Err(err) = zertz.play(GameInputData::put_marble_data(
                    put_coord,
                    remove_coord,
                    marble,
                )) {
                    println!("{}", err);
                    zertz.rewind();
                    continue 'main;
                }
            }
            game::GameState::CatchMarble => {
                println!("{:?}", zertz);
                let mut list = zertz.get_output().movable_list.expect("INTERNAL ERROR");

                print!("[");
                for (idx, catch_info) in list.iter().enumerate() {
                    print!(" ({idx}: {catch_info}) ");
                }
                println!("]");

                println!("Input the coordinate where you want to move it?");
                let mut idx = loop {
                    let tmp = rl.readline(PROMPT)?.trim().parse::<usize>().expect("idx??");
                    if tmp < list.len() {
                        break tmp;
                    }
                    println!("invalid input! (index is too large)");
                };

                while let Some(new_list) = {
                    if let Err(err) = zertz.play(GameInputData::catch_marble_data(list[idx])) {
                        println!("{}", err);
                        zertz.rewind();
                        continue 'main;
                    }
                    zertz.get_output().movable_list
                } {
                    list = new_list;
                    print!("[");
                    for (idx, catch_info) in list.iter().enumerate() {
                        print!(" ({idx}: {catch_info}) ");
                    }
                    println!("]");

                    println!("Input the coordinate where you want to move it?");
                    idx = loop {
                        let tmp = rl.readline(PROMPT)?.trim().parse::<usize>().expect("idx??");
                        if tmp < list.len() {
                            break tmp;
                        }
                        println!("invalid input! (index is too large)");
                    };
                }
            }
            game::GameState::GameEnd(winner) => {
                println!("The winner is {:?}", winner);
                break Ok(());
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
