mod my;
use std::time::Duration;
use std::thread;
use std::net::UdpSocket;
use std::str;
use std::cmp::min;
type T_ = usize;
const STEP: usize = 10;
const MAX: usize = 255;

fn matrix_w(w: &Vec<u8>, step: usize, n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![MAX; n]; n];
    let wss = weights(w, &possible_steps(step as i32, n as i32));
    for (v,ws) in res.iter_mut().zip(wss) {
        for (i, w) in ws { v[i] = w; }
    }
    res
}

fn lr_guards(m: &mut Vec<Vec<usize>>) {
    let n = m.len()-1;
    for i in 1..n {
        if i % STEP == 0 { m[i][i-1] = MAX; }
        if i % STEP == STEP-1 { m[i][i+1] = MAX; }
    }
}
fn d_zeros(m: &mut Vec<Vec<usize>>) {
    let n = m.len();
    for i in 0..n {
        m[i][i] = 0;
    }
}

fn weights(x: &Vec<u8>, y: &Vec<Vec<usize>>) -> Vec<Vec<(usize, usize)>> {
    type Result_ = Vec<(usize, usize)>;
    x.iter()
        .zip(y)
        .map(|(&c, xs)|
            xs.iter()
                .map(move |&x|
                    (x, if c as char == '.' { 1 } else { MAX }))
                .collect::<Result_>())
        .collect::<Vec<Result_>>()
}

fn possible_steps(step: i32, n: i32) -> Vec<Vec<usize>> {
    type Result_ = Vec<usize>;
    (0..n)
        .map(|x|
            vec![x-1, x-step, x+1, x+step].into_iter()
                .filter(|&x| x >= 0 && x < n)
                .map(|x| x as usize)
                .collect::<Result_>())
        .collect::<Vec<Result_>>()
}

fn act(x: &[u8]) -> i32 {
    match str::from_utf8(x).unwrap() {
        "ArrowLeft" => -1,
        "ArrowRight" => 1,
        "ArrowUp" => -(STEP as i32),
        "ArrowDown" => STEP as i32,
        _ => 2
    }
}

fn pr(w: &mut [u8], i: i32, j: i32) -> () {
    w[i as usize] = '.' as u8;
    w[j as usize] = '*' as u8;
}

fn ghost(w: &mut [u8], i: usize, j: usize) -> () {
    w[i] = '.' as u8;
    w[j] = 'g' as u8;
}

fn ghost_new_pos(xd: &Vec<Vec<usize>>, ghost: usize, hero: usize) -> usize {
    let nd = xd.len()-1;
    let h = my::power(xd.clone(), nd,
        |x: &my::Matrix<usize>, y: &my::Matrix<usize>|
            my::mmult(x, y,
                |&x: &usize, &y: &usize| min(x, y),
                |&x: &usize, &y: &usize| x+y));
    my::min_path(&h, &xd, ghost, hero, h[ghost][hero].clone())[1]
}

fn main() {
    let to_ = "127.0.0.1:41235";
    let socket = UdpSocket::bind("127.0.0.1:41234").unwrap();
    let mut w = make_world();

    let mut xd = matrix_w(&w, STEP, w.len());
    d_zeros(&mut xd);
    lr_guards(&mut xd);

    let mut pos = 38;
    let mut ghost_pos = 0;
    let mut c = 0;

    let mut buf = [0; 15];
    let _ = socket.set_read_timeout(Some(Duration::from_millis(1)));

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt,_)) => {
                let x = act(&mut buf[..amt]);
                pr(&mut w, pos, pos+x); pos += x;
            }
            Err(_) => { }
        }
        if c % 10 == 0 {
            let gp = ghost_new_pos(&xd, ghost_pos, pos as usize);
            ghost(&mut w, ghost_pos, gp);
            ghost_pos = gp;
        }
        let _ = socket.send_to(&view_world(&w), to_);
        thread::sleep(Duration::from_millis(10));
        c += 1; if c > 1000 { c = 0; }
    }
}

fn view_world(w: &[u8]) -> Vec<u8> {
    let mut x = Vec::new();
    for a in w.chunks(STEP) {
        x.extend_from_slice(a);
        x.extend_from_slice(b"<br />");
    }
    x
}

fn make_world() -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(
b"\
.,........\
.,.,,,....\
.....,,,,.\
,,.,,,,...\
.........."
);
    v
}

