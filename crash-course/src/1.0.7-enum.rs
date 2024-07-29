//enum igual do java
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//o parametro go é declarada como sendo do tipo enum Direction
//os dois pontos serve para acessar as variantes do enum
fn which_way(go: Direction) {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}
