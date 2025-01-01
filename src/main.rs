mod models;
mod enums;
mod utils;
mod commands;

use models::{colony::Colony, ant::Ant};
use enums::role::Role;


fn main() {
    let mut colony = Colony::new();

    colony.add_ant(Ant::new("Forager ant".to_string(), Role::Forager));
    colony.add_ant(Ant::new("Guard ant".to_string(), Role::Guard));
    colony.add_ant(Ant::new("Builder ant".to_string(), Role::Builder));

    
    while colony.can_survive() {
        commands::process(&mut colony);
    }

    println!("The food stock has been depleted, and the ants can no longer sustain themselves. The colony has perished.");
}
