
// the parameters of the bodies
#[derive(Debug)]
pub struct Parameters { 
    position: [f64; 3],
    velocity: [f64; 3],
    acceleration: [f64; 3],
    mass: f64,
}

// implementation of the kick and drift verlet integration method
impl Parameters {

    pub fn pos_vec_acc (&mut self, dt: f64) -> [f64; 3] {
        let mut position: [f64; 3] = self.position;
        let mut velocity: [f64; 3] = self.velocity;
        let mut acceleration: [f64; 3] = self.acceleration;
        let mass: f64 = self.mass;

        position, velocity, acceleration
    }

    pub fn calculating_force(position: &mut [f64; 3], velocity: &mut [f64; 3], acceleration: &mut [f64; 3], dt: f64, G: f64, other: [Parameters; 8]) {
        let distance: [f64; 3];
        let direction: [f64; 3];

        (for i in 0..3 {
                distance = ((other.position[i]).powf(2.0) - (position[i]).powf(2.0)).sqrt();
                direction[i] = - position[i]/distance[i];
                acceleration[i] = direction[i] * (G * other.mass) / (distance[i]).powf(2.0);
        });
        *acceleration;
    }

    pub fn kick_and_drift(position: &mut [f64; 3], velocity: &mut [f64; 3], acceleration: &mut [f64; 3], dt: f64) {
        let half_velocity: [f64; 3];
        let next_position: [f64; 3];
        let next_acceleration: [f64; 3]; 
        let next_velocity: [f64; 3];
        
        for i in 0..3 {    
            let half_velocity[i] = velocity[i] + ((acceleration[i] * dt)/2.0);
            let next_position[i] = position[i] + half_velocity[i] * dt;
            let next_acceleration[i] = (half_velocity[i] - velocity[i])/(dt/2.0);
            let next_velocity[i] = half_velocity[i] + ((next_acceleration[i] * dt)/2.0);
        }

        *velocity = next_velocity;
        *position = next_position;
    }
}
