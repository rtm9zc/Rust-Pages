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
	let i: int;
	match option {
	Some(x) => { i = x;}
	None => { i = 0;}
	}
	
	println!("{:d} has {:d} Collatz steps", i, collatz(i));
}

//inline 1
fn collatz(N: int) -> int {
	if N == 1 { return 0; }
	match N % 2 {
		0 => { 1 + collatz(N/2) }
		_ => { 1 + collatz(N*3+1) }
	}
//end 1
}
//end 2
