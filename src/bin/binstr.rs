use std::time::Instant;


pub fn main() {
    let n = std::env::args()
            .nth(1)
            .map(|arg| arg.parse::<usize>().expect("Número inválido"))
            .expect("Esperava um argumento");

    let now = Instant::now();
    for i in 0..(2usize.pow(n as u32)) {
        println!("{i:0>n$b}")
    }
    println!("Tempo: {:?}", now.elapsed());
}
