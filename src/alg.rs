pub type Matrix<T> = Vec<Vec<T>>;
pub type Integral = usize;

fn get_in<T>(x: &Matrix<T>, i: usize, j: usize) -> &T {
    unsafe { x.get_unchecked(i).get_unchecked(j) }
}

pub fn mmult<T, Add, Mult>(a: &Matrix<T>, b: &Matrix<T>, add: Add, mult: Mult) -> Matrix<T>
    where T: Default+Clone,
          Add: Fn(&T, &T) -> T,
          Mult: Fn(&T, &T) -> T {
    let n = a.len();
    let mut x: Matrix<T> = vec![Vec::with_capacity(n); n];
    for k in 0..n {
        for j in 0..n {
            let mut s = mult(get_in(a, j, 0), get_in(b, 0, k));
            for i in 1..n {
                s = add(&s, &mult(get_in(a, j, i), get_in(b, i, k)));
            }
            unsafe { x.get_unchecked_mut(j).push(s); };
        }
    }
    x
}

pub fn power<T, F>(mut x: T, mut n: Integral, op: F) -> T
    where T: Clone,
          F: Fn(&T, &T) -> T {

    while n & 1 == 0 {
        x = op(&x, &x);
        n >>= 1;
    }

    n -= 1;
    let mut res = x.clone();
    while n != 0 {
        x = op(&x, &x);
        n >>= 1;
        if n & 1 != 0 {
            res = op(&res, &x); n -= 1;
        }
    }
    res
}

#[allow(dead_code)]
pub fn min_path(x: &Matrix<Integral>, y: &Matrix<Integral>,
    i: Integral, j: Integral, path: Integral) -> Vec<Integral> {
    min_path_n(x, y, i, j, path, x.len())
}

pub fn min_path_n(x: &Matrix<Integral>, y: &Matrix<Integral>,
    i: Integral, j: Integral, path: Integral, n: Integral) -> Vec<Integral> {
    if n == 0 { return vec![]; }
    let mut c = 0; let mut v = vec![];
    {
        let pred = |i: &Integral| {
            if c != n { v.push(i.clone()); c+=1; true }
            else { false }
        };
        min_path_pred(x, y, i, j, path, pred);
    }
    if c != n { v.push(j); }
    v
}

pub fn min_path_pred<Predicate>(x: &Matrix<Integral>, y: &Matrix<Integral>,
    mut i: Integral, j: Integral, mut p: Integral, mut pred: Predicate) -> ()
    where Predicate: FnMut(&Integral) -> bool {
    let n = x.len(); let mut k = 0;
    loop {
        while k != n {
            let val = x[k][j];
            if val < p && x[i][k] == p-val && val != 0 && y[i][k] == p-val {
                break;
            }
            k += 1;
        }
        if !pred(&i) || k == n { break; }
        i = k; p = x[k][j]; k = 0;
    }
}
