
// the parameters of the bodies
#[derive(Debug)]
pub struct Parameters {
    name: String::from(..), 
    position: [f64; 3],
    velocity: [f64; 3],
    acceleration: [f64; 3],
    mass: f64,
}

// implementation of the kick and drift verlet integration method
impl Parameters {

    pub fn pos_vec_acc (&mut self, dt: f64) -> mut [f64; 3] {
        let mut position = self.position;
        let mut velocity = self.velocity;
        let mut acceleration = self.acceleration;
        let mass = self.mass;

        position, velocity, acceleration
    }

    pub fn calculating_force(&mut self, position: &mut [f64; 3], velocity: &mut [f64; 3], acceleration: &mut [f64; 3], dt: f64, G: f64, other: [Parameters; 8]) {
        let distance: [f64; 3];
        let direction: [f64; 3];

        for i in 0..3 {
                distance[i] = (other.position[i]).powf(2.0) - (position[i]).powf(2.0);
                direction[i] = - position[i]/distance[i];
                acceleration[i] = direction[i] * (G * other.mass) / distance[i];
        }
        acceleration
    }

    pub fn kick_and_drift(&mut self, position: &mut [f64; 3], velocity: &mut [f64; 3], acceleration: &mut [f64; 3], dt: f64) {
        let mut half_velocity: [f64; 3] = [0.0; 3];
        let mut next_position: [f64; 3] = [0.0; 3];
        let mut next_acceleration: [f64; 3] = [0.0; 3];
        let mut next_velocity: [f64; 3] = [0.0; 3];
        
        for i in 0..3 {    
            half_velocity[i] = velocity[i] + ((acceleration[i] * dt)/2.0);
            next_position[i] = position[i] + half_velocity[i] * dt;
            next_acceleration[i] = (half_velocity[i] - velocity[i])/(dt/2.0);
            next_velocity[i] = half_velocity[i] + ((next_acceleration[i] * dt)/2.0);
        }

        *velocity = next_velocity;
        *acceleration = next_acceleration;
        *position = next_position;

        (velocity, acceleration, position)
    }
}
