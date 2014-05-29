//inline 1
use std::io::{File, Append, ReadWrite};

fn main() {
   let file = File::open_mode(&Path::new("message.txt"), Append, ReadWrite);
}
//end 1