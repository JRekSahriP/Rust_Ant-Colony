use rand::Rng;

use crate::{models::ant::Ant, enums::role::Role};

pub fn random_number(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

pub fn get_input() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
    .expect("Error to read input");

    input.trim().to_string()
}

pub fn get_input_number() -> u32 {
    match get_input().parse::<u32>() {
        Ok(num) => return num,
        Err(_) => return 0,
    }
}

pub fn create_ant() -> Ant {
    println!("Name:");
    let name = get_input();

    println!("Role:");
    println!("[1]: Forager");
    println!("[2]: Guard");
    println!("[3]: Builder ");
    
    let mut role:Option<Role> = None;
    while let None = role {
        match get_input_number() {
            1 => role = Some(Role::Forager),
            2 => role = Some(Role::Guard),
            3 => role = Some(Role::Builder),
            _ => {}
        };
    }

    
    Ant::new(name, role.unwrap())
}
