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
fn check_file(file_lang: &String) {
    match File::open(create_path(file_lang.to_string())) {
        Ok(_) => {}
        Err(_) => {
            panic!(
                "could open the {} file at path {}",
                file_lang,
                create_path(file_lang.to_string())
            );
        }
    }

    match File::open(create_path("origin".to_string())) {
        Ok(_) => {}
        Err(_) => {
            panic!("couldn't open the origin file in ./lang");
        }
    }
}

//load the file into the vec line by line
fn load_files(text: &String) {
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

//function to call at the start of the program
pub fn init(new_lang: &String) {
    check_file(&new_lang);
    load_files(&new_lang);
}

pub fn rtr(text: &str) -> String {
    match ORIGIN_VEC.lock().unwrap().binary_search(&text.to_string()) {
        Ok(index) => {
            return DEST_VEC.lock().unwrap()[index].clone();
        }
        Err(_) => return String::from(text.to_string()),
    }
}
