pub fn add<T, const Q: usize>(A: (usize, [T; Q]), B: (usize, [T; Q])) -> (usize, [T; Q])
where
    T: std::ops::Add<Output = T> + Copy + Default ,
{
    let n = A.0;
    let mut C = (n, [T::default(); Q]);

    for i in 0..n {
        for j in 0..n {
                C.1[i * n + j] = A.1[i * n + j] + B.1[i * n + j];
        }
    }
    C
}

pub fn multiply<T, const Q: usize>(A: (usize, [T; Q]), B: (usize, [T; Q])) -> (usize, [T; Q])
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


