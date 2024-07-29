fn main() {
    let mut count = 0;
    loop {
        if count == 4 {
            break;
        }
        count = count + 1;
        print!("{:?}", count)
    }
}
