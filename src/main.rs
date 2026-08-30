use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::Error;
use colored::Colorize;
use serde::Deserialize;
use std::io::BufReader;

use struct_iterable::Iterable;

use crate::simple_user_input::get_input;

#[derive(Debug, Clone, Copy)]
enum FolderTypes {
    Applications, // atalhos tem o tipo de extensão de arquivo .lnk  |  a maioria são de executáveis, então, acho que é seguro apostar nisso
    Code,
    Media,
    Images,
    Misc
}

type FilesClassificationElement = Vec<String>;

#[derive(Deserialize, Debug, Iterable)]
struct FilesExtensions {
    applications: FilesClassificationElement,
    code: FilesClassificationElement,
    images: FilesClassificationElement,
}

// fazer um algoritmo capaz de distuinguir o determinado uso de uma pasta analisando os conteúdos dentro dela.

fn main() -> std::io::Result<()> {
    print!("\n");
    let current_user = get_current_user()?;

    let mut binding: String = get_input("Please specify the directory where you wish to organize the files (Default is desktop)");
    if binding.len() <= 1 {
        binding = format!("C:\\Users\\{}\\Desktop", current_user);
    }
    let to_organize_dir = Path::new(&binding);

    let my_files = Path::new(&to_organize_dir);

    //fs::rename("a.txt", "moved_new_dir/b.txt")?;
    read_data_log(my_files);
    move_dir_files(my_files)?;

    println!("{}", get_current_user()?);
    println!("{:?}", get_file_extensions_json()?);
    println!("{:?}", get_file_classification(&String::from("png"), pack_file_extensions(get_file_extensions_json()?)));
    get_input("Press enter to end.");
    Ok(())
}

fn get_file_extensions_json() -> std::io::Result<FilesExtensions>{
    let file: fs::File = fs::File::open("src/FileExtensions.json")?;
    let reader = BufReader::new(file);
    let f: FilesExtensions = serde_json::from_reader(reader)?;
    Ok(f)
}

fn pack_file_extensions(f: FilesExtensions) -> HashMap<String, FolderTypes>{
    let mut new_thing: HashMap<String, FolderTypes> = HashMap::new();
    for (key, classification_e) in f.iter() {
        if let Some(n) = classification_e.downcast_ref::<FilesClassificationElement>() {
            for (_, v) in n.iter().enumerate() {
                let folder_type = match key {
                    "code" => FolderTypes::Code,
                    "images" => FolderTypes::Images,
                    "applications" => FolderTypes::Applications,
                    _ => FolderTypes::Misc, // Uma variante padrão para chaves desconhecidas
                };
                new_thing.insert(v.to_string(), folder_type);
            }
        }
    }
    new_thing
}

fn get_file_classification(extension: &String, packed_hashmap: HashMap<String, FolderTypes>) -> FolderTypes {
    packed_hashmap[&extension.clone()]
}

fn read_data_log(f: &Path) {
    if let Some(n) = f.file_name() {
        println!("Reading {} data...", n.to_str().unwrap());
    }
}

fn get_current_user() -> Result<String, Error> {
    match std::env::var("USERNAME") {
        Ok(username) => Ok(username),
        Err(e) => Err(Error::other(format!("Erro: {}", e)))
    }
}

fn move_dir_files(path: &Path) -> std::io::Result<()> {
    let corresponding_path: HashMap<&Path, FolderTypes> = HashMap::new();
    if let Ok(n) = fs::read_dir(path) {
        for input in n {
            let input = input?;
            let path = input.path();

            if path.is_file() {
                println!("Found : [{}]", path.display());
                //println!("Extension : .{:?}", path.extension().unwrap().to_str().unwrap())
            } else if path.is_dir() {
                println!("Found : [{}]", path.display());
                move_dir_files(&path)?;
            }
        }
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
