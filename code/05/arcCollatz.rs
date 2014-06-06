//inline 1
extern crate sync;
use sync::{Mutex, Arc};

fn main() {
    let num = 0;
    let numArc = Arc::new(Mutex::new(num));
    for i in range(0, 5000) { 
        let locArc = numArc.clone();
        spawn(proc() {   
//inline 2
            let newNum;  
            {
                let mut value = locArc.lock();
                *value += 1;
                newNum = *value;
            }
//end 2
            let collatzN = collatz(newNum);
            println!("Collatz of {:d} = {:d}", newNum, collatzN);
        });
    }
}
//end 1
fn collatz(N: int) -> int {
    let mut nLoc = N;
    let mut out = 0;
    while (nLoc != 1) {
        out += 1;
        match nLoc % 2 {
	    0 => {nLoc = nLoc/2;}
	    _ => {nLoc = nLoc*3+1; }
	}
    }
    return out;
}
