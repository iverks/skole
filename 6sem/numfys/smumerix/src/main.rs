mod ex0;

fn main() {
    let res = ex0::probability_distribution();
    let slice = &res[0..10];
    println!("{slice:?}")
}
