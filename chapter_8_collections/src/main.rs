use std::collections::HashMap;

fn main() {
    // 1. Membuat vector kosong
    let v_empty: Vec<i32> = Vec::new();
    println!("v_empty: {:?}", v_empty);

    // 2. Membuat vector dengan data
    let v_with_data = vec![1, 2, 3,4,5];
    println!("v_with_data: {:?}", v_with_data);

    // 3. menambahkan elemen ke vector
    let mut v_mutable = Vec::new();
    v_mutable.push(5);
    v_mutable.push(10);
    v_mutable.push(15);
    println!("v_mutable after push: {:?}", v_mutable);

    // 4. mengakses elemen vector
    let first_element = v_with_data[0];
    println!("first_element: {}", first_element);

    let second_element = &v_with_data[1];
    println!("second_element: {}", second_element);

    // akses menggunakan get() dan match
    match v_with_data.get(2) {
        Some(elemen) => println!("third_element: {}", elemen),
        None => println!("third_element: None"),
    }

    // 5. Mendemonstrasikan [] bisa panic dan get() mengembalikan Option
    // let does_not_exist_indexed = &v_initial[100]; // Ini akan panic jika di-uncomment
    // println!("This will panic: {}", does_not_exist_indexed);

    let does_not_exist_get = v_with_data.get(100);
    println!("does_not_exist_get: {:?}", does_not_exist_get);
    // let does_not_exist_indexed = &v_with_data[100];
    // println!("This will panic: {}", does_not_exist_indexed);

    // 6. Mendemonstrasikan aturan borrowing (ini akan menyebabkan error kompilasi jika di-uncomment)
    // let mut v_borrow = vec![1, 2, 3, 4, 5];
    // let first_borrow = &v_borrow[0];
    // v_borrow.push(6); // Error: cannot borrow `v_borrow` as mutable because it is also borrowed as immutable
    // println!("First element after borrow attempt: {}", first_borrow);

    // 7. Iterasi immutable
    println!("Iterasi immutable:");
    for elemen in &v_with_data {
        println!("{}", elemen);
    }

    // Iterasi mutable
    println!("Iterasi mutable and modify:");
    let mut v_iter_mut = vec![100,43,55];
    for i in &mut v_iter_mut {
        *i *= 2;
    }
    println!("{:?}", v_iter_mut);

    // 8. Menggunakan Enum untuk menyimpan tipe data berbeda
    #[derive(Debug)] // Untuk bisa mencetak enum dengan {:?}
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("\nVector with mixed types (using Enum): {:?}", row);

    // --- Bagian String ---
    println!("\n--- String Examples ---");

    // 1. membuat string baru
    let mut s_new = String::new();
    println!("Empty String: {}", s_new);

    let s_from_literal = "initial contents".to_string();
    println!("String from literal (to string): {}", s_from_literal);

    let s_from_from = String::from("another initial contents");
    println!("String from from: {}", s_from_from);

    // String dengan karakter UTF-8
    let hello_cyrillic = String::from("Здравствуйте");
    println!("String with Cyrillic: {:?}", hello_cyrillic);

    // 2. Mengubah (Update) String
    let mut s1_push_str_borrow = String::from("hello ");
    let s2_push_str_borrow = "world";
    s1_push_str_borrow.push_str(s2_push_str_borrow);
    println!("s1 after push_str with borrow: {}", s1_push_str_borrow);
    println!("s2 still valid: {}", s2_push_str_borrow);

    let mut s_push_char = String::from("coo");
    s_push_char.push('l');
    println!("String after push char: {:?}", s_push_char);

    // Menggabungkan String dengan + operator
    let s1_plus = String::from("Hello");
    let s2_plus = String::from("world");
    let s3_plus = s1_plus + &s2_plus;
    println!("String after + operator: {}", s3_plus);

    // Menggabungkan String dengan format! macro
    let s1_format = String::from("Hello");
    let s2_format = String::from("world");
    let s3_format = format!("{} {}", s1_format, s2_format);
    println!("String after format! macro: {}", s3_format);
    println!("s1 still valid: {}", s1_format);

    // 3. Akses Element String
    let s_cyrillic_slice = "Здравствуйте";
    // Hati-hati dengan slicing! Ini akan panic jika tidak pada batas karakter UTF-8
    // let s_invalid_slice = &s_cyrillic_slice[0..1]; // Ini akan panic!
    let s_valid_slice = &s_cyrillic_slice[0..4]; // Зд (2 karakter, 4 byte)
    println!("Valid slice of Cyrillic String (0..4 bytes): {:?}", s_valid_slice);

    // Iterasi menggunakan chars()
    println!("Iterasi Cyrillic String using chars():");
    for c in s_cyrillic_slice.chars() {
        println!("{}", c);
    }

    // Iterasi menggunakan bytes()
    println!("Iterasi Cyrillic String using bytes():");
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

    // --- Bagian HashMap ---
    println!("\n--- HashMap Examples ---");

    // 1. Membuat HashMap baru
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial Scores: {:?}", scores);

    // 2. Mengakses Nilai dalam HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {} is {}", team_name, score);

    // Iterasi HashMap
    println!("\nIterasi HashMap:");
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // 3. HashMap and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map_ownership = HashMap::new();
    map_ownership.insert(field_name, field_value);
    println!("Map ownership: {:?}", map_ownership);
    // field_name dan field_value tidak valid lagi di sini!
    // println!("field_name: {}", field_name); // Ini akan error kompilasi!

    // 4. Memperbarui HashMap
    // Menimpa nilai
    let mut scores_overwrite = HashMap::new();
    scores_overwrite.insert(String::from("Blue"), 10);
    println!("Scores overwrite: {:?}", scores_overwrite);
    scores_overwrite.insert(String::from("Yellow"), 50);
    println!("Scores overwrite: {:?}", scores_overwrite);

    // menambahkan hanya jika kunci belum ada(entry().or_insert())
    let mut scores_entry = HashMap::new();
    scores_entry.insert(String::from("Blue"), 10);
    println!("Scores before entry().or_insert(): {:?}", scores_entry);
    
    scores_entry.insert(String::from("Yellow"), 50);
    scores_entry.entry(String::from("Blue")).or_insert(25);
    println!("Scores after entry().or_insert(): {:?}", scores_entry);

    // memperbarui nilai berdasarkan nilai lama (word count example)
    let text = "hello world wonderful world";
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:#?}", word_counts);

}
