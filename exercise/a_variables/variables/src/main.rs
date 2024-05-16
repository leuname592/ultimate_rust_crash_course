const READY_AMOUNT: i32 = 32;
const STARTING_MISSILES: i32 = 8;
fn main() {
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", missiles, ready);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
