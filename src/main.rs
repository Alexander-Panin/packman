mod alg;
mod world;
mod characters;
mod socket;
use std::time::Duration;
use std::thread;
use std::str;
use std::cmp::min;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

const STEP: usize = 22;
#[allow(non_camel_case_types)]
type world_t = Vec<u8>;
#[allow(non_camel_case_types)]
type matrix_t = Vec<Vec<usize>>;

struct Hero { current_position: usize }

impl Hero {
    fn new(current_position: usize) -> Self {
        Hero { current_position: current_position }
    }
    fn can_step(&self, diff: i32, w: &world_t) -> bool {
        can_step(diff, self.current_position as i32, w)
    }
    fn render(&mut self, world: &mut world_t, new_position: usize) {
        world[new_position] = '*' as u8;
        world[self.current_position] = '.' as u8;
        self.current_position = new_position;
    }
}

struct Ghost<'r> {
    current_position: usize,
    start_position: usize,
    view: char,
    previous_step: (usize, u8),
    powerless_timer: usize,
    min_paths: &'r matrix_t,
    weights: &'r matrix_t,
    speed: usize
}

impl<'r> Ghost<'r> {
    fn new(start: usize, v: usize, minp: &'r matrix_t, weights: &'r matrix_t) -> Self {
        Ghost {
            current_position: start.clone(),
            start_position: start.clone(),
            previous_step: (start, '.' as u8),
            view: 'G',
            powerless_timer: 0,
            min_paths: minp,
            weights: weights,
            speed: v
        }
    }
    fn can_step(&self, w: &world_t, dsts: Vec<usize>) -> Option<usize> {
        let path = |x: usize, y: usize| {
            alg::min_path_n(&self.min_paths,
                            &self.weights,
                            x, y, self.min_paths[x][y].clone(), 2)[1]
        };
        let pred = |c: char| c == 'g' || c == 'G' || c == '^';
        dsts.iter()
            .map(|&x: &usize| path(self.current_position, x))
            .find(|&x: &usize| !pred(w[x] as char))
    }
    fn can_hunt(&self) -> bool { self.powerless_timer == 0 }
    fn make_stronger(&mut self) { self.powerless_timer -= 1; }
    fn is_moveable(&self, time: usize) -> bool { time % self.speed == 0 }

    fn escape_or_default(&self, x: usize) -> Vec<usize> {
        let between = Range::new(0, 100);
        let mut rng = rand::thread_rng();
        let num = between.ind_sample(&mut rng);
        if num < 75 { vec![self.start_position, x] }
        else { vec![x, self.start_position] }
    }
    fn render_previous_step(&mut self, w: &mut world_t, new_position: usize) {
        // TODO can through away self.previous_step?
        w[self.previous_step.0] = self.previous_step.1;
        let mut c = w[new_position] as char;
        c = if c == '*' {'.'} else {c};
        self.previous_step.1 = c as u8;
        self.previous_step.0 = new_position;
    }
    fn render_current_step(&mut self, w: &mut world_t, new_position: usize) {
        let x = w[new_position] as char;
        let c = if self.can_hunt() {'G'} else {
            if x == '*' || self.view == '^' {'^'} else {'g'}
        };
        w[new_position] = c as u8;
        self.view = c;
        self.current_position = new_position;
    }
    fn render(&mut self, w: &mut world_t, new_position: usize) {
        self.render_previous_step(w, new_position);
        self.render_current_step(w, new_position);
    }
}

fn main() {
    let mut w: world_t = world::make(W);
    let weights = world::positions_graph(&w, STEP, w.len());
    let min_paths = alg::power(weights.clone(), weights.len()-1,
        |x: &alg::Matrix<usize>, y: &alg::Matrix<usize>|
            alg::mmult(x, y,
                |&x: &usize, &y: &usize| min(x, y),
                |&x: &usize, &y: &usize| x+y));

    let mut hero = Hero::new(w.iter().position(|&c| c as char == '*').unwrap());
    let gx; let gy;
    {
        let mut f = w.iter();
        gx = f.position(|&c| c as char == 'G').unwrap();
        gy = f.position(|&c| c as char == 'G').unwrap() + gx + 1;
    }
    let mut ghostx = Ghost::new(gx, 31, &min_paths, &weights);
    let mut ghosty = Ghost::new(gy, 29, &min_paths, &weights);

    let to_ = "127.0.0.1:41235"; let from_ = "127.0.0.1:41234";
    let socket_ = socket::One::new(from_, to_);
    let mut time_: usize = 0;

    loop {
        let step = match socket_.recv() { Some(v) => act(v), None => 0 };

        if let Some((position, c)) = hero_move(&mut hero, step, &mut w) {
            affect_ghost(&mut ghostx, c, position);
            affect_ghost(&mut ghosty, c, position);
        }
        ghost_move(&mut ghostx, time_, ghosty.start_position, &hero, &mut w);
        ghost_move(&mut ghosty, time_, ghostx.start_position, &hero, &mut w);

        socket_.send(&world::render(&w, &STEP));
        thread::sleep(Duration::from_millis(10));
        time_+=1;
    }
}

fn hero_move(hero: &mut Hero, x: i32, mut w: &mut world_t) -> Option<(usize, char)> {
    if !hero.can_step(x, w) {return None;}
    let new_position = (hero.current_position as i32 + x) as usize;
    let c = w[new_position] as char;
    hero.render(w, new_position);
    Some((new_position, c))
}

fn affect_ghost(g: &mut Ghost, c: char, x: usize) {
    if c == '!' {g.powerless_timer += 15;}
    if c == 'g' && x == g.current_position {g.view = '^';}
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

fn can_step(x: i32, cur: i32, w: &world_t) -> bool {
    if x == 0 {return false;}
    if x == 1 && cur as usize % STEP == STEP-1 {return false;}
    if x == -1 && cur as usize % STEP == 0 {return false;}
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
G-,,,,,,,---.........G\
.-.-----..!...........\
.......---,,,..---....\
--.-,,,....!----......\
..............!.....*.";

