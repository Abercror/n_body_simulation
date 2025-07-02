extern crate vsop87;
mod integration;
mod bodies;

const G: f64 = 6.67428e-11;


fn main() {
    let planet_parameters: [integration::Parameters; 8] = bodies::planets();
    let dt: f64 = 86400.0;
    let steps: f64 = 10000.0;
    let mut completed_steps: f64 = 0.0;

    while completed_steps < steps {

        

        println!("planet: {:?}", planet_parameters);
    }
    
    println!("Simulation complete");
}
        // for i in 0..8 {
        //     let (position, velocity, acceleration) = integration::pos_vec_acc(dt);
        //     for j in 0..8 {
        //         (if other[j] != planet_parameters[i] {
        //             integration::calculating_force(&mut position, &mut velocity, &mut acceleration, dt, G, other);
        //             integration::kick_and_drift(&mut position, &mut velocity, &mut acceleration, dt);
        //         }
        //         else
        //         {});
        //         completed_steps += 1.
        //     }