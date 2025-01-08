use std::{rc::Rc, cell::RefCell};

use crate::{enums::role::Role, utils};

pub struct Ant {
    name:String,
    role:Role
}

impl Ant {
    pub fn new(name: String, role: Role) -> Self {
        Self {
            name,
            role
        }
    }

    pub fn action(&self, food_stock: Rc<RefCell<i32>>, tunnels: Rc<RefCell<Vec<String>>>, defense: Rc<RefCell<u32>>) {
        match self.role {
            Role::Forager => {
                let max = tunnels.borrow().len() as u32;
                *food_stock.borrow_mut() += utils::random_number(0, max) as i32;
            },
            Role::Builder => {
                let mut tunnels_list = tunnels.borrow_mut();
                tunnels_list.push(format!("{}'s Tunnel", self.name));
            },
            Role::Guard => {
                *defense.borrow_mut() += 1;
            }
        }
    }

    pub fn display_info(&self) {
        println!("| {} -> {}", self.name, self.role.to_string());
    }
}