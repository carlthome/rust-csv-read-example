extern crate csv;

fn main() {
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
}
