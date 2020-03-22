use std::fs;
use std::collections::HashMap;

use log::{error};

pub fn get_content(path: &str) -> String {
    let content: String = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    content
}

pub fn create_file(path: &str) -> fs::File {
    let report_file = fs::File::create(path); 
    match report_file {
        Ok(f) => f,
        Err(s) => {
            error!("Unable to create report file");
            panic!(s)
        } 
    }
}

pub fn find_and_replace(mut content: String, map: &HashMap<String, String>) -> String {
    if content.contains("{{") { //Avoid unnecessary looping, might be tricked by json but would avoid most
        for (k, v) in map {
            let replaced_string = &format!("{{{{{}}}}}", k);
            let replacing_string = v;
            content = content.replace(replaced_string, replacing_string);
        }
    }
    
    content
}