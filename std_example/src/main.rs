//! This is a simple program beginning with the `main` function.
//!
//! To run the program type `cargo run` in the terminal.

mod tasks;
mod enums;

fn main() {
    println!("\n*** rust examples ***\n");

    // *** move ***
    let text: String = String::from("Hello, world!");

    fn my_print(text: String) {
        println!("{}", text);
    }

    my_print(text);
    // my_print(text);


    // fn my_print_ref(text: &String) {
    //     // text.push_str("_123");
    //     println!("{}", text);
    // }

    // my_print_ref(&text);
    // my_print_ref(&text);


    // fn my_print_mut_ref(text: &mut String) {
    //     text.push_str("_123");
    //     println!("{}", text);
    // }

    // my_print_mut_ref(&mut text);
    // my_print_mut_ref(&mut text);


    // *** enums ***

    // example 1: Option
    // let option = enums::maybe_get_a_value(true);
    // if option.is_some() {
    //     println!("there is some with {}", option.unwrap());
    // }

    // if enums::maybe_get_a_value(false).is_none() {
    //     println!("there is none!")
    // }

    // example 2: Result
    // println!("{:?}", enums::check_value(-5));
    // println!("{:?}", enums::check_value(42));
    // println!("{:?}", enums::check_value(999));
    // println!("");

    // let result = enums::check_value(999);
    // if result.is_err() {
    //     let err = result.unwrap_err();

    //     if let enums::MyErrors::Temperature { temperature, too_high } = err {
    //         if too_high {
    //             println!("Temperature is too high!");
    //         }
    //         println!("Temperature: {} Kelvin", temperature);
    //     }
    // }

    // example 3: propagating errors with the `?` operator
    // No need to check explicitly for errors. The `?` operator will return the error if there is one.
    // fn check_and_propagate_error(value: i64) -> Result<(), enums::MyErrors> {
    //     let result = enums::check_value(value)?;
    //     println!("Result: {}", result);
    //     Ok(())
    // }

    // check_and_propagate_error(999).unwrap_or_else(|err| {
    //     println!("Error: {:?}", err);
    // });

    // *** tasks (optional) ***
    // tasks::run();

    // *** slices (optional) ***
    // let data = [1, 2, 3, 4, 5, 6];
    // // let data: [u8; 10];
    // print_slice(&data);
}


#[allow(unused)]
fn print_slice(slice: &[u8]) {
    for a in slice {
        println!("{:?}", a);
    }

    // Die länge von einem slice (array) ist bekannt! Whaaaaaaaat?!?!?
    // for n in 0..slice.len() {
    // for n in 0..100 {
    //     println!("{:?}", slice[n]);
    // }
}

#[allow(unused)]
fn get_number(reference: &mut u8) -> u8 {
    *reference = 33;
    return 42;
}

// #[allow(unused)]
// fn try_get() -> u8 {

// }
