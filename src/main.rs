use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::Error;
use colored::Colorize;
use serde::Deserialize;
use std::io::BufReader;
use std::path::PathBuf;

use struct_iterable::Iterable;

use crate::simple_user_input::get_input;

#[derive(Debug, Clone, Copy)]
enum FolderTypes {
    Applications, // atalhos tem o tipo de extensão de arquivo .lnk  |  a maioria são de executáveis, então, acho que é seguro apostar nisso
    Code,
    Media,
    Images,
    Text,
    Misc
}

type FilesClassificationElement = Vec<String>;

#[derive(Deserialize, Debug, Iterable)]
struct FilesExtensions {
    applications: FilesClassificationElement,
    code: FilesClassificationElement,
    images: FilesClassificationElement,
    media: FilesClassificationElement,
    text: FilesClassificationElement
}

//TODO: fazer um algoritmo capaz de distuinguir o determinado uso de uma pasta analisando os conteúdos dentro dela.

fn main() -> std::io::Result<()> {
    print!("\n");
    let current_user = get_current_user()?;

    let mut binding: String = get_input("Please specify the directory where you wish to organize the files (Default is desktop)");
    if binding.len() <= 1 {
        binding = format!("C:\\Users\\{}\\Desktop", current_user);
    }
    let to_organize_dir = Path::new(&binding);

    read_data_log(to_organize_dir);
    let packed = pack_file_extensions(get_file_extensions_json()?);
    move_dir_files(to_organize_dir, &packed)?;

    get_input("Press enter to end.");
    Ok(())
}

fn get_directory(parent: &Path, name: &str) -> Result<PathBuf, Error> {
    let dir_path = parent.join(name);
    if !dir_path.exists() {
        fs::create_dir(&dir_path)?;
    }
    Ok(dir_path)
}

fn place_file_in_directory(file: &Path, directory: &Path, packed_hashmap: &HashMap<String, FolderTypes>) -> std::io::Result<()> {
    if !file.exists() {
        return Err(Error::other(format!("File not found: {}", file.display())));
    }

    let extension = file
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let classification = get_file_classification(extension, packed_hashmap);
    let classification_path = folder_types_to_string(classification);
    let dest_dir = get_directory(directory, classification_path)?;
    let dest = dest_dir.join(file.file_name().unwrap());

    fs::rename(file, dest)?;
    Ok(())
}



fn folder_types_to_string(n: FolderTypes) -> &'static str{
    match n {
        FolderTypes::Applications => &"Applications",
        FolderTypes::Code => &"Code",
        FolderTypes::Images => &"Images",
        FolderTypes::Media => &"Media",
        FolderTypes::Text => &"Text",
        FolderTypes::Misc => &"Misc",
    }
}

fn get_path_extension(path: &Path) -> String {
    path.extension().unwrap().to_str().unwrap().to_string()
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
                    "media" => FolderTypes::Media,
                    "text" => FolderTypes::Text,
                    _ => FolderTypes::Misc,
                };
                new_thing.insert(v.to_string(), folder_type);
            }
        }
    }
    new_thing
}

fn get_file_classification(extension: &str, packed_hashmap: &HashMap<String, FolderTypes>) -> FolderTypes {
    packed_hashmap
        .get(extension)
        .copied()
        .unwrap_or(FolderTypes::Misc)
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

fn move_dir_files(path: &Path, packed_hashmap: &HashMap<String, FolderTypes>) -> std::io::Result<()> {
    if let Ok(n) = fs::read_dir(path) {
        for input in n {
            let input = input?;
            let entry_path = input.path();

            if entry_path.is_file() {
                let name = entry_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.eq_ignore_ascii_case("desktop.ini") {
                    continue;
                }
                println!("Found : [{}]", entry_path.display());
                place_file_in_directory(&entry_path, path, packed_hashmap)?;
            } else if entry_path.is_dir() {
                println!("Found : [{}]", entry_path.display());
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
