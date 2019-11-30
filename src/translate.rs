// #![allow(non_snake_case)]
// #![allow(dead_code)]
// #[macro_use]

// extern crate lazy_static;

// pub mod translate {

    

//     use std::fs;
//     use std::fs::File;
//     use std::io::prelude::*;
//     use std::io::BufReader;
//     use std::io::LineWriter;
//     use std::sync::atomic::{AtomicBool, Ordering};
//     use std::sync::Mutex;

//     lazy_static! {
//         static ref strings : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line to be translated
//         static ref lang : Mutex<String> = Mutex::new(String::new()); //the langue of the current program
//         static ref fileExist : Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false)); //if the file exist
//         static ref path : Mutex<String> = Mutex::new(String::new()); //Path to the file
//     }

//     fn setLang(text: String) {
//         lang.lock().unwrap().clear();
//         lang.lock().unwrap().push_str(&text);
//     }

//     fn getLang() -> String {
//         return lang.lock().unwrap().to_string();
//     }

//     fn setFileExist(exist: bool) {
//         fileExist.lock().unwrap().store(exist, Ordering::Relaxed);
//     }

//     fn getFileExists() -> bool {
//         return fileExist.lock().unwrap().load(Ordering::Relaxed);
//     }

//     fn setPath() {
//         path.lock().unwrap().clear();
//         path.lock().unwrap().push_str("./lang/.txt");
//         path.lock().unwrap().insert_str(7, getLang().as_str());
//     }

//     fn getPath() -> String {
//         return path.lock().unwrap().to_string();
//     }

//     fn getVecSize() -> usize {
//         return strings.lock().unwrap().len();
//     }

//     //check if file exist true if exist
//     fn checkFile() -> bool {
//         match File::open(getPath()) {
//             Ok(f) => {
//                 setFileExist(true);
//                 return true;
//             }
//             Err(e) => {
//                 setFileExist(false);
//                 return false;
//             }
//         }
//     }

//     //load the file into the vec line by line
//     fn loadFile() {
//         let f = File::open(getPath()).unwrap();
//         let f = BufReader::new(f);

//         for line in f.lines() {
//             strings.lock().unwrap().push(line.unwrap());
//         }
//     }

//     //create the file and his parent folder
//     fn createFile() {
//         fs::create_dir_all("./lang");
//         File::create(getPath());
//     }

//     //write every strings into the file
//     fn exportFile() {

//         let mut v = strings.lock().unwrap().to_vec();
//         let f = File::open(getPath()).unwrap();
//         let mut f = LineWriter::new(f);
//         v.sort_unstable(); // sort the vector
//         v.dedup(); //remove all duplicated

//         for i in 0..v.len(){
//             v[i].push_str("\n"); //add a line return after each line
//             f.write_all(v[i].as_bytes()).unwrap();
//         }
//     }

//     fn searchIndex(text : String) -> usize {
//         match strings.lock().unwrap().binary_search(&text) {
//             Ok(index) => return index,
//             Err(e) => return strings.lock().unwrap().len()+1,
//         }
//     }
//     //function to call at the start of the program
//    pub fn init(newLang: String) -> Result<i32, String> {
//         setLang(newLang);
//         if checkFile() == true {
//             loadFile();
//         } else {
//             createFile();
//         }
//         return Ok(1);
//     }

//     //the main function of the crate, use it for every string that you want to be translated
//     pub fn tr(text: String) -> String {
//         let v = &strings.lock().unwrap();
//         if getFileExists() == true {
//             let mut newText = v[searchIndex(text)].clone();
//             newText.truncate(newText.len() - 2);
//             return newText;
//         }
//         else {
//             strings.lock().unwrap().push(text.clone());
//             return text;
//         }
//     }
// }
