extern crate termion;

use std::io::{stdout, self, Write};

use std::env;

use std::fs::{self};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use users::get_user_by_uid;

use chrono::{TimeZone, Utc};

use termion::raw::IntoRawMode;
use termion::{color, style};


fn main() -> std::io::Result<()>  {
    // get arguments
    let args: Vec<String> = env::args().collect();

    // MODES:
    // 0 = default - show only non-hidden file names
    // 1 = show hidden and non-hdden file names
    // 2 = show only non-hidden file information
    // 3 = show all information on all hidden and non-hidden files
    let mut mode = 0;

    if args.len() == 2 && args[1] == "-a" {
        mode = 1;
    } else if args.len() == 2 && args[1] == "-la" {
        mode = 3;
    } else if args.len() == 2 && args[1] == "-l" {
        mode = 2;
    }

    display(mode)?; 

    Ok(())
}


fn display(mode: i8) -> std::io::Result<()> {

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    for f in fs::read_dir("./")? {
        // Get Directory/File information and metadata
        let f = f?;
        let path = f.path();
        let meta = path.metadata()?;
        let perms = meta.permissions();
        let modified = Utc.timestamp(meta.mtime(), 0);
        let owner = get_user_by_uid(meta.uid()).unwrap();
        let size = meta.size();

        let f_name = path.to_string_lossy();
        
        if mode == 2 && &f_name[2..3] != "." || mode == 3 {
            // build permisisons string 
                
            let cvec: Vec<char> = format!("{:o}", perms.mode()).chars().collect();

            let mut beginnning = 0;

            if path.is_dir() {
                beginnning = 2;
                write!(stdout, "{}d", color::Fg(color::Blue))?;
            } else {
                beginnning = 3;
                write!(stdout, "{}.", color::Fg(color::White))?;
            }
            for n in beginnning..cvec.len() {
                // change this to remove the first char if its a directoy to combine these two cases
                match cvec[n] {
                    '0' => write!(stdout, "{}---", color::Fg(color::White))?,
                    '1' => write!(
                        stdout,
                        "{}--{}x",
                        color::Fg(color::White),
                        color::Fg(color::Green)
                    )?,
                    '2' => write!(
                        stdout,
                        "{}-{}w-",
                        color::Fg(color::White),
                        color::Fg(color::Red)
                    )?,
                    '3' => write!(
                        stdout,
                        "{}-{}w{}x",
                        color::Fg(color::White),
                        color::Fg(color::Red),
                        color::Fg(color::Green)
                    )?,
                    '4' => write!(
                        stdout,
                        "{}{}r--",
                        color::Fg(color::White),
                        color::Fg(color::Yellow)
                    )?,
                    '5' => write!(
                        stdout,
                        "{}{}r-{}x",
                        color::Fg(color::White),
                        color::Fg(color::Yellow),
                        color::Fg(color::Green)
                    )?,
                    '6' => write!(
                        stdout,
                        "{}{}r{}w{}-",
                        color::Fg(color::White),
                        color::Fg(color::Yellow),
                        color::Fg(color::Red),
                        color::Fg(color::White)
                    )?,
                    '7' => write!(
                        stdout,
                        "{}{}r{}w{}x",
                        color::Fg(color::White),
                        color::Fg(color::Yellow),
                        color::Fg(color::Red),
                        color::Fg(color::Green)
                    )?,
                    _ => write!(stdout, "")?,
                };
            }
            
            // TODO this needs to be recplaced with columnn functionality
            write!(stdout, "\t")?;

            // calculate and write size format
            // TODO add constants for sizes
            write!(stdout, "{}", color::Fg(color::Green))?;
            if size < 1024 {
                write!(stdout, "{:.1$}b", size, 1)?;
            } else if size < 1048576 {
                write!(stdout, "{:.1$}k", size as f64 / 1024.0, 1)?;
            } else if size < 1073741824 {
                write!(stdout, "{:.1$}m", size as f64 / 1048576.0, 1)?;
            } else if size < 1099511627776 {
                write!(stdout, "{:.1$}g", size as f64 / 1073741824.0, 1)?;
            } else {
                write!(stdout, "{:.1$}t", size as f64 / 1099511627776.0, 1)?;
            }

            // TODO this needs to be recplaced with columnn functionality
            write!(stdout, "\t")?;

            // write last modified date
            write!(
                stdout,
                "{}{}",
                color::Fg(color::Blue),
                modified.format("%Y-%m-%d")
            )?;

            // TODO this needs to be recplaced with columnn functionality
            write!(stdout, "\t")?;

            // write owner
            write!(
                stdout,
                "{}{}",
                color::Fg(color::Yellow),
                owner.name().to_string_lossy()
            )?;

            // TODO this needs to be recplaced with columnn functionality
            write!(stdout, "\t")?;


            write!(stdout, "{}{}", color::Fg(color::Blue), &f_name[2..])?;
            write!(stdout, "\r\n")?;
 
        } else if mode == 0 && &f_name[2..3] != "." || mode ==1 {
            write!(stdout, "{}{}", color::Fg(color::Blue), &f_name[2..])?;
            write!(stdout, "\r\n")?; 
        }

        
    }   

    Ok(())
}
