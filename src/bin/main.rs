// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.


use std::{collections::HashMap};

use managerlib::{bill::Bill, menu::MenuOptions, utils::get_user_input};







fn main() {
    let mut hash_map: HashMap<i32, Bill> = HashMap::new();
    loop {
        MenuOptions::show_menu();
        let option_input = get_user_input();
        if option_input.is_none() {
            println!("Wrong input! Please pick a number 1 - 5");
            continue;
        }
        let choice = MenuOptions::from_str(option_input.unwrap().as_str());
        if choice.is_none() {
            println!("Wrong choice!");
            continue;
        }

        MenuOptions::pick_service(choice.unwrap(), &mut hash_map);

        println!("Do you wish to perform another operation?Y/N");
        match get_user_input() {
            Some(var) => {
                if var.to_lowercase() == "y" {
                    continue;
                }
                else {
                    break;
                }

            }
            None => break
        }

    }
}





