fn main() {
    for i in (0..10).map(|x| x as f64 * 0.1) {
        println!("{:.1}", i);
    }
}