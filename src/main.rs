mod alg;
mod world;
mod characters;
mod socket;
use std::time::Duration;
use std::thread;
use std::str;
use std::cmp::min;
type T_ = usize;
const STEP: usize = 10;

fn main() {
    let mut w = world::make(W);
    let pgraph = world::positions_graph(&w, STEP, w.len());
    let mpath_graph = alg::power(pgraph.clone(), pgraph.len()-1,
        |x: &alg::Matrix<usize>, y: &alg::Matrix<usize>|
            alg::mmult(x, y,
                |&x: &usize, &y: &usize| min(x, y),
                |&x: &usize, &y: &usize| x+y));

    let mut pos: i32 = 48;
    let mut gpos = 0;
    let mut c = 0;

    let to_ = "127.0.0.1:41235"; let from_ = "127.0.0.1:41234";
    let socket = socket::One::new(from_, to_);

    loop {
        let x = match socket.recv() { Some(v) => act(v), None => 0 };
        characters::go(&mut w, pos as usize, (pos+x) as usize); pos += x;
        if c % 35 == 0 {
            let gp = characters::next_step(&mpath_graph, &pgraph, gpos, pos as usize);
            characters::go(&mut w, gpos, gp);
            gpos = gp;
        }
        socket.send(&world::render(&w, &STEP));
        thread::sleep(Duration::from_millis(10));
        c += 1; if c > 1001 { c = 0; }
    }
}

fn act(x: Vec<u8>) -> i32 {
    match str::from_utf8(&x).unwrap() {
        "ArrowLeft" => -1,
        "ArrowRight" => 1,
        "ArrowUp" => -(STEP as i32),
        "ArrowDown" => STEP as i32,
        _ => 0
    }
}

const W: &'static[u8] =
b"\
g-........\
.-.---....\
.....----.\
--.----...\
........*.";

