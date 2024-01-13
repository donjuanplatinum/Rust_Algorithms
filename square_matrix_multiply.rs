fn square_matrix_multiply<T, const Q: usize>(A: (usize, [T; Q]), B: (usize, [T; Q])) -> (usize, [T; Q])
where
    T: std::ops::Add<Output = T> + Copy + Default  + std::ops::Mul<Output = T>,
{
    let n = A.0;
    let mut C = (n, [T::default(); Q]);

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C.1[i * n + j] = C.1[i * n + j] + A.1[i * n + k] * B.1[k * n + j];
            }
        }
    }
    C
}

fn main() {
    
    let matrix = [1,2,3,4,5,6,7,8,9];
    let A = (3,matrix);
    let B = (3,matrix);
    let C =square_matrix_multiply(A,B);
    println!("matrix is {:?}", C);
}
