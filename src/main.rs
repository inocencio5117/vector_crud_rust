use std::io;
use std::vec;
use std::process;

fn read_input(variable: &mut String) {
    io::stdin()
       .read_line(variable)
       .expect("Error reading the value");
}

fn add_value(vector: &mut Vec<String>) {
    println!("Insert the value that you want to add");

    let mut value = String::new();

    read_input(&mut value);

    vector.push(value);
}

fn remove_value(vector: &mut Vec<String>) {
    let mut position = String::new();

    println!("What position do you want to change?");

    read_input(&mut position);
    
    let position: usize = match position.trim().parse() {
        Ok(num) => num,
        Err(_) => return 
    };

    vector.remove(position);
}

fn show_all(vector: &mut Vec<String>) {
    println!("{:?}", vector)
}

fn change_value(vector: &mut Vec<String>) {
    let mut value = String::new();
    let mut position = String::new();

    println!("What position do you want to change?");

    read_input(&mut position);
    
    println!("Insert the value:");

    read_input(&mut value);
    
    let position: usize = match position.trim().parse() {
        Ok(num) => num,
        Err(_) => return 
    };

    vector[position] = value;
}

fn exit_loop() {
    process::exit(0);
}

fn main() {
    println!("Welcome to Rust agenda!");

    let mut names: Vec<String> = vec![];

    loop {
        println!("");
        println!("To exit the program type 0");
        println!("To add a name type 1");
        println!("To delete a name type 2");
        println!("To change a name type 3");
        println!("To view all names type 4");

        let mut option = String::new();

        read_input(&mut option);

        
        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => exit_loop(),
            1 => add_value(&mut names),
            2 => remove_value(&mut names),
            3 => change_value(&mut names),
            4 => show_all(&mut names),
            _ => println!("There's something else than One"),
        }
    }

}


