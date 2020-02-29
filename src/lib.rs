#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Mutex;



lazy_static! {
    static ref ORIGIN_VEC : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the origin langage
    static ref DEST_VEC : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the destination language (e.g fr, it, de...)
}

fn create_path(source: String) -> String {
    let mut new_path = String::from("./lang/.txt");
    new_path.insert_str(7, source.as_str());
    return new_path;
}

//check if file exist true if exist
fn check_file(file_lang: &str) {
    match File::open(create_path(file_lang.to_string())) {
        Ok(_) => {}
        Err(e) => {
            panic!(
                "could open the {} file at path {}, error {}",
                file_lang,
                create_path(file_lang.to_string()),
                e
            );
        }
    }

    match File::open(create_path("origin".to_string())) {
        Ok(_) => {}
        Err(e) => {
            panic!("couldn't open the origin file in ./lang, error {}", e);
        }
    }
}

//load the file into the vec line by line
fn load_files(text: &str) {
    //open the origin langage file

    let path = create_path("origin".to_string());
    {
        let f = match File::open(&path) {
            Ok(file) => file,
            Err(e) => panic!("couldn't open file {}, error {}", path, e),
        };

        let f = BufReader::new(f);
        for line in f.lines() {
            ORIGIN_VEC.lock().unwrap().push(line.unwrap());
        }
    }

    let path = create_path(text.to_string());
    {
        let g = match File::open(&path) {
            Ok(file) => file,
            Err(e) => panic!("couldn't open file {}, error {}", path, e),
        };

        let g = BufReader::new(g);
        for line in g.lines() {
            DEST_VEC.lock().unwrap().push(line.unwrap());
        }
    }
}

///Set the language for the program
/// # Examples
/// 
/// To set the program in french
/// 
/// ```
/// init("fr");
/// ```
///Don't forget that it will need the fr.txt file
/// 
pub fn init(new_lang: &str) {
    check_file(&new_lang);
    load_files(&new_lang);
}

///Use with every str that you need to translate
/// # Examples
/// 
/// ```
/// init("fr");
/// println(rtr("hello world"));
/// ```
/// Will return the sentence that correspond to "hello world" in the ./lang/fr.txt file
pub fn rtr(text: &str) -> String {
    match ORIGIN_VEC.lock().unwrap().binary_search(&text.to_string()) {
        Ok(index) => {
            return DEST_VEC.lock().unwrap()[index].clone();
        }
        Err(_) => return text.to_string(),
    }
}
