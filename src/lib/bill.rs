
use std::collections::HashMap;

use super::utils::*;



#[derive(Debug)]
pub struct Bill {
    pub exp_name: String,
    pub amount: f32,
}

impl Bill {
    pub fn new() -> Bill {
        Self {
            exp_name: String::from(""),
            amount: 0.0,
        }
    }

    pub fn add_bill(bills_archive: &mut HashMap<i32, Bill>) {
        let index = (bills_archive.len() + 1) as i32;

        let mut bill = Bill::new();
        println!("Please assign a name to your expense:");
        bill.exp_name = get_user_input().unwrap();
        println!("Please write the amount:");
        let amount_str_buf = get_user_input().unwrap();
        bill.amount = amount_str_buf.trim().parse::<f32>().unwrap();


        bills_archive.insert(index, bill);
        println!("\nBill added!\n");
        Bill::view_bills(bills_archive);
    }

    pub fn view_bills(bills_archive: &HashMap<i32, Bill>) {
        println!("\nHere's the current list of your bills:\n");
        for (i, j) in bills_archive.iter() {
            println!("Bill ID: {} -- Expense Name: {:?} -- Amount: {:?}\n", i, j.exp_name, j.amount);
        }   

    }

    pub fn remove_bill(storage: &mut HashMap<i32, Bill>) {
        
        println!("\nPlease input the ID of the bill you want to remove:");
        let id = get_user_input().unwrap().trim().parse::<i32>().unwrap();

        if storage.contains_key(&id) {

            storage.remove(&id);

        } else {

            println!("non-existent id!")
        }
    }

    pub fn edit_bill(storage: &mut HashMap<i32, Bill>) {
        
        println!("\nPlease input the ID of the bill you want to update:");
        let id = get_user_input().unwrap().trim().parse::<i32>().unwrap();

        if storage.contains_key(&id) {

            storage.remove(&id);

            let mut bill = Bill::new();
            
            println!("Please assign a NEW name to your expense:");
            bill.exp_name = get_user_input().unwrap();

            println!("\nPlease write the NEW amount:");
            let amount_str_buf = get_user_input();
            bill.amount = amount_str_buf.unwrap().trim().parse::<f32>().unwrap();

            storage.insert(id, bill);
            Bill::view_bills(&storage);

        } else {
            println!("\nnon-existent id!")
        }

    }

    pub fn calculate_bills_total(storage: &mut HashMap<i32, Bill>) {
        let mut running_sum = 0.0;
        for (_i, j) in storage.iter() {
            running_sum += j.amount;
        }

        println!("\nThe TOTAL EXPENSES so far amount to {:?}", running_sum);
    }



}