pub struct ValidNumber {
    value: i32,
}

impl ValidNumber {
    // Konstruktor yang sekarang mengembalikan Result
    pub fn new(value: i32) -> Result<ValidNumber, Box<dyn std::error::Error>> {
        if value < 1 || value > 100 {
            // Mengembalikan Err dengan pesan error
            Err(format!("Nilai harus antara 1 dan 100, Anda memasukkan {}.", value).into())
        } else {
            // Mengembalikan Ok jika nilai valid
            Ok(ValidNumber { value })
        }
    }

    // Getter untuk mendapatkan nilai
    pub fn value(&self) -> i32 {
        self.value
    }
}