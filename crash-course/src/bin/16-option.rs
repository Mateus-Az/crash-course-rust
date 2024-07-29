#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Matt".to_owned(),
        locker: Some(12),
    };

    println!("{:?}", student.name);
    match student.locker {
        Some(num) => println!("{:?}", num),
        None => print!("{:?}", "None value here"),
    }

    println!("{:?}", student);
}
