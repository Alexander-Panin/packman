mod alg;
mod world;
mod characters;
mod socket;
use std::time::Duration;
use std::thread;
use std::str;
use std::cmp::min;
const STEP: usize = 22;
type world_t = Vec<u8>;

fn main() {
    let mut w: world_t = world::make(W);
    let pgraph = world::positions_graph(&w, STEP, w.len());
    let mpath_graph = alg::power(pgraph.clone(), pgraph.len()-1,
        |x: &alg::Matrix<usize>, y: &alg::Matrix<usize>|
            alg::mmult(x, y,
                |&x: &usize, &y: &usize| min(x, y),
                |&x: &usize, &y: &usize| x+y));

    let mut pos: i32 = 108;

    let mut gpos = 0;
    let gposs = 0;
    let mut gpose = 21;
    let mut prev_gpos = (0, '.' as u8);
    let mut gv = 'G';
    let mut gposw = 0;
    let mut gposk = 0;

    let mut gpos2 = 21;
    let gpos2s = 21;
    let mut gpos2e = 0;
    let mut prev_gpos2 = (21, '.' as u8);
    let mut gv2 = 'G';
    let mut gpos2w = 0;
    let mut gpos2k = 0;

    let mut c = 0;

    let to_ = "127.0.0.1:41235"; let from_ = "127.0.0.1:41234";
    let socket_ = socket::One::new(from_, to_);

    loop {
        let x = match socket_.recv() { Some(v) => act(v), None => 0 };
        if can_step(&x, &pos, &w) {
            if w[(pos+x) as usize] == '!' as u8 { gposw += 15; gpos2w += 15; }
            w[(pos+x) as usize] = '*' as u8;
            w[pos as usize] = '.' as u8;
            pos += x;
        }
        if c % 29 == 0 {
            let mut gp;
            let mut skip = false;
            if gposw == 0 {
                gv = 'G';
                gp = characters::next_step(&mpath_graph, &pgraph, gpos, pos as usize);
            } else {
                gv = if gv == '^' { '^' } else {'g'};
                gp = characters::next_step(&mpath_graph, &pgraph, gpos, gpose as usize);
                if mpath_graph[gpos][gpose] < 2 {
                    gpose = if gpose == gposs { gpos2s } else { gposs };
                    gp = characters::next_step(&mpath_graph,
                        &pgraph, gpos, gpose as usize);
                }
                gposw -= 1;
            }
            let predy = |y: u8| y == 'g' as u8 || y == 'G' as u8 || y == '^' as u8;
            if predy(w[gp]) {
                gp = characters::next_step(&mpath_graph,
                    &pgraph, gpos, gposs as usize);
                if predy(w[gp]) {
                    gp = characters::next_step(&mpath_graph,
                        &pgraph, gpos, gpos2s as usize);
                    if predy(w[gp]) {
                        skip = true;
                    }
                }
            }
            if !skip {
                w[prev_gpos.0] = if prev_gpos.1 == '*' as u8 {'.' as u8} else { prev_gpos.1 };
                prev_gpos.0 = gp; prev_gpos.1 = w[gp];
                w[gp] = gv as u8;
                gpos = gp;
            }
        }

        if c % 23 == 0 {
            let mut skip = false;
            let mut gp;
            if gpos2w == 0 {
                gv2 = 'G';
                gp = characters::next_step(&mpath_graph, &pgraph, gpos2, pos as usize);
            } else {
                gv2 = if gv2 == '^' { '^' } else {'g'};
                gp = characters::next_step(&mpath_graph,
                    &pgraph, gpos2, gpos2e as usize);
                if mpath_graph[gpos2][gpos2e] < 2 {
                    gpos2e = if gpos2e == gpos2s { gposs } else { gpos2s };
                    gp = characters::next_step(&mpath_graph,
                        &pgraph, gpos2, gpos2e as usize);
                }
                gpos2w -= 1;
            }
            let predy = |y: u8| y == 'g' as u8 || y == 'G' as u8 || y == '^' as u8;
            if predy(w[gp]) {
                gp = characters::next_step(&mpath_graph,
                    &pgraph, gpos2, gposs as usize);
                if predy(w[gp]) {
                    gp = characters::next_step(&mpath_graph,
                        &pgraph, gpos2, gpos2s as usize);
                    if predy(w[gp]) {
                        skip = true;
                    }
                }
            }
            if !skip {
                w[prev_gpos2.0] = if prev_gpos2.1 == '*' as u8  {'.' as u8} else { prev_gpos2.1 };
                prev_gpos2.0 = gp; prev_gpos2.1 = w[gp];
                w[gp] = gv2 as u8;
                gpos2 = gp;
            }
        }
        if pos == gpos as i32 {
            if gv == 'g' { gv = '^'; gposw = 25; }
        }
        if pos == gpos2 as i32 {
            if gv2 == 'g' { gv2 = '^'; gpos2w = 25; }
        }

        socket_.send(&world::render(&w, &STEP));
        thread::sleep(Duration::from_millis(10));
        c += 1; if c > 1001 { c = 1; }
    }
}

fn can_step(&x: &i32, &cur: &i32, w: &world_t) -> bool {
    if x == 0 {return false;}
    if x == 1 && cur as usize % STEP == STEP-1 {return false;}
    if x == -1 && cur as usize % STEP == 1 {return false;}
    if x + cur < 0 || x + cur > (w.len()-1) as i32 {return false;}
    if w[(x+cur) as usize] == '-' as u8 {return false;}
    true
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
G-,,,,,,,,---........G\
.-.-----..!...........\
.......---,,,..---....\
--.-,,,....!----......\
..............!.....*.";

