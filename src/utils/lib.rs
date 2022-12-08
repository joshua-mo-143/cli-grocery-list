use std::{error::Error, fs::OpenOptions, path::Path};
use serde::{Deserialize, Serialize};
use csv::{Reader, WriterBuilder};

#[derive(Deserialize, Serialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct Item {
    pub name: String,
    pub quantity: String,
    pub price: f64
}

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
pub struct Data {
    pub items: Vec<Item>
}

pub trait Info {
    fn get_total_qty(&self) -> String;
    fn get_total_cost(&self) -> String;
    fn sort(&mut self);
}

impl Info for Data {
    fn get_total_qty(&self) -> String {
        let total_entries = &self.items.len();

        let mut total_items: i32 = 0;
        for item in &self.items {
            total_items += item.quantity.parse::<i32>().unwrap();
        }

        return format!("You currently have {} entries in your shopping list and {} items in total left to buy.", 
                    total_entries, total_items);
    }

    fn get_total_cost(&self) -> String {

        let mut total_cost: f64 = 0.00;
        for item in &self.items {
            total_cost += item.price * item.quantity.parse::<f64>().unwrap();
        }

        return format!("Your current total is: £{}", total_cost);
    }

    fn sort(&mut self) {
        self.items.sort_by(|a, b| a.name.cmp(&b.name));
        println!("Sorted items.")
    }
}

pub fn check_source_ok() {
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
}

pub fn read_csv_data() -> Result<Vec<Item>, Box<dyn Error>> {
    check_source_ok();

    let file = Path::new("grocerylist.csv");
    let mut data: Vec<Item> = Vec::new();
    let mut rdr = Reader::from_path(file)?;

    for result in rdr.deserialize() {
        let record: Item = result?;
        data.push(record);
    }
    Ok(data)
}

pub fn write_to_csv(data: Vec<Item>) -> Result<(), Box<dyn Error>>{
    let path: &Path = Path::new("grocerylist.csv");

    let mut wtr = WriterBuilder::new().from_path(path)?;

    for item in data {
        wtr.serialize(Item {
            name: item.name,
            quantity: item.quantity,
            price: format!("{:.2}", item.price).parse::<f64>().unwrap()
        }).map_err(|e| e.to_string()).ok();
    }

    wtr.flush().ok();
    
    Ok(())
}

pub fn init_data() -> Data {
    let items = read_csv_data().unwrap();

    let data = Data {
        items: items
    };

    return data;
}

pub fn init_file() -> String {
    std::fs::File::create("grocerylist.csv").ok();
    
    return format!("Source file has been created.");
}   