fn main() {
    show_result_display(sum(10, 5));

    show_result_debbug(sum(100, 50));
}

fn sum(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

fn show_result_display(result: i32) {
    print!("{result} \n");
}

fn show_result_debbug(result: i32) {
    print!("{:?}\n", result);
}
