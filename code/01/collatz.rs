//inline 2
use std::os;
use std::int;

fn main() {
//inline 3
	if os::args().len() < 2 {
		println!("Error: Please provide a number as argument.");
		return;
	}
//end 3

	let option = int::parse_bytes(os::args().get(1).clone().into_bytes(), 10);
	let i: int = match option { 
                         Some(x) => { x }
   	                 None => { 0 } 
                     } ;
	println!("{:d} has {:d} Collatz steps", i, collatz(i));
}

//inline 1
fn collatz(n: int) -> int {
	if n == 1 { return 0; }
	match n % 2 {
		0 => { 1 + collatz(n / 2) }
		_ => { 1 + collatz(n*3 + 1) }
	}
}
//end 1
//end 2
