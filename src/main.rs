use std::env;
use std::path::Path;
use crate::config::config::TextStorage;
use crate::config::loader::load_config;
use crate::markdown::conversions::{create_md, to_md};
use crate::scanner::checker::parse_file;

pub mod config;
pub mod scanner;
pub mod markdown;
pub mod hugo;

pub mod testing;


fn main(){
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let config = load_config(path).unwrap();
    let mut stor = TextStorage::new();
    let file_path = Path::new(&args[2]);
    let result = parse_file(&file_path, &config, &mut stor);


    stor.print();

    let file =create_md();
    let res = to_md(&mut file.unwrap(), &stor );

    if let Some(err) = result.err() {
        eprintln!("{}", err);
    }

    println!("YO WE DONE!");

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
}


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












