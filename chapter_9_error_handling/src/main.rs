mod validation;

use std::io;
use std::error::Error;

fn get_valid_number_from_user() -> Result<validation::ValidNumber, Box<dyn Error>> {
    println!("Masukkan angka antara 1 dan 100:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // Baca input, sebarkan error jika ada

    let num: i32 = input.trim().parse()?; // Parse input, sebarkan error jika tidak bisa di-parse

    // Tangani Result dari ValidNumber::new
    // Jika Ok, kembalikan Ok(ValidNumber)
    // Jika Err, ubah String error menjadi Box<dyn Error> dan sebarkan
    Ok(validation::ValidNumber::new(num)?) // Gunakan operator '?' di sini
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        match get_valid_number_from_user() {
            Ok(valid_num) => {
                println!("Anda memasukkan angka: {}", valid_num.value());
                break; // Keluar dari loop jika input valid
            },
            Err(e) => {
                // Tangani berbagai jenis error yang mungkin terjadi
                if let Some(_parse_err) = e.downcast_ref::<std::num::ParseIntError>() {
                    eprintln!("Input tidak valid. Harap masukkan angka.");
                } else if let Some(io_err) = e.downcast_ref::<io::Error>() {
                    eprintln!("Terjadi kesalahan I/O: {}", io_err);
                }
                else {
                    eprintln!("Terjadi error: {}", e);
                }
                // Lanjutkan loop untuk meminta input lagi
            }
        }
    }

    println!("Terima kasih!");
    Ok(())
}