use rand::distributions::Uniform;
use rand::prelude::Distribution;

fn main() {
    let wall_percent = 0.4;
    let grid_x = 8;
    let grid_y = 8;

    let mut rng = rand::thread_rng();
    let uni: Uniform<f32> = Uniform::from(0.0..=1.0);

    for _i in 0..grid_y {
        for _j in 0..grid_x {
            let result: f32 = uni.sample(&mut rng);

            if result < wall_percent {
                print!("{}", '\u{2593}' as char);
            } else {
                print!("{}", '\u{2591}' as char);
            }
            print!("  ");
        }
        println!();
    }
}