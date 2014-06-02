//inline 1
let name[: type] [= value];
//end 1

//inline 2
let foo = 5;
foo = 6;
//end 2

//inline 3
if foo == 5 
    println!("it worked");
//end 3

//inline 4
let x = (51, true);
//end 4

//inline 5 
fn main() {
    // Code goes here
}
//end 5

//inline 6
while condition {
	`code`
}
//end 6

//inline 7
loop {
	`code`
}
//end 7

//inline 8
fn name() {
	`code`
}
//end 8

//inline 9
    let x = ~10;
    let y = x;
    println!("{:d}", *x);
//end 9

//inline 10
    let x = ~10
    increment(x)
//end 10

//inline 11
    let mut val1 = 10;
    let mut val2 = 20;
    let mut borrowed = &val1;
    borrowed = &val2;
    *borrowed = 11;
//end 11

//inline 12
    let mut val1 = 10;
    let mut val3 = 10.0;
    let borrowed = &val1;
    borrowed = &val3;
//end 12

//inline 13
    let mut x = 10;
    {
        let y = &x;
        x = 11; // Error
    }
    x = 12; //This is fine
//end 13

//inline 14
    let mut reference: &~int;
    {
        let val: ~int = ~10;
        reference = &val;
    } //val deallocated here
    println!("{:d}", **reference); //Referencing something that's gone!
//end 14

//inline 15
do expr { block }
//end 15

//inline 16
expr(proc() { block })
//end 16

//inline 17
do spawn { a; }
//end 17

//inline 18
spawn(proc() { a; })
//end 18

//inline 19
fn main() {
    let mut n = 10;
    spawn(proc() { count("A", n); });
    ...
//end 19