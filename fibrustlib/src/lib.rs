
pub fn fib_sec(n: usize) -> Vec<u64> {
    let mut fibsec: Vec<u64> = vec![0, 1];

    while fibsec.len() < n {
        let mut num = fibsec[fibsec.len() - 1] ;
         num = num +  fibsec[fibsec.len() - 2];
        fibsec.push(num);
    }

      return fibsec
}
