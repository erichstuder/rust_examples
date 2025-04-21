//! This is a simple program beginning with the `main` function.
//!
//! To run the program type `cargo run` in the terminal.

fn main() {
    let mut text: String = String::from("Hello, world!");
    my_print_mut_ref(&mut text);
    my_print_mut_ref(&mut text);

    //run_tasks();
}

// This is a function.
// Note: No forward declaration is needed like for example in C.
// #[allow(unused)] suppresses the warning for unused function. Normally, this is not needed.
#[allow(unused)]
fn my_print(text: String) {
    println!("{}", text);
}

#[allow(unused)]
fn my_print_ref(text: &String) {
    //text.push_str("\n");
    println!("{}", text);
}

fn my_print_mut_ref(text: &mut String) {
    text.push_str("\n");
    println!("{}", text);
}
