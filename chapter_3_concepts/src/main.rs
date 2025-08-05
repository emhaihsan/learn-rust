fn main() {
    println!("--- Chapter 3 Concepts ---");

    // Functions
    println!("\n--- Functions ---");
    another_function();
    function_with_parameter(5);
    function_with_multiple_parameters(10, 'm');
    let five_result = five();
    println!("The value from five() is: {five_result}");
    let plus_one_result = plus_one(9);
    println!("The value from plus_one(9) is: {plus_one_result}");

    // Comments
    println!("\n--- Comments ---");
    // This is a single-line comment
    let comment_example = 10; // Comment at the end of a line
    println!("Comment example variable: {comment_example}");

    // Control Flow - if Expressions
    println!("\n--- Control Flow: if Expressions ---");
    let number_if = 7;
    if number_if < 5 {
        println!("Condition was true (number_if < 5)");
    } else {
        println!("Condition was false (number_if < 5)");
    }

    let number_divisible = 6;
    if number_divisible % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_divisible % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_divisible % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition_let = true;
    let number_from_if = if condition_let { 5 } else { 6 };
    println!("The value of number_from_if is: {number_from_if}");

    // Control Flow - Loops
    println!("\n--- Control Flow: Loops ---");
    // loop keyword
    let mut counter_loop = 0;
    let result_loop = loop {
        counter_loop += 1;
        if counter_loop == 3 {
            break counter_loop * 10;
        }
    };
    println!("Result from loop: {result_loop}");

    // while loop
    let mut number_while = 3;
    while number_while != 0 {
        println!("{}!", number_while);
        number_while -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop with array
    let a_for = [10, 20, 30, 40, 50];
    println!("Looping through array with for:");
    for element in a_for {
        println!("the value is: {element}");
    }

    // for loop with range and .rev()
    println!("Countdown with for and .rev():");
    for number_for in (1..4).rev() {
        println!("{}!", number_for);
    }
    println!("LIFTOFF!!!");
}

// --- Functions Examples ---

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn function_with_multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
