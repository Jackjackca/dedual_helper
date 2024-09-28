use crate::articles::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};
#[allow(unused)]
fn vec_push_bench() {
    let mut n = 0;
    let cycle = 1_000_000;
    let expect_exe_time_secs = Duration::from_secs(10);
    let mut vec1 = Vec::new();
    let mut vec_op_time = Vec::new();
    let now = Instant::now();
    let pb = ProgressBar::new(cycle);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message("Processing");
    while n < cycle {
        thread::sleep(Duration::from_secs(expect_exe_time_secs.as_secs() / cycle));
        pb.inc(1);
        vec1.push(n);
        vec_op_time.push(now.elapsed());
        n = n + 1;
    }
    let avg_op_time = vec_op_time.iter().sum::<Duration>() / vec1.len() as u32;
    pb.finish_with_message("done.");
    println!("avg_op_time: {:?}", avg_op_time);
    println!("total: {:?}", vec_op_time.iter().sum::<Duration>());
}

#[allow(unused)]
fn test_trait_bound() {
    let tweet = Tweet::new();
    let article = Article::new();
    fn trait_bound_display(ar: impl CustomDisplay) {
        println!("1");
        ar.display();
    }
    trait_bound_display(tweet);
    trait_bound_display(article);
    //CustomDisplay not implemented for struct A
    // let a = A::new();
    //trait_bound_display(a);
}