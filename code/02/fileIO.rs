//inline 1
use std::io::File;

fn main()
{
    match File::create(&Path::new("message.txt")) {
        Some(mut file) => {
            file.write(bytes!("line one\n"));
            file.write_str("line two\n");
        }
        None =>{
            println("Opening message.txt failed!");
        }
    }
}
//end 1

fn fileRead() {
//inline 2
    match File::open(&Path::new("message.txt")) {
        Some(file) => {
            let reader = BufferedReader::new(file);
            //reading from file
        }
        None =>{
            println("Opening message.txt failed!");
        }
    }
//end 2
}