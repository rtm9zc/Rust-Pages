fn main() {
//inline 1
	 match File::create(&Path::new("foo.txt")) {
    	       Some(mut file) => { //File was created successfully!, the variable "file" is the instance of File opened }
    	       None => { //File failed to open correctly
      	       	    fail!("Creating foo.txt failed!");
	 	    }
	 }
//end 1
//inline 2
      let reader = io::stdio::stdin();
      match reader.read_line() {
      	    Ok(stringRead) => println!("Line read: {}", stringRead),
    	    Err(e) => println!("Read failed! Error: {}", e)
	    }
//end 2
}