fn increment(v: &[int]) -> ~[int] {
    let mut res = ~[];

    for x in v.iter() {
        res.push(*x + 1);
    }

    res
}

//inline 1
fn main() {
   let p = ~[1, 2, 3];
   let q = increment(p);
   for &x in q.iter() {
      print!("{:d} ", x);
   }
}
//end 1
