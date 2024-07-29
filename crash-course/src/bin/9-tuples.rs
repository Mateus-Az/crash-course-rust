fn main() {
    let (y1, y2) = cartesian_y();
    if y1 == 5 {
        println!("equal");
    } else if y2 > 5 {
        println!("greater");
    } else {
        println!("less")
    }
}

fn cartesian_y() -> (i8, i8) {
    (1, 10)
}
