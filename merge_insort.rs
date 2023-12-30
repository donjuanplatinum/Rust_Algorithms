fn merge(a: &[i32], p:usize ,q: usize, r: usize ) -> Vec<i32> {
    let n1 = q - p + 1;
    println!("this is n1={}",n1);
    let n2 = r - q ;
    println!("this is n2={}", n2);
    let b  = &a[p..=q];
    let c  = &a[q+1..=r];
    println!("this is b {:?}", b);
    println!("this is c {:?}", c);
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut d: Vec<i32> = vec!();
    for k in p..=r {
        println!("this is k={}", k);
        println!("this is i = {} j = {}",i, j);
        println!("this is b[i] and c[j] {} {}", b[i], c[j]);
        if k == r - 1  {
            if b[i] >= c[j] {
                d.push(c[j]);
                d.push(b[i]);
            }
            else {
                d.push(b[i]);
                d.push(c[j]);
            }
        break;
        } else if b[i] >= c[j] {
            d.push(c[j]);
            if j < n2  {
                j += 1;
            } else {i += 1;}
        } else {
            d.push(b[i]);
            if i < n1 {
                i += 1;
            } else {j += 1;}
        }
    }
    println!("this is vector d ={:?}", d);
    d
}

fn main(){
    let a: [i32;10] = [1,2,3,4,5,2,3,4,5,6];
    let p: Vec<i32> = merge(&a, 0, 4, 9);
    assert_eq!(p,[1,2,2,3,3,4,4,5,5,6]);
}


//array:  1 1 1 1 1 | 2 4 5 | 2 1 3 6 2
//index:  0 1 2 3 4   5 6 7   8 9 10 11 12
//
