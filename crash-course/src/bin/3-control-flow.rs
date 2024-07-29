fn main() {
    show_message(true);
}

fn show_message(condition: bool) {
    if condition {
        print!("{:?}", "hello");
    } else {
        print!("{:?}", "googbye");
    }
}
