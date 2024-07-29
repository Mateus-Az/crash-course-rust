//vectores podem armazenas multiplos tipos de dados, possui alguns métodos como len, push e pop

struct Test {
    score: i32,
}

fn main() {
    //macro vec
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 93 },
    ];

    for test in my_scores {
        println!("score = {}", test.score);
    }
}
