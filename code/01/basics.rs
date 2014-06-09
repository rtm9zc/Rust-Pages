#[allow(dead_code)]
fn main() {
//inline 8
let x = 4;
match x {
	0 => { } // Do nothing
	4 => { foo() } 
	_ => { bar() } // Matches every integer value
}
//end 8
//inline 1
let foo = 5; 
//end 1
//inline 4
//inline 10
let tup = (4, 5.0, false, "hello"); 
//end 4
let (a, b, c, _) = tup;
//end 10
println!("{} {} {}", a, b, c); // just to avoid unused warnings
//inline 5
if foo == 5 { 
    println!("it’s five") 
}
else if foo == 6 { 
    println!("it’s six")
}
else { 
    println!("it’s not five or six")
}
//end 5
//inline 6
match isOdd(foo) {
	true => println!("Odd"), // Notice the comma
	false => println!("Even")
}
//end 6
//inline 7
match isOdd(foo) {
	true => { println!("Odd") }
	false => { println!("Even") }
}
//end 7
//inline 9
match x {
	3 | 5 | 6  => { println!("First arm!") }
	10..16 => { println!("Second arm!") }   
	_      => { println!("Default arm!") }
}
//end 9
//inline 14
// Calls foo2 with 0, 1, ..., 9
for i in range(0, 10) {
	foo2(i)
}
//end 14
//inline 13
let mut i = 0;
while i < 10 { 
	println!("Hi there");
	i += 1; // Rust doesn't support ++ or --
}
//end 13
}
#[allow(dead_code)]
fn otherAlloc() {
//inline 2
let foo: int = 5;
//end 2
println!("{}", foo); // avoid warnings
}
fn foo2(_i: int) {

}
#[allow(dead_code)]
fn mutable() {
//inline 3
let mut foo = 5;
foo = foo + 1;
//end 3
println!("{}", foo); // avoid warnings
}

fn isOdd(x: int) -> bool {
	if x % 2 == 0 {
		false
	} else {
		true
	}
}

fn foo() {

}

fn bar() {

}

#[allow(dead_code)]
fn victory(status: (int, bool)) {
//inline 11
match status {
	(0, true) => println!("Success"),
	(_, true) => println!("Pyrrhic victory"), // Any first value matches
	(_, _) => println!("Complete loss") // Any pair of values will match
}
//end 11
}

#[allow(dead_code)]
fn increasing(tup: (int, int)) {
//inline 12
match tup {
	(x, y) if x > y => { println!("Decreasing") }
	(x, y) if y > x => { println!("Increasing") }
	_              => { println!("Equal") }
}
//end 12
}

#[allow(dead_code)]
fn express(y: int, x: int) {
//inline 16
let _status = match y {
    0..9    =>  { "Less than 10" }
    _       =>  { "Greater than 10" }
};
//end 16
//inline 15
let _foo = if x == 5 {
                "five"
          }
          else if x == 6 {
                "six"
          }
          else {
                "neither"
          };
//end 15
}

#[allow(dead_code)]
//inline 17
fn foobar() {
	println!("foo");
}
//end 17

#[allow(dead_code)]
//inline 18
fn foo1() {
	fn bar1() { println!("bar"); }
	bar1();
}
//end 18

#[allow(dead_code)]
//inline 19
fn rprime_sum(x: int, y: int, m: int) {
	match (x + y) % m {
		0 => println!("Multiple"),
		_ => println!("Relatively prime")
	}
}
//end 19


#[allow(dead_code)]
//inline 20
fn rprime(x: int, m: int) -> bool {
	(x % m) == 0
}
//end 20

#[allow(dead_code)]
//inline 21
fn square(x: int) -> int {
	x * x
}
//end 21
