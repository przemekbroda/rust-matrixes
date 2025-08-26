mod matrix;
mod zero;
mod one;

use crate::matrix::{AddMultiThreaded, Matrix, MulMultiThreaded, SubMultiThreaded};

fn main() { 
    let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

    let m1 = Matrix::new_with_values(1000, 1000, 5i32);
    let m2 = Matrix::new_with_values(1000, 1000, 5i32);

    let threads_num = std::thread::available_parallelism().unwrap();
    println!("threads: {:?}", threads_num);

    let result = m1.mul_multithreaded(&m2, threads_num);

    println!("{:?}", result.unwrap().get_value(0, 0));

    let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap() - t1;
    println!("duration: {:?}", t2);
}


