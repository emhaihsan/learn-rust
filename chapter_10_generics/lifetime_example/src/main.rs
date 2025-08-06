fn main() {
    let string1 = String::from("long string is long");
    let result; // result dideklarasikan di scope luar

    { // Scope dalam dimulai
        let string2 = String::from("xyz"); // string2 dideklarasikan di scope dalam
        result = longest(string1.as_str(), string2.as_str());
    } // Scope dalam berakhir, string2 "mati" di sini

    // Kita mencoba menggunakan 'result' di sini, tapi jika 'result' meminjam dari 'string2',
    // maka 'string2' sudah tidak ada!
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}