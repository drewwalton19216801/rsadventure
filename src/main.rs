mod gamestate;
mod player;
mod world;
mod commands;
mod location;
mod exit;
mod item;
mod npc;

use gamestate::GameState;
use commands::handle_command;
use std::io::{self, Write};
use crate::player::Player;
use crate::world::World;
use crate::location::Location;
use crate::exit::Exit;

fn main() {
    let mut game_state = initialize_game();

    println!("Welcome to rsAdventure!");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        command = command.trim().to_string();

        if command == "quit" {
            break;
        }

        handle_command(&command, &mut game_state);
    }
}

fn initialize_game() -> GameState {
    // Initialize the game state with player, world, etc.
    let player = Player {
        name: String::from("Hero"),
        inventory: Vec::new(),
        current_location: 0
    };

    let world = World {
        locations: vec![
            Location {
                name: "Start".to_string(),
                description: "You are at the start of your adventure.".to_string(),
                items: vec![],
                npcs: vec![],
                exits: vec![
                    Exit {
                        direction: "north".to_string(),
                        destination: 1,
                    },
                ],
            },
            Location {
                name: "Forest".to_string(),
                description: "You are in a dark forest.".to_string(),
                items: vec![],
                npcs: vec![],
                exits: vec![
                    Exit {
                        direction: "south".to_string(),
                        destination: 0,
                    },
                ],
            },
        ],
    };

    GameState { player, world }
}