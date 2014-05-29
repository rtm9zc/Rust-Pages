//inline 1
use std::io::buffered::BufferedReader;
use std::io::stdin;

fn main(){
    let mut stdin = BufferedReader::new(stdin());
    for line in stdin.lines() {
        print(line);
    }
}
//end 1

fn io2() {
//inline 2
    let mut stdin = BufferedReader::new(stdin());
    for i in range(0, 5) {
        match stdin.read_line() {
            Some(line) => {
                print(line);
            }
            None => {
                println("End of input!");
                break;
            }
        }
    }
//end 2
}