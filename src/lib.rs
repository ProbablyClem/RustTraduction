//! # rtr
//! 
//! rtr is a crate that return strings from translation files <br/>
//! _Thoses files need to be manually created with the [rtrTranslator](https://github.com/ProbablyClem/rtrTranslator) software_
//! source code at : 
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate string_format;
use string_format::*;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref ORIGIN_VEC : Vec<String> = load_vec("origin"); //The vector that containt every line of the origin langage
    static ref DEST_VEC : Vec<String> = load_vec("fr"); //The vector that containt every line of the destination language (e.g fr, it, de...)
    static ref STATE : Mutex<AtomicBool> = Mutex::new(AtomicBool::new(true));
}

fn create_path(source: String) -> String {
    let mut new_path = String::from("./lang/.txt");
    new_path.insert_str(7, source.as_str());
    return new_path;
}

fn load_vec(lang : &str) -> Vec<String>{
    let f = File::open(create_path(lang.to_string())).unwrap();
    let mut vec = Vec::new();

    let f = BufReader::new(f);

        for line in f.lines() {
           vec.push(line.unwrap());
        }
    vec
}


///Set the language for the program
/// # Examples
/// 
/// To set the program in french
/// 
/// ```
/// extern crate rtr;
/// 
/// match rtr::init("fr"){
///     Ok(_) => (),
///     Err(e) => panic!("couldn't load the fr translation file, error {}", e),
/// };
/// ```
///Don't forget that it will need the ./lang/fr.txt file
/// 
/// # Errors
///Will return an std::io:Error in case of trouble loading the translation file 
pub fn init(new_lang: &str) -> Result<(), io::Error> {
    // let f = File::open(create_path("origin".to_string()))?;

    //     let f = BufReader::new(f);

    //     for line in f.lines() {
    //         ORIGIN_VEC.lock().unwrap().push(line.unwrap());
    //     }

    //     let g = File::open(create_path(new_lang.to_string()))?;

    //     let g = BufReader::new(g);
    //     for line in g.lines() {
    //         DEST_VEC.lock().unwrap().push(line.unwrap());
    //     }

        Ok(())
}

///Disable the translation <br/>
/// [rtr](./fn.rtr.html) will returns the argument
/// 
/// # Examples
/// 
/// disable() is mostly useful for error handling
/// ```
/// extern crate rtr;
/// use rtr::rtr;
/// 
/// match rtr::init("fr") {
///     Ok(_) => (),
///     Err(e) => {
///         println!("couldn't load fr translation file");
///         rtr::disable();
///     }
/// }
/// ```
/// 
/// use [enable](./fn.enable.html) to enable it again
pub fn disable() {
    &STATE.lock().unwrap().store(false, Ordering::Relaxed);
}

///Enable the translation<br/>
///[rtr](./fn.rtr.html) will now try to read strings from the translation file <br/>
///*_The translation then need to be loaded with [init](./fn.init.html)!_*<br/>
///Only useful after using [disable](./fn.disable.html)
pub fn enable() {
    &STATE.lock().unwrap().store(true, Ordering::Relaxed);
}

///Return the state of the translation<br/>
///If *false* [rtr](./fn.rtr.html) will return the argument<br/>
///If  *true* [rtr](./fn.rtr.html) will act normaly<br/>
///True by default
///  # Examples
/// ```
///extern crate rtr;
///use rtr::rtr;
///rtr::init("fr").unwrap();
/// 
///assert_eq!(rtr::is_enabled(), true);
/// //assuming that "hello" correspond to "bonjour" in the fr.txt file
///assert_eq!(rtr("hello"), "salut".to_string());
/// 
///rtr::disable();
///assert_eq!(rtr("hello"), "hello".to_string());
pub fn is_enabled() -> bool{
    return STATE.lock().unwrap().load(Ordering::Relaxed);
}

///Use with every str that you need to translate
/// # Examples
/// 
/// ```
/// extern crate rtr;
/// use rtr::rtr;
/// 
/// rtr::init("fr");
/// println!("{}", rtr("hello world"));
/// ```
/// Will return the sentence that correspond to "hello world" in the ./lang/fr.txt file
pub fn rtr(text: &str) -> String {
    if &STATE.lock().unwrap().load(Ordering::Relaxed) == &true {
        match ORIGIN_VEC.binary_search(&text.to_string()) {
            Ok(index) => {
                return DEST_VEC[index].clone();
            }
            Err(_) => return text.to_string(),
        }
    }

    else {
        return text.to_string();
    }
}

///Same as rtr but will format the text.  
/// Will translate the first argument and then format with the others.
/// 
/// # Example
/// ```
/// #[macro_use]
/// extern crate rtr;
/// use rtr::*;
/// 
/// init("fr");
/// 
/// assert_eq!(rtr!("hello {} {}", "cruel".to_string(), "monde".to_string()), String::from("salut cruel monde"));
/// ```
#[macro_export]
macro_rules! rtr {
    ($($x:expr)?, $($arg:tt)*) => {
        string_format!(rtr($($x)?), $($arg)*);
    };
}

#[test]
fn format(){
    init("fr");
    //assert_eq!(rtr("hello {}"), String::from("salut {}"));
    assert_eq!(rtr!("hello {} {}", "cruel".to_string(), "monde".to_string()), String::from("salut cruel monde"));
}