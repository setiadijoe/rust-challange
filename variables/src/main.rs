const STARTING_MISSILE: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32)= (STARTING_MISSILE, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);

}
