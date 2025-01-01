use crate::{utils,models::colony::Colony};

pub fn process(colony: &mut Colony) {
    match utils::get_input().to_lowercase().as_str() {
        "ants list" | "ants" | "list" | "as" => colony.list_ants(),
        "ant new" | "new" | "an" => colony.add_ant(utils::create_ant()),
        "state" | "colony" | "s" => colony.display_state(), 
        "next" | "n" => colony.next_day(),
        "help" => help(),
        _ => {println!("Type [help] to see the list of available commands.");}
    }
}

fn help() {
    println!("Commands list:");
    println!("| {:<30} -> {}", "ants list | ants | list | as", "List all ants in the colony.");
    println!("| {:<30} -> {}", "ant new | new | an", "Add a new ant to the colony.");
    println!("| {:<30} -> {}", "state | colony | s", "Display the current state of the colony.");
    println!("| {:<30} -> {}", "next | n", "Progress to the next day.");
    println!("| {:<30} -> {}", "help", "Display this. :)");
    println!("| {:<30} -> {}", "quit | exit", "exit the program.");
}
