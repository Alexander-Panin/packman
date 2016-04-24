use alg;
pub type Integral = usize;

pub fn go(w: &mut [u8], i: Integral, j: Integral) { w.swap(j,i); }

pub fn next_step(x: &Vec<Vec<Integral>>, y: &Vec<Vec<Integral>>,
    ghost: Integral, hero: Integral) -> Integral {
    alg::min_path_n(&x, &y, ghost, hero, x[ghost][hero].clone(), 2)[1]
}
