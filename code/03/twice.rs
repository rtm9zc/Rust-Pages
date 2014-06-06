//fn twice(n: int, f: extern fn(int) -> int) -> int {
//inline 1
fn twice(n: int, f: |int| -> int) -> int {
    let x = f(n);
    f(x)
}
//end 1
//inline 3
fn fofg(n: int, f: |int| -> int, g: |int| -> int) -> int  {
    f(g(n))
}
//end 3
//inline 2
fn successor(n: int) -> int { n + 1 }
fn double(n:int) -> int { n * 2 }

fn main()
{
    println!("Result: {:d}", twice(twice(1, successor), double));
}
//end 2
