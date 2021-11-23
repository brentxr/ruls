extern crate termion;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use termion::{color, style};

// TODO get argument for directoy to check for files. If empty then use current directory.
fn main() -> io::Result<()> {

    //let files = fs::read_dir("./").unwrap();
    for f in fs::read_dir("./")? {
        let f = f?;
        let path = f.path();
        if path.is_dir() {
            println!("{}{:?}", color::Fg(color::Blue), path)
        }else {
            println!("{}{:?}", color::Fg(color::Yellow), path)
        }
        
        
    }

    Ok(())

}

// TODO add config file to change colors. If no config file then use defaults.


