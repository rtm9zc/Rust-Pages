//inline 3
fn borrow2(r : &int) -> int {
   borrow(r)
}
//end 3

//inline 1
fn borrow(r : &int) -> int {
    *r
}
//end 1

//inline 4
fn increment(r : &mut int) {
    *r = *r + 1;
}
//end 4

fn incrementExample() {
//inline 5
    let mut x = box 10;
    increment(x);
//end 5
}
fn main() {
//inline 2
    let mut x = box 10;
    println!("borrow(x): {:d}", borrow(x));
//end 2
    increment(x);
    println!("borrow2(x): {:d}", borrow2(x));

    let mut val1 = 10;
    let mut val2 = 20;
//inline 6
    let mut borrowed = &mut val1;
//end 6
    borrowed = &mut val2;
    *borrowed = 11;
}


