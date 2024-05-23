use std::error::Error;

pub async fn save_value(data: String) -> Result<(), Box<dyn Error>> {
    println!("Saving data: {}", data);
    Ok(())
}
