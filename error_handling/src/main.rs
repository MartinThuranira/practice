use std::{error::Error,fs::File, io::{ErrorKind,Read,self}};
fn main(){
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => (file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:#?}", e),
            },
            other_error => {
                panic!("Problem creating the file {:#?}", other_error);
            }
        },
    };

    //let g = File::open("My_name.txt").unwrap();
    let g = File::open("My_name.txt").expect("File doesn't exist");
    _read_username_from_file();
    _read_username_from_file_shortcut();
}
//using closures
/*fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
*/

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
    }
    }

    fn _read_username_from_file_shortcut() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        //alternative
        //File::open("hello.txt")?.read_to_string(&mut s)?;
        //shortest
        //fs::read_to_string("hello.txt");
        Ok(s)
        }