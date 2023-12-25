// benchmarks.rs

// filedate::benchmarks::benchmark_500k();    

pub mod benchmarks {
    use std::time::{Instant};
    use filedate::stamp::stamp_str;  // imports the lib.rs namespace, so we can call its functions.

    #[allow(dead_code)]
    pub fn benchmark_500k() {
        const FILENAME: &str = "some_file_name.txt";

        #[allow(unused_variables)]
        let mut x = 0;
        let start = Instant::now();  // capture starttime
        while x < 500000 {
            self::stamp::add_suffix(FILENAME, &None);
            x = x + 1;
        }
        let duration = start.elapsed();
        println!("Number of iterations: {}", x);
        println!("Total duration: {:?}", duration);
        println!("Average duration: {:?}", duration / x);
    }
}