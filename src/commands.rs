use crate::gamestate::GameState;
pub(crate) fn handle_command(command: &str, state: &mut GameState) {
    match command {
        "look" => look(state, "look"),
        "inventory" => show_inventory(state),
        "go north" | "go south" | "go east" | "go west" => go(state, command.split(" ").nth(1).unwrap()),
        "look north" | "look south" | "look east" | "look west" => look(state, command.split(" ").nth(1).unwrap()),
        "help" => help(),
        _ => println!("Unknown command")
    }
}

fn look(state: &GameState, direction: &str) {
    if direction == "look" {
        let location = &state.world.locations[state.player.current_location];
        println!("{}", location.description);
        for item in &location.items {
            println!("{}", item.name);
        }
        for npc in &location.npcs {
            println!("{}", npc.name);
        }
    }

    if direction == "north" || direction == "south" || direction == "east" || direction == "west" {
        let location = &state.world.locations[state.player.current_location];
        if let Some(exit) = location.exits.iter().find(|exit| exit.direction == direction) {
            // Grab the name of the location
            let destination = &state.world.locations[exit.destination];
            println!("{}", destination.name);
            for item in &destination.items {
                println!("{}", item.name);
            }
            for npc in &destination.npcs {
                println!("{}", npc.name);
            }
        } else {
            println!("There is no exit in that direction");
        }
    }
}

fn show_inventory(state: &GameState) {
    for item in &state.player.inventory {
        println!("{}", item.name);
    }
}

fn go(state: &mut GameState, direction: &str) {
    let location = &state.world.locations[state.player.current_location];
    if let Some(exit) = location.exits.iter().find(|exit| exit.direction == direction) {
        state.player.current_location = exit.destination;
        // tell the player where they are
        look(state, "look");
    } else {
        println!("There is no exit in that direction");
    }
}

fn help() {
    println!("Commands:");
    println!("  look");
    println!("  inventory");
    println!("  go <direction>");
    println!("  help");
}