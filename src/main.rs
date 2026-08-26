use std::fs;
use std::path::Path;
use std::io::Error;
use colored::Colorize;

use crate::simple_user_input::get_input;

fn main() -> std::io::Result<()> {
    println!("{}", "WARNING : This application works by deleting and copying files into a new place.".yellow());

    let binding = get_input("Please specify the directory where you wish to organize the files.");
    let to_organize_dir = Path::new(&binding);

    let my_files = Path::new(&to_organize_dir);

    if let Ok(_) = create_dir(my_files) {
        println!("Sucess!")
    }

    Ok(())
}

fn create_dir(path: &Path) -> Result<Result<(), Error>, String>{
    let dir_path = Path::new(path);
    let dir = fs::create_dir(dir_path);

    match dir {
        Ok(_) => {
            let file_name = dir_path.file_name().unwrap().to_str().unwrap();
            println!("Directory [{}] created!", file_name.green());
            Ok(dir)
        }
        Err(_) => {
            Err("Error creating directory, it probably already exists.".to_string())
        }
    }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}