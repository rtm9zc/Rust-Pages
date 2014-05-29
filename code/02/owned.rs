fn main() {
//inline 1
    let x = ~10;
    println!("{:d}", *x);
//end 1
}

fn cloning() {
//inline 2
    let x = ~10;
    let y = x.clone();
    println!("{:d}", *x);
//end 2
}
