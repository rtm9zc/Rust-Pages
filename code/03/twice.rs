//fn twice(n: int, f: extern fn(int) -> int) -> int {
//inline 1
fn twice(n: int, f: |int| -> int) -> int {
    f(f(n))
}
//end 1
//inline 2
fn successor(n: int) -> int { n + 1 }
fn double(n:int) -> int { n * 2 }

fn main()
{
    println!("Result: {:d}", twice(twice(1, successor), double));
}
//end 2
