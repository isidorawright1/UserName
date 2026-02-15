//import libraries/modules
use std::io;
//use test_case::test_case;
use titlecase::{titlecase};

//for input validation
use input_validation::get_input;

//function for getting user input
fn get_user_input() -> String
{
    loop {
        //Create a mutable string to store user input
        let mut user_input :String = get_input("Please enter your name (first, middle, and/or last) separated by spaces: ");
        //read the input from the user
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        //input validation
        if input_validation(&user_input)
        {
            return user_input
        }
        else {
            println!("Only alphabetic characters and spaces are allowed");
        }
    }
}

fn input_validation(user_input: &String) -> bool
{
    //panic!("Please enter a valid alphabetic character.");
    user_input.trim().chars().all(|c| c.is_alphabetic() || c == ' ')
}

//function that uses titlecase to capitalize the first letter in each name
fn capitalize(names: Vec<&str>) -> Vec<String>
{
    names.into_iter().map(|name| titlecase(name)).collect()
}

//call function that reorders the names in the vector
fn print_usernames(names: Vec<String>)
{
    //now get the length to ensure min and max are met
    let length = names.len();

    //input validation for number of arguments
    if length > 0 && length <= 3 {
        //here is my switch case!
        match names.len() {
            1 => println!("{}", names[0]), //This is first name only
            2 => println!("{}, {}", names[1], names[0]), //last, first
            3 => println!("{}, {} {}", names[2], names[0], names[1]), //last, first middle
            _ => println!("ERROR"),
        }
    }
    else {
        println!("Please enter a minimum of 1 and a maximum of 3 names (first middle and/or last) separated by spaces. Try again.");
    }
}

fn main() {
    //call the function to ask for user input
    let user_input = get_user_input();

    //trim
    let names = user_input.trim().split_whitespace().collect::<Vec<&str>>();

    //call function that capitalizes the first letter in each name
    let titlecase = capitalize(names);

    //call function that reorders the names and prints the result to the console
    print_usernames(titlecase);
}

//creating tests and input validation
//Using the cfg attribute to declare test modules. This allows the build to exclude tests from the binary of the code.
#[cfg(test)]
//declare modules
mod tests {
    use super::*;
    //make discoverable by the cargo test cli tool
    #[test]
    fn test_capitalize_works() {
        //need super because we need our child module to reference our function in the main module
        //can declare super and import all function
        let vector_strings :Vec<&str> = vec!["is", "capital"];
        let result = capitalize(vector_strings);
        let compare = vec!["Is", "Capital"];
        assert_eq!(result, compare);
    }
    #[test]
    fn test_capitalize_no_work () {
        let vector_strings :Vec<&str> = vec!["is", "capital"];
        let result = super::capitalize(vector_strings);
        let compare = vec!["is", "capital"];
        assert_ne!(result, compare);
    }
    #[test]
    fn test_input_validation_works() {
        let string_name = String::from("Isidora Wright");
        assert_eq!(input_validation(&string_name), true);
    }
    #[test]
    fn test_input_validation_no_work() {
        let string_name = String::from("Isidora Wr7ight");
        assert_ne!(input_validation(&string_name), true);
    }
}