extern crate termion;

use std::fmt;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use termion::{color, style};


fn main() -> io::Result<()> {


    for f in fs::read_dir("./")? {
        let f = f?;
        let path = f.path();
        if path.is_dir() {
            let meta = path.metadata()?;
            let perms = meta.permissions();
            //println!("{}", perms_to_string(format!("{:o}", perms.mode())));
            println!("{}{}\t\t{:?}", color::Fg(color::Blue), perms_to_string(format!("{:o}", perms.mode()), 6), path);
        }else {
            let meta = path.metadata()?;
            let perms = meta.permissions();
            println!("{}{}\t\t{:?}", color::Fg(color::Yellow), perms_to_string(format!("{:o}", perms.mode()), 9), path);
        }
        
        
    }

    //perm_to_string("11234");

    Ok(())

}


fn perms_to_string(perms_string: String, index_split: u8) -> String {

    // TODO need to implmement a better way to size string while handling errors
    let mut ret = String::with_capacity(15);

    // convert string to char vector
    let cvec: Vec<char> = perms_string.chars().collect();

    for n in 0..cvec.len() {
        
        match cvec[n] {
            '0' => ret.push_str("---"),
            '1' => ret.push_str("--x"),
            '2' => ret.push_str("-w-"),
            '3' => ret.push_str("-wx"),
            '4' => ret.push_str("r--"),
            '5' => ret.push_str("r-x"),
            '6' => ret.push_str("rw-"),
            '7' => ret.push_str("rwx"),
            _ => ret.push_str(""),
        }
    }

    // this needs to be handled better!!!!
    if index_split == 6 {
        ret[6..].to_string()
    } else if index_split == 9 {
        ret[9..].to_string()
    } else {
        ret[6..].to_string()
    }

    

}



