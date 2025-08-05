fn main() {
    println!("--- Chapter 4: Understanding Ownership ---");

    // --- 1. Ownership: Scope and Data Interaction (Move, Clone, Copy) ---
    println!("\n--- 1. Ownership ---");

    // String Type (Heap-allocated data)
    let s1 = String::from("hello"); // s1 comes into scope
    println!("s1 initially: {}", s1);

    // Move: s1's value moves into s2, s1 is no longer valid
    let s2 = s1;
    // println!("s1 after move: {}", s1); // This would cause a compile-time error: borrow of moved value
    println!("s2 after move from s1: {}", s2);

    // Clone: Deep copy of heap data
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 is still valid
    println!("s3 after clone to s4: {}", s3);
    println!("s4 after clone from s3: {}", s4);

    // Integer Type (Stack-only data - Copy Trait)
    let x = 5; // x comes into scope
    let y = x; // y gets a copy of x, x is still valid (i32 implements Copy trait)
    println!("x: {}", x);
    println!("y: {}", y);

    // Ownership with functions
    let s_func_owner = String::from("function ownership"); // s_func_owner comes into scope
    takes_ownership(s_func_owner); // s_func_owner's value moves into the function, no longer valid here
    // println!("s_func_owner after function call: {}", s_func_owner); // Error!

    let x_func_copy = 10; // x_func_copy comes into scope
    makes_copy(x_func_copy); // x_func_copy is copied, still valid here
    println!("x_func_copy after function call: {}", x_func_copy);

    let s_return_owner = gives_ownership(); // gives_ownership moves its return value into s_return_owner
    println!("s_return_owner from function: {}", s_return_owner);

    let s_takes_gives = String::from("takes and gives back"); // s_takes_gives comes into scope
    let s_returned = takes_and_gives_back(s_takes_gives); // s_takes_gives moves in, return value moves out
    println!("s_returned from function: {}", s_returned);


    // --- 2. References and Borrowing ---
    println!("\n--- 2. References and Borrowing ---");

    // Immutable References (Borrowing)
    let s_ref = String::from("hello reference");
    let len = calculate_length(&s_ref); // s_ref is borrowed immutably
    println!("The length of '{}' is {}.", s_ref, len); // s_ref is still valid here

    // Attempting to modify with immutable reference (will not compile)
    // change_immutable(&s_ref); // Error: `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    // Mutable References
    let mut s_mut_ref = String::from("hello mutable reference");
    println!("Before change: {}", s_mut_ref);
    change_mutable(&mut s_mut_ref); // s_mut_ref is borrowed mutably
    println!("After change: {}", s_mut_ref); // s_mut_ref is still valid here

    // Restriction: One mutable reference at a time
    let mut s_one_mut = String::from("one mutable ref");
    let r1 = &mut s_one_mut;
    // let r2 = &mut s_one_mut; // This would cause a compile-time error: cannot borrow `s_one_mut` as mutable more than once at a time
    println!("Mutable reference r1: {}", r1); // r1's scope ends after its last use

    let r2 = &mut s_one_mut; // This is allowed because r1's scope ended
    println!("Mutable reference r2: {}", r2);


    // Restriction: No mutable reference while immutable references exist
    let mut s_mix_ref = String::from("mix references");
    let imm_r1 = &s_mix_ref; // Immutable borrow
    let imm_r2 = &s_mix_ref; // Another immutable borrow (allowed)
    println!("Immutable references: {} and {}", imm_r1, imm_r2); // Last use of imm_r1 and imm_r2

    // let mut_r = &mut s_mix_ref; // This would cause a compile-time error: cannot borrow `s_mix_ref` as mutable because it is also borrowed as immutable
    // println!("Mutable reference: {}", mut_r);

    let mut_r = &mut s_mix_ref; // Allowed now because imm_r1 and imm_r2 are out of scope
    println!("Mutable reference after immutable ones are out of scope: {}", mut_r);


    // Dangling References (will not compile if uncommented)
    // let reference_to_nothing = dangle(); // Error: missing lifetime specifier / cannot return reference to local variable
    let valid_string = no_dangle(); // Returns owned String, no problem
    println!("Valid string from no_dangle: {}", valid_string);


    // --- 3. Slices ---
    println!("\n--- 3. Slices ---");

    // String Slices
    let s_slice = String::from("hello world slice");
    let hello = &s_slice[0..5];
    let world = &s_slice[6..11];
    println!("Original string: {}", s_slice);
    println!("Slice 'hello': {}", hello);
    println!("Slice 'world': {}", world);

    // Shorthand for slices
    let full_slice = &s_slice[..]; // Entire string
    let start_slice = &s_slice[..5]; // From start to index 5 (exclusive)
    let end_slice = &s_slice[6..]; // From index 6 to end
    println!("Full slice: {}", full_slice);
    println!("Start slice: {}", start_slice);
    println!("End slice: {}", end_slice);

    // first_word function using string slices
    let my_string = String::from("programming rust");
    let word1 = first_word_slice(&my_string);
    println!("First word of '{}': {}", my_string, word1);

    let my_string_literal = "another example";
    let word2 = first_word_slice(my_string_literal); // Works with &str directly
    println!("First word of literal '{}': {}", my_string_literal, word2);

    // Demonstrating slice safety (will not compile if uncommented)
    let  s_slice_safety = String::from("safe slice");
    let word_safe = first_word_slice(&s_slice_safety);
    // s_slice_safety.clear(); // Error: cannot borrow `s_slice_safety` as mutable because it is also borrowed as immutable
    println!("Word from safe slice: {}", word_safe);


    // Other Slices (Array Slices)
    let a = [10, 20, 30, 40, 50];
    let array_slice = &a[1..4]; // Slice from index 1 to 4 (exclusive)
    println!("Original array: {:?}", a);
    println!("Array slice: {:?}", array_slice);
    assert_eq!(array_slice, &[20, 30, 40]);
} // End of main function scope. Variables go out of scope and are dropped.


// --- Functions for Ownership ---
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("Inside takes_ownership: {}", some_string);
} // some_string goes out of scope and `drop` is called.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Inside makes_copy: {}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}


// --- Functions for References and Borrowing ---
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope, but value it refers to is not dropped.

// fn change_immutable(some_string: &String) {
//     some_string.push_str(", world"); // This would cause a compile-time error
// }

fn change_mutable(some_string: &mut String) { // some_string is a mutable reference
    some_string.push_str(", world!");
}

// Function that would cause a dangling reference (will not compile)
// fn dangle() -> &String {
//     let s = String::from("dangling"); // s is created here
//     &s // We try to return a reference to s
// } // s goes out of scope here and is dropped. The reference would be invalid.

fn no_dangle() -> String {
    let s = String::from("no dangling");
    s // Ownership of s is moved out, so it's not dropped here
}


// --- Functions for Slices ---
// Function to find the first word using string slices
fn first_word_slice(s: &str) -> &str { // Accepts &str for flexibility
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // If no space found, return the whole string slice
}