//inline 1
fn twice(f: proc(int) -> int) -> (proc(int) -> int) {
    proc(n: int) { f(f(n)) }
}
//end 1
fn double(n:int) -> int { n * 2 }

fn main()
{
//inline 2
    let hexaple = twice(twice(double));
    println!("Result: {:d}", hexaple(2));
//end 2
}
