use std::{rc::Rc, cell::RefCell};

use crate::utils;

use super::ant::Ant;

pub struct Colony {
    food_stock: Rc<RefCell<i32>>,
    tunnels: Rc<RefCell<Vec<String>>>,
    ants: Vec<Ant>,
    defense: Rc<RefCell<u32>>
}

impl Colony {
    pub fn new() -> Self {
        Self {
           food_stock: Rc::new(RefCell::new(100)),
           tunnels: Rc::new(RefCell::new(vec!["Main tunnel".to_string()])),
           ants: Vec::new(),
           defense: Rc::new(RefCell::new(0))
        }
    }

    pub fn add_ant(&mut self, ant: Ant) {
        self.ants.push(ant);
        *self.food_stock.borrow_mut() -= self.ants.len() as i32;
    }

    pub fn next_day(&self) {
        self.actions();
        self.verify_threat();
        self.consume_food();
    }
    
    fn actions(&self) {
        for ant in self.ants.iter() {
            ant.action(self.food_stock.clone(), self.tunnels.clone(), self.defense.clone());
        }
    }

    fn verify_threat(&self) {
        let add: u32 = (self.ants.len() + self.tunnels.borrow().len()) as u32; 
        let threat = utils::random_number(0, 1 + add);
        
        let defended = threat <= *self.defense.borrow();

        println!("Threat level: {}, Defense level: {}", threat, self.defense.borrow());

        if !defended {
            match utils::random_number(0, 2) {
                0 => {
                    *self.food_stock.borrow_mut() -= 5;
                },
                1 => {
                    if !self.tunnels.borrow().is_empty() {
                        self.tunnels.borrow_mut().remove(0);
                    }
                },
                2 => {
                    let percentage = utils::random_number(5, 90);
                    let decrease_amount = (*self.food_stock.borrow() as f64 * (percentage as f64 / 100.0)).round() as i32;
                    *self.food_stock.borrow_mut() -= decrease_amount;
                },
                _ => {},
            }
        } else {
            *self.defense.borrow_mut() -= threat;
        }

        println!("{}", if defended {"Defense holds, no damage"} else {"Threat exceeds defense, colony affected"});
    }

    fn consume_food(&self) {
        let consum = 2 * self.ants.len() as i32;
        *self.food_stock.borrow_mut() -= consum;

        println!("Daily food consumption: {}. Remaining food stock: {}", consum, self.food_stock.borrow());
    }

    pub fn display_state(&self) {
        println!("Coloy state:");
        println!("| Food stock: {}", self.food_stock.borrow());
        println!("| Tunnels: {:?}", self.tunnels.borrow());
        println!("| Defense: {}", self.defense.borrow());
    }

    pub fn list_ants(&self) {
        println!("Ants:");
        self.ants.iter().for_each(|n| n.display_info());
    }

    pub fn can_survive(&self) -> bool {
        *self.food_stock.borrow() > 0
    }

}