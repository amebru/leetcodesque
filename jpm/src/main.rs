use rand::seq::SliceRandom;
use indicatif::{ProgressBar, ProgressStyle};

fn simulate_amoebas(amoebas: u32, minutes_left: u32) -> u32 {
    if minutes_left == 0 {
        return amoebas;
    } else {
        if amoebas == 0 {
            return 0;
        } else {
            let mut _amoebas: u32 = amoebas;
            for _ in 0..amoebas {
                let choices = vec![-1, 0, 1, 2];
                let mut rng = rand::thread_rng();
                let choice = choices.choose(&mut rng).unwrap();
                _amoebas = (choice + (_amoebas as i32)) as u32;
            }
            return simulate_amoebas(_amoebas, minutes_left - 1);
        }
    }
}

fn main() {
    const NUM_SIMULATIONS: u32 = 1000;
    const SIMULATION_MINUTES: u32 = 25;
    
    // Create a new progress bar
    let pb = ProgressBar::new(NUM_SIMULATIONS.into());
    
    // Optional: Set a nice style for the progress bar
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap()
        .progress_chars("#>-"));
    
    // Collect simulations with progress updates
    let simulations: Vec<u32> = (0..NUM_SIMULATIONS)
        .map(|_| {
            // Run the simulation
            let result = simulate_amoebas(1, SIMULATION_MINUTES);
            
            // Update the progress bar
            pb.inc(1);
            
            result
        })
        .collect();
    
    // Finish the progress bar
    pb.finish_with_message("All simulations complete!");
    
    // Calculate probability
    let extinctions = simulations.iter().filter(|&s| *s == 0).count();
    let prob: f32 = extinctions as f32 / simulations.len() as f32;
    
    println!("Extinction probability = {:.4}", prob);
    println!("({} extinctions out of {} simulations)", extinctions, simulations.len());
}
