//inline 1
fn fofg(f: proc(int) -> int, g:proc(int) -> int) -> (proc(int) -> int) {
    proc(n: int) { g(f(n)) }
}
//end 1
fn square(n:int) -> int { n * n }

fn main()
{
//inline 2
    let toEighth = fofg(fofg(square, square), fofg(square, square));
    println!("Result: {:d}", toEighth(2));
//end 2
}
