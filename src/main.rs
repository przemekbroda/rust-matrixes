mod matrix;
mod zero;
mod one;

use crate::matrix::{AddMultiThreaded, Matrix, SubMultiThreaded};

fn main() {
    let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

    let m1 = Matrix::new_with_values(10000, 10000, 5i32);
    let m2 = Matrix::new_with_values(10000, 10000, 5i32);


    /*
    let Ok(result) = m1.clone() + m2.clone() else {
        println!("Error");
        return;
    };

     */

    let threads_num = std::thread::available_parallelism().unwrap();
    println!("threads: {:?}", threads_num);

    for _i in 0..500 {
        let result = m1.sub_multithreaded(&m2, threads_num).unwrap();
        assert_eq!(0, result.get_value(9999, 9999).unwrap());
        assert!(result.get_value(10000, 10000).is_err());
    }

    let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap() - t1;
    println!("duration: {:?}", t2);
}


