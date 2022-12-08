use std::{error::Error};

mod utils;
use utils::lib::{check_source_ok, read_csv_data, Item};

use crate::utils::lib::write_to_csv;

pub fn add_item(quantity: &i32, item: &String, price: f32) -> Result<(), Box<dyn Error>> {
    check_source_ok();

    if check_if_item_exists(item.to_lowercase().to_owned()) == true {
        panic!("That item already exists!");
    };

    let new_record = Item {
        name: item.to_owned(),
        quantity: quantity.to_string(),
        price: f64::from(price),
    };

    let mut data = read_csv_data().unwrap();
    data.push(new_record);

    write_to_csv(data).ok();

    println!("Successfully added {:?}x {:?}", quantity, item);
    Ok(())
}

pub fn check_if_item_exists(item: String) -> bool {
    check_source_ok();
    let data = read_csv_data().unwrap();

    if data.iter().any(|i| &i.name == &item) {
        return true;
    } else {
        return false;
    }
}

pub fn delete_item(item: String) -> Result<String, Box<dyn Error>> {
    check_source_ok();

    let mut data = read_csv_data().unwrap();
    if check_if_item_exists(item.to_owned()) == false {
        panic!("That item doesn't exist!");
    };

    data.retain(|x| *x.name != item);

    write_to_csv(data).ok();

    Ok("Deleted item.".to_string())
}

pub fn update_item(search: String, new_quantity: i32) -> Result<(), Box<dyn Error>> {
    if check_if_item_exists(search.clone()) == false {
        panic!("That item doesn't exist!")
    };

    let mut data = read_csv_data().unwrap();

    for item in data.iter_mut() {
        if item.name == search {
            item.quantity = new_quantity.to_string();
        }
    }

    write_to_csv(data).ok();

    Ok(())
}
