#[allow(non_snake_case)]

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::sync::Mutex;

    lazy_static! {
        static ref originVec : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the origin langage
        static ref destLangVec : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the source language
    }

    fn CreatePath(source : String) -> String {
        let mut newPath = String::from("./lang/.txt");
        newPath.insert_str(7, source.as_str());
        return newPath;
    }
    
    //check if file exist true if exist
    fn checkFiles(fileLang : &String) {
        match File::open(CreatePath(fileLang.to_string())) {
            Ok(_) => {
            }
            Err(_) => {
                panic!("could open the {} file at path {}", fileLang, CreatePath(fileLang.to_string()));
            }
        }

        match File::open(CreatePath("origin".to_string())) {
            Ok(_) => {
            }
            Err(_) => {
                panic!("couldn't open the origin file in ./lang");
            }
        }
    }

    //load the file into the vec line by line
    fn loadFiles(text : &String) {
        //open the origin langage file

        let path = CreatePath("origin".to_string()); {
            
            let f = match File::open(&path){
                Ok(file) => file,
                Err(e) => panic!("couldn't open file {}, error {}", path, e)
            };
        

            let f = BufReader::new(f);
            for line in f.lines() {
                originVec.lock().unwrap().push(line.unwrap());
            }
        }

        let path = CreatePath(text.to_string()); {
            let g = match File::open(&path){
                Ok(file) => file,
                Err(e) => panic!("couldn't open file {}, error {}", path, e)
            };

            let g = BufReader::new(g);
            for line in g.lines() {
                destLangVec.lock().unwrap().push(line.unwrap());
            }
        }
    }

    //function to call at the start of the program
   pub fn init(newLang: &String) {
        checkFiles(&newLang);
        loadFiles(&newLang);
    }

    pub fn rtr(text : &str) -> String {
        
        match originVec.lock().unwrap().binary_search(&text.to_string())
            {
                Ok(index) => {
                return destLangVec.lock().unwrap()[index].clone();
                },
                Err(_) => return String::from(text.to_string()),

            }
    }
