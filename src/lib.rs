pub mod lab3 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
    use std::thread;
    use std::time::Duration;
    use rayon::prelude::*;

    pub fn proceed_seq(array: &Vec<u64>) -> u64{
        let mut acc: u64 = 0;
        for i in 0..array.len() {
            if array[i] > 10 {
                acc += array[i];
            }
        }
        return acc;
    }

    pub fn proceed_par_crossbeam(array: &Vec<u64>, thread_num: usize) -> u64 {
        let acc = AtomicU64::new(0);
        crossbeam::scope(|s| {
            let chunk_size = array.len() / thread_num;
            for slice in array.chunks(chunk_size) {
                s.spawn(|_|{
                    for i in 0..slice.len() {
                        if slice[i] > 10 {
                            acc.fetch_add(slice[i], Ordering::Relaxed);
                        }
                    }
                });
            }
        });
        return acc.load(Ordering::Relaxed);
    }

    pub fn proceed_par_rayon(array: &[u64]) -> u64 {
        return array.par_iter().filter(|&&x| x > 10).sum();
    }


    pub fn proceed_par_arc(array: Arc<Vec<u64>>, thread_num: usize) -> u64 {
        let acc = Arc::new(AtomicU64::new(0));
        let mut threads = vec![];

        for i in 0..thread_num {
            let begin = (array.len() * i) / thread_num;
            let end = ((array.len() * (i + 1)) / thread_num) - 1;
            let array_cloned = Arc::clone(&array);
            let mut acc_cloned = Arc::clone(&acc);
            threads.push(thread::spawn(move || {
                for num in begin..end+1 {
                    if array_cloned[num] > 10 {
                        acc_cloned.fetch_add(array_cloned[num], Ordering::Relaxed);
                    }
                }
            }))
        }
        for t in threads {
            t.join();
        }
        return acc.load(Ordering::Relaxed)
    }

    pub fn proceed_par_arc_comp_ex(array: Arc<Vec<u64>>, thread_num: usize) -> u64 {
        let acc = Arc::new(AtomicU64::new(0));
        let mut threads = vec![];

        for i in 0..thread_num {
            let begin = (array.len() * i) / thread_num;
            let end = ((array.len() * (i + 1)) / thread_num) - 1;
            let array_cloned = Arc::clone(&array);
            let mut acc_cloned = Arc::clone(&acc);
            threads.push(thread::spawn(move || {
                for num in begin..end+1 {
                    if array_cloned[num] > 10 {
                        let mut stored = acc_cloned.load(Ordering::Relaxed);
                        loop {
                            match acc_cloned.compare_exchange(stored, stored + array_cloned[num], Ordering::SeqCst, Ordering::Relaxed) {
                                Ok(_) => break,
                                Err(x) => stored = x
                            };
                        }
                    }
                }
            }))
        }
        for t in threads {
            t.join();
        }
        return acc.load(Ordering::Relaxed)
    }

}


