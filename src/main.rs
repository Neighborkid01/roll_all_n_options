use std::collections::HashMap;
use rand::Rng;

fn main() {
    let dice_faces = vec![4, 6, 8, 10, 12, 20, 100];
    let mut die_roll_avgs: HashMap::<usize, f64> = HashMap::new();

    for num_die_faces in dice_faces {
        let sample_size = 100000;
        let mut faces_rolled = vec![false; num_die_faces];
        let mut rolls_needed = 0;
        let mut rolls_needed_hash: HashMap::<usize, usize> = HashMap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..sample_size {
            while faces_rolled.iter().any(|&x| x == false) {
                let roll = rng.gen_range(0..num_die_faces);
                faces_rolled[roll] = true;
                rolls_needed += 1;
            }

            *rolls_needed_hash.entry(rolls_needed).or_insert(0) += 1;
            faces_rolled = vec![false; num_die_faces];
            rolls_needed = 0;
        }

        let one_through_n = (1..=num_die_faces).collect::<Vec<usize>>();
        let one_over_k = one_through_n.iter().fold(0.0, |acc, &k| acc + (1.0/k as f64));
        let die_roll_avg = rolls_needed_hash.iter().fold(0, |acc, (k, v)| acc + k * v) as f64 / sample_size as f64;
        println!("{} expected: {}, got: {}", num_die_faces,  num_die_faces as f64 * one_over_k, die_roll_avg);
        die_roll_avgs.insert(num_die_faces, die_roll_avg);
    }
}
