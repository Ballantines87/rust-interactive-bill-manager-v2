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





