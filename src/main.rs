use crate::config::config::TextStorage;
use crate::config::loader::load_config;
use crate::markdown::conversions::{create_md_from_file, write_to_md};
use crate::scanner::checker::{get_file_write_loc, parse_file};
use std::env;
use std::path::Path;
use walkdir::WalkDir;
pub mod config;
pub mod hugo;
pub mod markdown;
pub mod scanner;

pub mod testing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config_path = Path::new(&args[1]);
    let input_path = Path::new(&args[2]);

    let config = load_config(config_path)?;

    for entry in WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let mut file_stor = TextStorage::new();
        parse_file(file_path, &config, &mut file_stor)?;

        let loc_to_write = get_file_write_loc(&config, file_path).unwrap_or_default();

        let mut output_dir = Path::new(&config.hugo_config.loc).to_path_buf();
        if !loc_to_write.is_empty() {
            output_dir.push(&loc_to_write);
        }

        let file_stem = file_path
            .file_stem()
            .ok_or_else(|| format!("Invalid file name: {:?}", file_path))?
            .to_str()
            .ok_or_else(|| format!("Non-UTF8 filename: {:?}", file_path))?;

        let mut file = create_md_from_file(&output_dir, file_stem)?;

        let res = write_to_md(&mut file, &file_stor, &config);

        if res.is_ok() {
            println!("Wrote: {}/{}", output_dir.display(), file_stem);
        }
    }

    Ok(())
}

//so the main would look something like this
/*
first collect args
get the path of the config file from args[1]

then load the config from that path
create a new store
NOW if the user has passed in a path that is a file then do the normal steps
BUT IF the user has passed a dir, iterate through it while saving its path for depth
so if there is a dir like src/conc/basics.rs, then the name would appear as

the str passed to the create_md_from_file function + the above till .
!!IMP now we need to extract everything till the . to get the complete pathname
then just pass it to function

!!FUTURE OPTIMISATION
if we have lots of dirs, we may divide tasks into thread and asyn them so you know noice
 */

// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let argc = args.len();
//
//     if argc < 2 {
//         println!("Not enough arguments!");
//         println!("Usage: cargo run -- first_arg");
//
//         return;
//     }
//
//     println!("Path of the file: {}", args[1]);
//
//     let file_path = &args[1];
//     let file = File::open(file_path);
//     let reader = BufReader::new(file.unwrap());
//
//
//     //printing the type of file that will be passed let extension :
//     let extension = file_path.split_once('.')
//         .map(|(_, val)| val.to_string())
//         .unwrap_or_default();
//
//     println!("File extension: {}", extension);
//
//     for line in reader.lines() {
//         let line = line.unwrap();
//         println!("{}", line);
//     }
//
//     //So what we will have is the user will define a toml
//     //parsingConfig.toml
//     //and it will have things like
//
//     // [[simple comment]]
//     // !/! (begin)
//     // !/! (end)
//
//     // [[warning]]
//     // !!! (begin)
//    // !!! (end)
//
//     //then the program will first read that and then construct a hashmap pairing the text inside the [[]] to a vec<struct>
//     // where the struct will contain 3 things for now
//     //1) start_str
//     //2) end_str
//     //3) line_num
//
//     //so start and end is just obvious and the line num will mark the start of that particular comment because there can be a lot of simple comments
//     //so this will hold the line_num of each of new comments
//     //ALSO: maybe we dont need to store the start_str and end_str because we already know if we have simple_comment it will start with a particular
//     //substr and also end with one so maybe we can store it somewhere common..?
//
//
// }
//

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let path = Path::new(&args[1]);
//
//     let config = crate::itr2::load_config(path).unwrap();
//     itr2::print_comments_types(&config).expect("TODO: panic message");
//
//
// }
