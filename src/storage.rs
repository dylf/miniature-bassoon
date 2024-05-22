
pub struct Storage {}

impl Storage {
    pub fn new() -> Self {
        Storage {}
    }

    pub async fn save(&self, data: String) {
        println!("Saving data: {}", data);
    }
}
