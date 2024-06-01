use std::fs::File;
use std::io::{self, Read};

fn main() {
    // panic!("There is an error here");

    /*
    let greeting_file_result = File::open("filename.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("filename.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file {:?}", error)
            },
            other_error => panic!("There was a problem {:}", other_error)
        }
    };
    println!("{:?}", greeting_file)
    */
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("filename.txt");
        let mut username_file =  match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username){
            Ok(_) => Ok(username),
            Err(error) => Err(error)
        }
    }
    */
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("filename.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn last_char_frist_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
