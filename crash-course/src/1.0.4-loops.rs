//loop infinito ou até atingir uma condição, interessante a forma do loop bem enxuta, break igual ao do java
let mut a = 0;
loop {
    if a == 5 {
        break;
    }
    println!("{:?}", a);
    a = a + 1;
}

//whilezinho
let mut a = 0;
while a != 5 {
    println!("{:?}", a);
    a = a + 1;
}
