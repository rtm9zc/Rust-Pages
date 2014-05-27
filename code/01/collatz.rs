//inline 2
use std::os;

fn main() {
//inline 3
	if os::args().len() < 2 {
		println!("Error: Please provide a number as argument.");
		return;
	}
//end 3

	let i = from_str::<int>(os::args()[1]).unwrap();
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
