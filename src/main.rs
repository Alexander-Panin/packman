mod alg;
mod world;
mod characters;
mod socket;
use std::time::Duration;
use std::thread;
use std::str;
use std::cmp::min;
use std::slice::Iter;
use characters::{Hero, Ghost};

const STEP: usize = 44;
#[allow(non_camel_case_types)]
type world_t = Vec<u8>;
#[allow(non_camel_case_types)]
type matrix_t = Vec<Vec<usize>>;

fn main() {
    let mut w: world_t = world::make(W);
    let weights = world::positions_graph(&w, STEP, w.len());
    let min_paths = alg::power(weights.clone(), weights.len()-1,
        move |x: &alg::Matrix<usize>, y: &alg::Matrix<usize>|
            alg::mmult(x, y,
                move |&x: &usize, &y: &usize| min(x, y),
                move |&x: &usize, &y: &usize| x+y));

    let mut hero = Hero::new(w.iter().position(|&c| c as char == '*').unwrap());
    let (gx, gy, gz) = find_ghosts_positions(w.iter());
    let mut ghostx = Ghost::new(gx, 31, &min_paths, &weights);
    let mut ghosty = Ghost::new(gy, 29, &min_paths, &weights);
    let mut ghostz = Ghost::new(gz, 23, &min_paths, &weights);

    let to_ = "127.0.0.1:41235"; let from_ = "127.0.0.1:41234";
    let socket_ = socket::One::new(from_, to_);
    let mut time_: usize = 0;

    loop {
        let step = match socket_.recv() { Some(v) => act(v), None => 0 };

        if let Some((position, c)) = hero_move(&mut hero, step, &mut w) {
            affect_ghost(&mut ghostx, c, position);
            affect_ghost(&mut ghosty, c, position);
            affect_ghost(&mut ghostz, c, position);
        }
        ghost_move(&mut ghostx, time_, ghosty.start_position, &hero, &mut w);
        ghost_move(&mut ghosty, time_, ghostz.start_position, &hero, &mut w);
        ghost_move(&mut ghostz, time_, ghostx.start_position, &hero, &mut w);

        socket_.send(&world::render(&w, &STEP));
        thread::sleep(Duration::from_millis(10));
        time_+=1;
    }
}

fn hero_move(hero: &mut Hero, x: i32, mut w: &mut world_t) -> Option<(usize, char)> {
    if !hero.can_step(x, w, STEP) {return None;}
    let new_position = (hero.current_position as i32 + x) as usize;
    let c = w[new_position] as char;
    hero.render(w, new_position);
    Some((new_position, c))
}
fn ghost_move(ghost: &mut Ghost, t: usize, x: usize, hero: &Hero, mut w: &mut world_t) {
    if !ghost.is_moveable(t) { return; }
    let mut goals = vec![];
    if ghost.can_hunt() {goals.push(hero.current_position);}
    else {ghost.make_stronger();}
    goals.extend(ghost.escape_or_default(x));
    if let Some(next_step) = ghost.can_step(w, goals) {
        ghost.render(w, next_step);
    }
}
fn affect_ghost(g: &mut Ghost, c: char, x: usize) {
    if c == '!' {g.powerless_timer += 15;}
    if c == 'g' && x == g.current_position {g.view = '^';}
}
fn find_ghosts_positions(mut f: Iter<u8>) -> (usize, usize, usize) {
    let gx = f.position(|&c| c as char == 'G').unwrap();
    let gy = f.position(|&c| c as char == 'G').unwrap() + gx + 1;
    let gz = f.position(|&c| c as char == 'G').unwrap() + gy + 1;
    (gx, gy, gz)
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
G,,,,,,,,|||!..........,,,,,,,,|||.........G\
...|||||.................|||||..............\
.......|||,,,..|||...........|||,,,..|||....\
||.,,,,.....||||.G.....|||,,,....!||||......\
..,,,,,,,,,|||..........,,,,,,,|||!.........\
...|||||...............|||||||..............\
.......|||,,,..|||...........|||,,,..|||....\
|||||,,,...!||||......||||,,,.....||||......\
..............|||||.........................\
||.|||||...............|||||||..!...........\
.......|||,,,..|||...........|||,,,..|||....\
||||,,,....!||||......||||,,,.....||||......\
..............!...........................*.";
