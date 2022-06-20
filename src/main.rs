mod evaluator;
use std::io::{ stdin };
use evaluator::evaluator_mod;

// Get information on a graph from stdin.
fn get_graph_info(graphvis: bool) -> evaluator_mod::GraphInput {

    let     graph_input:  evaluator_mod::GraphInput;
    let mut buffer:       String = String::new();
    let mut y:            String = String::new();
    const   GRAPHVIS_AMT: i32    = 20;
    let     amt:          i32;

    println!("f(x) = ? ");
    stdin().read_line(&mut y).expect("Failed to read into buffer.");
    buffer.clear();

    // Check if we are printing a visual graph.
    if !graphvis {
        println!("Amount to test?");
        stdin().read_line(&mut buffer).expect("Failed to read into buffer.");
        amt = buffer.trim().parse::<i32>().unwrap();
    }
    else {
        // Graphvis will always test GRAPHVIS_AMT, so there's no need to take input here.
        amt = GRAPHVIS_AMT;
    }

    graph_input = evaluator_mod::GraphInput::new(y, amt);
    println!("Graphing...");
    graph_input
}

// Print beginning information to stdout.
fn print_begin_info(verbose: bool) {
    println!("\n\nType: `quit` to quit the program.");
    println!("Type: `graph` to print `n` number of points on a graph.");
    println!("Type: `graphvis` to print `n` number of points on a graph and draw a visual graph.");
    println!("[NOTE] `graphvis` needs gnuplot installed to function properly.");
    println!("To enable verbose mode, re-run with `-v`");
    println!("Verbose mode enabled? [{}]", verbose);
}

fn main() {
    let mut graph_input: evaluator_mod::GraphInput;
    let mut buffer        = String::new();
    let mut history       = Vec::<f64>::new();
    let args: Vec<String> = std::env::args().collect();
    let mut verbose: bool = false;

    if args.len() == 2 {
        verbose = if &args[1] == "-v" { true } else { false };
    } else if args.len() > 2 {
        println!("Usage: {} <filename.txt>", args[0]);
        std::process::exit(1);
    }

    print_begin_info(verbose);

    while buffer.trim() != "quit" {
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read into buffer.");

        // Print plot points on a graph.
        if buffer.trim() == "graph" {
            graph_input = get_graph_info(false);
            println!("|\nV\n--------------------------------");
            for graph in evaluator_mod::create_graph(&graph_input.get_y(), graph_input.get_amt(), verbose) {
                println!("x: [{}] y:[{}]", graph.get_x(), graph.get_y());
            }
            println!("--------------------------------\n");
            graph_input.clear_y();
        }

        // Draw a visual graph.
        else if buffer.trim() == "graphvis" {
            graph_input = get_graph_info(true);
            println!("|\nV\n--------------------------------\nDrawing...");
            evaluator_mod::draw_graph(evaluator_mod::create_graph(&graph_input.get_y(), graph_input.get_amt(), verbose));
            println!("--------------------------------\n");
            graph_input.clear_y();
        }

        // Anything else is the calculator.
        else {
            println!("|\nV\n--------------------------------");
            history.push(evaluator_mod::parse_equation(&buffer, verbose));
            if history.len() > 0 {
                print!("\nHistory: [ ");
                for i in &history {
                    print!("{} ", i);
                }
                print!("]\n");
            }

            // Print the result to stdout.
            println!("\nResult -> {}\n", history[history.len()-1]);
            println!("--------------------------------\n");
        }
    }
}

