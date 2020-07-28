pub struct Map {
    pub grid: [u8; 10000],
}

impl Map {
    pub fn new() -> Self {
        Self { grid: [12; 10000] }
    }
}
