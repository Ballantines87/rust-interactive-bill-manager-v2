use std::{collections::HashMap};
use super::bill::*;
use super::menu::Bill;


pub enum MenuOptions {
    AddBill,
    ViewBills,
    RemoveBill,
    EditBill,
    ComputeTotal,
}

impl MenuOptions {
    pub fn from_str(string_input: &str) -> Option<MenuOptions> {
        //let number_input = string_input.trim().parse::<i32>().unwrap();
        //println!("{}",number_input);
        match string_input.trim() {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::EditBill),
            "5" => Some(Self::ComputeTotal),
            _ => None,
        }
    }

    pub fn show_menu() {
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Compute Bill");
        println!("");
        println!("Please input your choice:");
        
    }

    pub fn pick_service(menu_choice: MenuOptions, storage: &mut HashMap<i32, Bill>) {

        
        match menu_choice {
            MenuOptions::AddBill => {

                Bill::add_bill(storage)
            }
            MenuOptions::ViewBills => Bill::view_bills(storage),
            MenuOptions::RemoveBill => {

                Bill::view_bills(&storage);
                Bill::remove_bill(storage);
                Bill::view_bills(&storage);
            }
            MenuOptions::EditBill => {

                Bill::edit_bill(storage);
            }
            MenuOptions::ComputeTotal => Bill::calculate_bills_total(storage)
        }
    }
}