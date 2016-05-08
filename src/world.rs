const MAX: usize = 2048;
pub type Integral = usize;

pub fn positions_graph(w: &Vec<u8>, step: Integral, n: Integral) -> Vec<Vec<Integral>> {
    let mut res = vec![vec![MAX; n]; n];
    mark_weights(&w, &mut res, step.clone(), n);
    remove_rows_mark_errors(&mut res, &step);
    mark_zeros(&mut res);
    res
}

pub fn render(w: &[u8], step: &Integral) -> Vec<u8> {
    let mut x = Vec::new();
    for a in w.chunks(step.clone()) {
        x.extend_from_slice(a);
        x.extend_from_slice(b"<br />");
    }
    x
}

pub fn make(s: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(s); v
}

fn mark_weights(w: &Vec<u8>, m: &mut Vec<Vec<Integral>>, step: Integral, n: Integral) {
    let xs = steps(step as i32, n as i32);
    let wss = weights(&w, &xs);
    for (v,ws) in m.iter_mut().zip(wss) {
        for (i, w) in ws { v[i] = w; }
    }
}

fn remove_rows_mark_errors(m: &mut Vec<Vec<Integral>>, step: &Integral) {
    let n = m.len()-1;
    for i in 1..n {
        if i % step == 0 { m[i][i-1] = MAX; }
        if i % step == step-1 { m[i][i+1] = MAX; }
    }
}
fn mark_zeros(m: &mut Vec<Vec<Integral>>) {
    let n = m.len();
    for i in 0..n { m[i][i] = 0; }
}

fn steps(step: i32, n: i32) -> Vec<Vec<Integral>> {
    (0..n)
        .map(|x|
            vec![x-1, x-step, x+1, x+step].into_iter()
                .filter(|&x| x >= 0 && x < n)
                .map(|x| x as Integral)
                .collect::<Vec<Integral>>())
        .collect::<Vec<Vec<Integral>>>()
}

fn weights(w: &Vec<u8>, steps: &Vec<Vec<Integral>>) -> Vec<Vec<(Integral, Integral)>> {
    w.iter()
        .zip(steps)
        .map(|(&c, xs)|
            xs.iter()
                .map(move |&x|
                    (x, if c as char == '|' { MAX } else { 1 }))
                .collect::<Vec<(Integral, Integral)>>())
        .collect::<Vec<Vec<(Integral, Integral)>>>()
}
