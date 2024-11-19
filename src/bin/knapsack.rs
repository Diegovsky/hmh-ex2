use std::{
    fs::File,
    io::{self, BufRead, BufReader}, time::Instant,
};

use bitvec::prelude::*;
use ex1::Weight;
use ex2::{Args, Mode};

#[derive(Debug, Clone, Copy)]
pub struct Item {
    weight: Weight,
    value: Weight,
}

fn read_knapsack(args: &Args) -> io::Result<(Weight, Vec<Item>)> {
    let file = args.open_file();
    let mut nums = vec![];
    for line in file.lines() {
        // adiciona todos os números no vetor `nums`
        nums.extend(
            line?
                .split_whitespace()
                .map(|num| num.parse::<Weight>().expect("Número inválido")),
        )
    }

    // Remove primeira linha
    let info = nums.drain(0..=1).collect::<Vec<_>>();
    let weight = info[1];
    Ok((
        weight,
        nums.chunks_exact(2)
            .map(|pair| Item {
                value: pair[0],
                weight: pair[1],
            })
            .collect(),
    ))
}

#[derive(Clone, Default)]
struct Solution {
    items: BitVec,
}

impl Solution {
    fn initial(k: usize) -> Self {
        let mut items = BitVec::repeat(false, k);
        items.set(0, true);
        Self {
            items
        }
    }
    fn evaluate(&self, items: &[Item]) -> f64 {
        self.items
            .iter_ones()
            .map(|index| &items[index])
            .map(|i| i.value as f64 / i.weight as f64)
            .sum()
    }
    fn total_value(&self, items: &[Item]) -> Weight {
        self.items
            .iter_ones()
            .map(|index| items[index].value)
            .sum()
    }
    fn total_weight(&self, items: &[Item]) -> Weight {
        self.items
            .iter_ones()
            .map(|index| items[index].weight)
            .sum()
    }
    fn flip(&mut self, index: usize) {
        let val = !self.items[index];
        self.items.set(index, val);
    }
}

impl std::fmt::Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.items)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv();
    let (maxw, items) = read_knapsack(&args)?;
    let k = items.len();
    assert_ne!(k, 0);

    let mut best_solution: Solution = Solution::initial(k);
    let mut best_solution_worth = best_solution.evaluate(&items);

    println!("Solução inicial: {best_solution:?}\nValor: {}", best_solution.total_value(&items));
    let now = Instant::now();
    for i in 0..args.iter_steps {
        let mut improved = false;
        for a in 1..k {
            let mut new_solution = best_solution.clone();
            new_solution.flip(a);

            let new_worth = new_solution.evaluate(&items);
            if new_solution.total_weight(&items) <= maxw && new_worth >= best_solution_worth {
                best_solution = new_solution;
                best_solution_worth = new_worth;
                improved = true;
                // Procura a primeira melhoria
                if args.mode == Mode::First {
                    break;
                }
                // Caso contrário, varre todas em busca da melhor
            }
        }
        // Se não houve mudança, desiste mais cedo
        if !improved {
            println!("Desisto na {i}ª vizinhança");
            break;
        }
    }
    println!("Solução final: {best_solution:?}\nValor: {}", best_solution.total_value(&items));
    println!("Tempo de execução: {:?}", now.elapsed());
    Ok(())
}
