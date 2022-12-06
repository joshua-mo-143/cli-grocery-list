use csv::{WriterBuilder};
use std::{error::Error, path::Path};

mod utils;
use utils::lib::{read_csv_data, check_source_ok,
                Item};

pub fn add_item(quantity: &i32, item: &String, price: f32) -> Result<(), Box<dyn Error>> {
    check_source_ok();
    let path = Path::new("grocerylist.csv");
    let items = read_csv_data().unwrap();

    if check_if_item_exists(item.to_lowercase().to_owned()) == true {
        panic!("That item already exists!");
    };


    let mut wtr = WriterBuilder::new().has_headers(true).from_path(path)?;

    wtr.write_record(&["name", "quantity", "price"])
        .map_err(|e| e.to_string())
        .ok();

        wtr.serialize([&item, &quantity.to_string(), &price.to_string()]).ok();
    
    for line in items {
        wtr.serialize([line.name, line.quantity.to_string(), line.price.to_string()]).ok();
    }

    wtr.flush().ok();

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

pub fn delete_item(item: String) -> Result<String, Box <dyn Error>> {
    let path: &Path = Path::new("grocerylist.csv");
    check_source_ok();
    let data = read_csv_data().unwrap();
    if check_if_item_exists(item.to_owned()) == false {
        panic!("That item doesn't exist!");
    };

    let output = data.iter().filter(|list_item| *list_item.name != item).collect::<Vec<&Item>>();

    let mut wtr = WriterBuilder::new().has_headers(true).from_path(path)?;

    wtr.write_record(&["name", "quantity"])
        .map_err(|e| e.to_string())
        .ok();
    
    for line in output {
        wtr.serialize([&line.name, &line.quantity.to_string()]).ok();
    }

    wtr.flush().ok();

    Ok("Deleted item.".to_string())
}

pub fn update_item(search: String, new_quantity: i32) -> Result<(), Box<dyn Error>> {
    let path = Path::new("grocerylist.csv");
    if check_if_item_exists(search.clone()) == false {
        panic!("That item doesn't exist!")
    } else {

    let csv = read_csv_data().unwrap();

    let mut wtr = WriterBuilder::new().has_headers(true).from_path(path)?;

    wtr.write_record(&["name", "quantity", "price"])
    .map_err(|e| e.to_string())
    .ok();


    for line in csv {
        if line.name == search {
            wtr.serialize([line.name, new_quantity.to_string(), line.price.to_string()]).ok();
        } else {
            wtr.serialize([line.name, line.quantity.to_string(), line.price.to_string()]).ok();
        }
    }

    wtr.flush().ok();


    Ok(())
    }
}