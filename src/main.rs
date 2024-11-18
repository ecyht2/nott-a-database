//! Simple CLI to parse the raw data and store it into the database.

use nott_a_database::StudentResult;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Invalid Command Line Arguments".into());
    }

    let file = args
        .pop()
        .expect("There should be atleast 2 elements in args");

    let data = StudentResult::from_workbook(file)?;
    println!("{}", data.len());

    Ok(())
}
