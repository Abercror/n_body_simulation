extern crate vsop87;
mod integration;
mod bodies;

const G: f64 = 6.67428e-11;


fn main() {
    let planet_parameters: [integration::Parameters; 8] = bodies::planets();
    let other: [T; 8] = ["sun", "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune"];
    let dt: f64 = 86400.0;
    let steps: f64 = 10000.0;
    let mut completed_steps: f64 = 0.0;

    while completed_steps < steps {
        for i in 0..8 {
            let (position[i], velocity[i], acceleration[i]) = integration::pos_vec_acc(dt);
            (if other[i] != planet_parameters[i] {
                integration::calculating_force(&mut position, &mut velocity, &mut acceleration, dt, G, other);
                integration::kick_and_drift(&mut position, &mut velocity, &mut acceleration, dt);
            }
            else
            {});
            completed_steps += 1.0
        }

        println!("planet: {:?}", planet_parameters);
    }
    
    println!("Simulation complete");
}
