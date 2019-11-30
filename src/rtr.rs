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
                panic!("could open the {} file", fileLang)
            }
        }

        match File::open(CreatePath("origin".to_string())) {
            Ok(_) => {
            }
            Err(_) => {
                panic!("couldn't open the origin file");
            }
        }
    }

    //load the file into the vec line by line
    fn loadFiles(text : &String) {
        //open the orginin langage file
        let f = File::open(CreatePath("origin".to_string())).unwrap();
        let f = BufReader::new(f);
        for line in f.lines() {
            originVec.lock().unwrap().push(line.unwrap());
        }

        let g = File::open(CreatePath(text.to_string())).unwrap();
        let g = BufReader::new(g);
        for line in g.lines() {
            destLangVec.lock().unwrap().push(line.unwrap());
        }
    }

    //create the file and his parent folder
    // fn createFile() {
    //     fs::create_dir_all("./lang");
    //     File::create(getPath());
    // }

    // //write every strings into the file
    // fn exportFile() {

    //     let mut v = strings.lock().unwrap().to_vec();
    //     let f = File::open(getPath()).unwrap();
    //     let mut f = LineWriter::new(f);
    //     v.sort_unstable(); // sort the vector
    //     v.dedup(); //remove all duplicated

    //     for i in 0..v.len(){
    //         v[i].push_str("\n"); //add a line return after each line
    //         f.write_all(v[i].as_bytes()).unwrap();
    //     }
    // }
    //function to call at the start of the program
   pub fn init(newLang: &String) {
        checkFiles(&newLang);
        loadFiles(&newLang);
    }

    //the main function of the crate, use it for every string that you want to be translated
    // pub fn tr(text: String) -> String {
    //     let v = &strings.lock().unwrap();
    //     if getFileExists() == true {
    //         let mut newText = v[searchIndex(text)].clone();
    //         newText.truncate(newText.len() - 2);
    //         return newText;
    //     }
    //     else {
    //         strings.lock().unwrap().push(text.clone());
    //         return text;
    //     }
    // }

    pub fn rtr(text : &str) -> String {
        
        match originVec.lock().unwrap().binary_search(&text.to_string())
            {
                Ok(index) => {
                let destVec = destLangVec.lock().unwrap();
                return destVec[index].clone();
                },
                Err(_) => return String::from(text.to_string()),

            }
    }
