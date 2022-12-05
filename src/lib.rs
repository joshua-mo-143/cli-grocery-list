use csv::{Reader, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::OpenOptions, path::Path};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Item {
    pub name: String,
    pub quantity: String
}


pub fn check_source_exists() -> Result<(), Box<dyn Error>> {
    let path: &Path = Path::new("grocerylist.csv");
    let file_exists = path.exists();

    if !file_exists {
        println!("Source file doesn't exist! Creating file...");
        OpenOptions::new()
            .create_new(true)
            .open(path)
            .map_err(|e| e.to_string())
            .ok();

        println!("Source file has been created.");
    }

    Ok(())
}

pub fn add_item(quantity: &i32, item: &String) -> Result<(), Box<dyn Error>> {
    check_source_exists().ok();
    let file = Path::new("grocerylist.csv");
    let items = read_csv_data().unwrap();

    if check_if_item_exists(item.to_lowercase().to_owned()) == true {
        panic!("That item already exists!");
    };


    let mut wtr = WriterBuilder::new().has_headers(true).from_path(file)?;

    wtr.write_record(&["name", "quantity"])
        .map_err(|e| e.to_string())
        .ok();

        wtr.serialize([&item, &quantity.to_string()]).ok();
    
    for line in items {
        wtr.serialize([line.name, line.quantity.to_string()]);
    }

    wtr.flush().ok();

    println!("Successfully added {:?}x {:?}", quantity, item);
    Ok(())
}


pub fn read_csv_data() -> Result<Vec<Item>, Box<dyn Error>> {
    let file = Path::new("grocerylist.csv");
    let mut data: Vec<Item> = Vec::new();
    let mut rdr = Reader::from_path(file)?;

    for result in rdr.deserialize() {
        let record: Item = result?;
        data.push(record);
    }
    Ok(data)
}

pub fn check_if_item_exists(item: String) -> bool {
    check_source_exists().ok();
    let data = read_csv_data().unwrap();

    if data.iter().any(|i| &i.name == &item) {
        return true;
    } else {
        return false;
    }
}

pub fn delete_item(item: String) {
    check_source_exists().ok();
    let data = read_csv_data().unwrap();
    if check_if_item_exists(item.to_owned()) == false {
        panic!("That item doesn't exist!");
    };
}