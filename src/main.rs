#[derive(Debug, Copy, Clone)]
enum Pin {
    A,
    B,
    C,
}
fn move_tower(height: u32, start: Pin, finish: Pin, temp: Pin) {
    if height == 1 {
        println!("{:?} to {:?}", &start, &finish);
    } else {
        move_tower(height - 1, start, temp, finish);
        println!("{:?} to {:?}", &start, &finish);
        move_tower(height - 1, temp, finish, start);
    }
}
fn main() {
    let height = 5;
    println!("The tower with {} stories is being moved", &height);
    move_tower(height, Pin::A, Pin::B, Pin::C);
}
