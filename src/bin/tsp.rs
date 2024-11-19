use std::{default, fs::File, io::BufReader, time::Instant};

use ex1::{Graph, GraphMat, Node, Weight};
use ex2::{Args, Mode};

fn solution_value(solution: &[Node], g: &dyn Graph) -> Weight {
    let first = *solution.first().unwrap();
    let last = *solution.last().unwrap();
    solution
        .windows(2)
        .map(|window| g.get_edge_weight(window[0], window[1]).unwrap())
        .sum::<Weight>()
        + g.get_edge_weight(last, first).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv();
    let mut file = args.open_file();
    let mut graph = GraphMat::default();
    ex2::fill_tsp_graph(&mut file, &mut graph)?;

    let k = graph.node_count() as Node;

    // Solução inicial consiste em nós em órdem crescente
    let mut best_solution: Vec<Node> = (0..k).collect();
    let mut best_solution_worth = solution_value(&best_solution, &graph);
    println!("Solução inicial: {best_solution:?}\nValor: {best_solution_worth}");

    let now = Instant::now();
    for i in 0..args.iter_steps {
        let mut improved = false;
        for a in 1..(k as usize) {
            for b in 0..a {
                let mut new_solution = best_solution.clone();
                new_solution.swap(a, b);
                let new_worth = solution_value(&new_solution, &graph);
                if new_worth < best_solution_worth {
                    best_solution = new_solution;
                    best_solution_worth = new_worth;
                    improved = true;
                    // Procura a primeira melhoria
                    if args.mode == Mode::First {
                        break
                    }
                    // Caso contrário, varre todas em busca da melhor
                }
            }
        }
        // Se não houve mudança, desiste mais cedo
        if !improved {
            println!("Desisto na {i}ª vizinhança");
            break
        }
    }
    println!("Solução final: {best_solution:?}\nValor: {best_solution_worth}");
    println!("Tempo de execução: {:?}", now.elapsed());
    Ok(())
}
