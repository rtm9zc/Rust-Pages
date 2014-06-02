fn plustwo(int: x) -> int {
   x+2
}

fn main() {
//inline 1
   let (port, chan): (Port<int>, Chan<int>) = Chan::new();
   spawn(proc() {
        let x = 10;
        chan.send(plustwo(x));
    });
    let new_x = port.recv(); // new_x == 12
//end 1
//inline 2
    let (port1, chan1): (Port<int>, Chan<int>) = Chan::new();
    let (port2, chan2): (Port<int>, Chan<int>) = Chan::new();
    do spawn {
        let x = port2.recv();
        chan1.send(plustwo(x));
    }
    chan2.send(10);
    let new_x = port1.recv(); //new_x == 12
//end 2
}