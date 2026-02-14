//import libraries/modules
use std::io;
//use test_case::test_case;
use titlecase::{titlecase};

//for input validation
use input_validation::get_input;

//function for getting user input
fn get_user_input() -> String
{
    //Create a mutable string to store user input
    let mut user_input :String = get_input("Please enter your name (first, middle, and/or last) separated by spaces: ");
    //read the input from the user
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    //input validation
    //input_validation(&user_input);

    user_input
}

/*fn input_validation(user_input: &String)
{
    if !user_input.chars().all(char::is_alphabetic)
    {
        panic!("Please enter a valid alphabetic character.");
        loop
            get_user_input()
    }
}*/

//function to trim and separate the user input into different variables
/*fn fix_user_input<'a>(user_input: String) -> Vec<&'a str>
{
    user_input.trim().split_whitespace().collect::<Vec<&str>>()
}*/

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
    //loop {
    let user_input = get_user_input();

    //trim
    let names = user_input.trim().split_whitespace().collect::<Vec<&str>>();

    //call function that capitalizes the first letter in each name
    let titlecase = capitalize(names);

    //call function that reorders the names and prints the result to the console
    print_usernames(titlecase);
    //}
}

//creating tests and input validation
//Using the cfg attribute to declare test modules. This allows the build to exclude tests from the binary of the code.
#[cfg(test)]
//declare modules
mod tests {
    use super::*;
    //make discoverable by the cargo test cli tool
    #[test]
    fn capitalize_works() {
        //need super because we need our child module to reference our function in the main module
        //can declare super and import all function
        let vector_strings :Vec<&str> = vec!["is", "capital"];
        let result = capitalize(vector_strings);
        let compare = vec!["Is", "Capital"];
        assert_eq!(result, compare);
    }
    #[test]
    fn capitalize_no_work () {
        let vector_strings :Vec<&str> = vec!["is", "capital"];
        let result = super::capitalize(vector_strings);
        let compare = vec!["is", "capital"];
        assert_ne!(result, compare);
    }
}






//loop indefinitely until the user inputs correct input
/*loop {
    //Create a mutable string to store user input
    let mut user_input = String::new();

    //ask user for input
    println!("Please enter your name (first, middle, and/or last) separated by spaces: ");

    //read the input from the user
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //println!("Name is: {}", user_input);

    //first get user input, trim, and separate by whitespaces
    let names = user_input.trim().split_whitespace().collect::<Vec<&str>>();

    //input validation to ensure all characters are letters
    if !names[0].chars().all(char::is_alphabetic) || !names[1].chars().all(char::is_alphabetic) || !names[2].chars().all(char::is_alphabetic)
    {
        println!("All names must contain letters only! No numbers or special characters. Try again.");
    }
    else
    {
        //now get the length to ensure min and max are met
        let length = names.len();

        //input validation for number of arguments
        if length > 0 && length <= 3 {
            //here is my switch case!
            match names.len() {
                1 => println!("{}", titlecase(names[0])), //This is first name only
                2 => println!("{}, {}", titlecase(names[1]), titlecase(names[0])), //last, first
                3 => println!("{}, {} {}", titlecase(names[2]), titlecase(names[0]), titlecase(names[1])), //last, first middle
                _ => println!("ERROR"),
            }
            break;
        }
        else {
            println!("Please enter a minimum of 1 and a maximum of 3 names (first middle and/or last) separated by spaces. Try again.");
        }
    }
}*/


//Create a mutable string to store user input
/*let mut user_input = String::new();

//ask user for input
println!("Please enter your name (first, middle, and/or last) separated by spaces: ");

//read the input from the user
io::stdin().read_line(&mut user_input).expect("Failed to read line");
println!("Name is: {}", user_input);

//remove the newline character at the end of the input using trim
let trimmed_input = user_input.trim();

//now split the string by the spaces
/*
I could use the split(" ") function but split_whitespace seems like
a better option since it will remove any leading and trailing whitespace

This function returns a list of words
 */
let separated_name : Vec<&str> = trimmed_input.split_whitespace().collect::<Vec<&str>>();
/*
Purpose: Print Lastname, firstname middle
OR
last, first
OR
first
*/
//I will do this by size of the list (vector)
// Could probably do a switch statement, but I don't know that yet
if separated_name.len() == 3 {
    //this means that first, middle, and last names were all input
    println!("{}, {} {}", separated_name[2], separated_name[0], separated_name[1]);
}
else if separated_name.len() == 2 {
    //this means that first and last names were inputted
    println!("{}, {}", separated_name[1], separated_name[0]);
}
else if separated_name.len() == 1 {
    //this means that only the first name was inputted
    println!("{}", separated_name[0]);
}
else {
    //this is the case of when more than 3 names were input or nothing was input
    println!("Please enter a minimum of 1 and a maximum of 3 names (first middle and/or last) separated by spaces. Try again.");
    //ask again for user input
    //Maybe in the future, I will do a loop that says while out of bounds, keep asking for user input
    //io::stdin().read_line(&mut user_input).expect("Failed to read line");
}*/