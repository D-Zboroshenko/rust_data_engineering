use csv::{Writer, Reader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("Pineapple", 3.00),
    ];

    let mut wtr = Writer::from_path("output.csv")?;
    wtr.write_record(["Fruit", "Price"])?;
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?;
    }
    wtr.flush()?;

    let mut rdr = Reader::from_path("output.csv")?;
    let mut fruits = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let fruit = record.get(0).unwrap().to_string();
        let price = (record.get(1).unwrap().parse::<f32>().unwrap() * 0.9 * 1000.0).round() / 1000.0;
        fruits.push((fruit, price));
    }
    //println!("{:?}", fruits);

    let mut wtr = Writer::from_path("discounted_products.csv")?;
    wtr.write_record(["Fruit", "Discount Price"])?;
    for (fruit, price) in fruits.into_iter() {
        wtr.write_record([fruit, price.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}
