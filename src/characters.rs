use alg;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct Hero { pub current_position: usize }

impl Hero {
    pub fn new(current_position: usize) -> Self {
        Hero { current_position: current_position }
    }
    pub fn can_step(&self, x: i32, w: &Vec<u8>, step: usize) -> bool {
        let cur = self.current_position;
        if x == 0 {return false;}
        if x == 1 && cur % step == step-1 {return false;}
        if x == -1 && cur % step == 0 {return false;}
        let curi32 = cur as i32;
        if x + curi32 < 0 || x + curi32 > (w.len()-1) as i32 {return false;}
        if w[(x+curi32) as usize] == '|' as u8 {return false;}
        true
    }
    pub fn render(&mut self, world: &mut Vec<u8>, new_position: usize) {
        world[new_position] = '*' as u8;
        world[self.current_position] = '.' as u8;
        self.current_position = new_position;
    }
}

pub struct Ghost<'r> {
    pub current_position: usize,
    pub start_position: usize,
    pub view: char,
    pub previous_step: (usize, u8),
    pub powerless_timer: usize,
    min_paths: &'r Vec<Vec<usize>>,
    weights: &'r Vec<Vec<usize>>,
    speed: usize
}

impl<'r> Ghost<'r> {
    pub fn new(start: usize, v: usize, minp: &'r Vec<Vec<usize>>, ws: &'r Vec<Vec<usize>>) -> Self {
        Ghost {
            current_position: start.clone(),
            start_position: start.clone(),
            previous_step: (start, '.' as u8),
            view: 'G',
            powerless_timer: 0,
            min_paths: minp,
            weights: ws,
            speed: v
        }
    }
    pub fn can_step(&self, w: &Vec<u8>, dsts: Vec<usize>) -> Option<usize> {
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
    pub fn can_hunt(&self) -> bool { self.powerless_timer == 0 }
    pub fn make_stronger(&mut self) { self.powerless_timer -= 1; }
    pub fn is_moveable(&self, time: usize) -> bool { time % self.speed == 0 }

    pub fn escape_or_default(&self, x: usize) -> Vec<usize> {
        let between = Range::new(0, 100);
        let mut rng = rand::thread_rng();
        let num = between.ind_sample(&mut rng);
        if num < 75 { vec![self.start_position, x] }
        else { vec![x, self.start_position] }
    }
    pub fn render(&mut self, w: &mut Vec<u8>, new_position: usize) {
        self.render_previous_step(w, new_position);
        self.render_current_step(w, new_position);
    }
    fn render_previous_step(&mut self, w: &mut Vec<u8>, new_position: usize) {
        // TODO can through away self.previous_step?
        w[self.previous_step.0] = self.previous_step.1;
        let mut c = w[new_position] as char;
        c = if c == '*' {'.'} else {c};
        self.previous_step.1 = c as u8;
        self.previous_step.0 = new_position;
    }
    fn render_current_step(&mut self, w: &mut Vec<u8>, new_position: usize) {
        let x = w[new_position] as char;
        let c = if self.can_hunt() {'G'} else {
            if x == '*' || self.view == '^' {'^'} else {'g'}
        };
        w[new_position] = c as u8;
        self.view = c;
        self.current_position = new_position;
    }
}
