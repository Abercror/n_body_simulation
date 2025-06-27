const G: f64 = 6.67428e-11;

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

    pub fn pos_vec_acc (&mut self, dt: f64) {
        let mut position = self.position;
        let mut velocity = self.velocity;
        let mut acceleration = self.acceleration;
        let other;
        self.calculating_force(&mut position, &mut velocity, &mut acceleration, dt, G, other);
        self.kick_and_drift(&mut position, &mut velocity, &mut acceleration, dt);

    }

    pub fn calculating_force(&mut self, position: &mut [f64; 3], velocity: &mut [f64; 3], acceleration: &mut [f64; 3], dt: f64, G: f64, other) {
        let distance: [f64; 3];
        let direction: [f64; 3];

        for i in 0..3 {
            distance[i] = (other.position[i]).powf(2.0) - (self.position[i]).powf(2.0);
            direction[i] = - self.position[i]/distance[i];
            acceleration[i] = (G * other.mass) / distance[i];
            velocity[i] = acceleration[i] * dt
        }

        *acceleration;
        *velocity;
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
        *position = next_position
    }
}
