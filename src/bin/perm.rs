use std::time::Instant;

fn perm(k: usize, elements: &mut [usize], print: impl Fn(&[usize]) + Copy) {
    if k <= 1 {
        print(elements);
        return;
    }

    perm(k - 1, elements, print);
    for i in 0..(k - 1) {
        if k % 2 == 0 {
            elements.swap(i, k - 1);
        } else {
            elements.swap(0, k - 1);
        }
        perm(k - 1, elements, print);
    }
}

pub fn main() {
    let n = std::env::args()
        .nth(1)
        .map(|arg| arg.parse::<usize>().expect("Número inválido"))
        .expect("Esperava um argumento");
    let mut elements = (1..=n).collect::<Vec<usize>>();
    let width = (elements.iter().copied().max().unwrap() as f64)
        .log10().floor() as usize + 2;
    let now = Instant::now();
    perm(n, &mut elements, move |elements| {
        println!("{}", elements.iter()
            .copied()
            .map(|x| format!("{x: <width$}"))
            .collect::<Vec<String>>()
            .concat())
    });
    println!("Tempo: {:?}", now.elapsed());
}
