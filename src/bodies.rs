extern crate vsop87;
use crate::integration::Parameters;

static DATE_1: f64 = 2159345.0;
static DATE_2: f64 = DATE_1 + DT;
static DT: f64 = 1.0;

// imports the planetary positions at a given julian date and 1 day after
// calculates the velocity of the planet, returning an array of the planets positions and velocities
// position has units of AU from the sun and 
fn data_extraction() -> ([[f64; 3]; 8], [[f64; 3]; 8]) {
    let mercury_data_1 = vsop87::vsop87c::mercury(DATE_1);
    let venus_data_1 = vsop87::vsop87c::venus(DATE_1);
    let earth_data_1 = vsop87::vsop87c::earth(DATE_1);
    let mars_data_1 = vsop87::vsop87c::mars(DATE_1);
    let jupiter_data_1 = vsop87::vsop87c::jupiter(DATE_1);
    let saturn_data_1 = vsop87::vsop87c::saturn(DATE_1);
    let uranus_data_1 = vsop87::vsop87c::uranus(DATE_1);
    let neptune_data_1 = vsop87::vsop87c::neptune(DATE_1);

    let mercury_data_2 = vsop87::vsop87c::mercury(DATE_2);
    let venus_data_2 = vsop87::vsop87c::venus(DATE_2);
    let earth_data_2 = vsop87::vsop87c::earth(DATE_2);
    let mars_data_2 = vsop87::vsop87c::mars(DATE_2);
    let jupiter_data_2 = vsop87::vsop87c::jupiter(DATE_2);
    let saturn_data_2 = vsop87::vsop87c::saturn(DATE_2);
    let uranus_data_2 = vsop87::vsop87c::uranus(DATE_2);
    let neptune_data_2 = vsop87::vsop87c::neptune(DATE_2);
    
    let mercury_velocity: [f64; 3] = [(mercury_data_2.x - mercury_data_1.x)/DT,  (mercury_data_2.y - mercury_data_1.y)/DT, (mercury_data_2.z - mercury_data_1.z)/DT];
    let mercury_position: [f64; 3] = [mercury_data_2.x, mercury_data_2.y, mercury_data_2.z];

    let venus_velocity: [f64; 3] = [(venus_data_2.x - venus_data_1.x)/DT, (venus_data_2.y - venus_data_1.y)/DT, (venus_data_2.z - venus_data_1.z)/DT];     
    let venus_position: [f64; 3] = [venus_data_2.x, venus_data_2.y, venus_data_2.z];

    let earth_velocity: [f64; 3] = [(earth_data_2.x - earth_data_1.x)/DT, (earth_data_2.y - earth_data_1.y)/DT, (earth_data_2.z - earth_data_1.z)/DT];
    let earth_position: [f64; 3] = [earth_data_2.x, earth_data_2.y, earth_data_2.z];

    let mars_velocity: [f64; 3] = [(mars_data_2.x - mars_data_1.x)/DT, (mars_data_2.y - mars_data_1.y)/DT, (mars_data_2.z - mars_data_1.z)/DT];
    let mars_position: [f64; 3] = [mars_data_2.x, mars_data_2.y, mars_data_2.z];

    let jupiter_velocity: [f64; 3] = [(jupiter_data_2.x - jupiter_data_1.x)/DT, (jupiter_data_2.y - jupiter_data_1.y)/DT, (jupiter_data_2.z - jupiter_data_1.z)/DT];
    let jupiter_position: [f64; 3] = [jupiter_data_2.x, jupiter_data_2.y, jupiter_data_2.z];

    let saturn_velocity: [f64; 3] = [(saturn_data_2.x - saturn_data_1.x)/DT, (saturn_data_2.y - saturn_data_1.y)/DT, (saturn_data_2.z - saturn_data_1.z)/DT];
    let saturn_position: [f64; 3] = [saturn_data_2.x, saturn_data_2.y, saturn_data_2.z];

    let uranus_velocity: [f64; 3] = [(uranus_data_2.x - uranus_data_1.x)/DT, (uranus_data_2.y - uranus_data_1.y)/DT, (uranus_data_2.z - uranus_data_1.z)/DT];
    let uranus_position: [f64; 3] = [uranus_data_2.x, uranus_data_2.y, uranus_data_2.z];

    let neptune_velocity: [f64; 3] = [(neptune_data_2.x - neptune_data_1.x)/DT, (neptune_data_2.y - neptune_data_1.y)/DT, (neptune_data_2.z - neptune_data_1.z)/DT];
    let neptune_position: [f64; 3] = [neptune_data_2.x, neptune_data_2.y, neptune_data_2.z];


    let mut planet_position: [[f64; 3]; 8] = [mercury_position, venus_position, earth_position, mars_position, jupiter_position, saturn_position, neptune_position, uranus_position];

    let mut planet_velocity: [[f64; 3]; 8] = [mercury_velocity, venus_velocity, earth_velocity, mars_velocity, jupiter_velocity, saturn_velocity, neptune_velocity, uranus_velocity];

    for i in 0..8 {
        for j in 0..3 {
            planet_position[i][j] = planet_position[i][j] * 1.5e11;
            planet_velocity[i][j] = planet_velocity[i][j] * 1.5e11 / 86400.0;
        }
    }

    (planet_position, planet_velocity)
}

pub fn planets() -> [Parameters; 8] {

    let (positions, velocities) = data_extraction();

    let sun = Parameters {
        name: "sun",
        position: [0.0, 0.0, 0.0],
        velocity: [0.0, 0.0, 0.0],
        acceleration: [0.0, 0.0, 0.0], 
        mass: 1_988_400e24,
    };
    let mercury = Parameters {
        name: "mercury",
        position: positions[0],
        velocity: velocities[0],
        acceleration: [0.0,0.0,0.0], 
        mass: 0.33e24,
    };
    let venus = Parameters {
        name: "venus", 
        position: positions[1],
        velocity: velocities[1],
        acceleration: [0.0,0.0,0.0], 
        mass: 4.87e24,
    };
    let earth = Parameters {
        name: "earth", 
        position: positions[2],
        velocity: velocities[2],
        acceleration: [0.0,0.0,0.0], 
        mass: 5.97e24,
    };
    let mars = Parameters {
        name: "mars", 
        position: positions[3],
        velocity: velocities[3],
        acceleration: [0.0,0.0,0.0], 
        mass: 0.642,
    };
    let jupiter = Parameters {
        name: "jupiter", 
        position: positions[4],
        velocity: velocities[4],
        acceleration: [0.0,0.0,0.0], 
        mass: 1898e24,
    };
    let saturn = Parameters {
        name: "saturn", 
        position: positions[5],
        velocity: velocities[5],
        acceleration: [0.0,0.0,0.0], 
        mass: 568e24,
    };
    let uranus = Parameters {
        name: "uranus", 
        position: positions[6],
        velocity: velocities[6],
        acceleration: [0.0,0.0,0.0], 
        mass: 86.8e24,
    };
    let neptune = Parameters {
        name: "neptune", 
        position: positions[7],
        velocity: velocities[7],
        acceleration: [0.0,0.0,0.0], 
        mass: 102e24,
    };

    println!{"Sun: {:?}", sun}
    println!{"Mercury: {:?}", mercury};
    println!{"Venus: {:?}", venus};
    println!{"Earth: {:?}", earth};
    println!{"Mars: {:?}", mars};
    println!{"Jupiter: {:?}", jupiter};
    println!{"Saturn: {:?}", saturn};
    println!{"Uranus: {:?}", uranus};
    println!{"Neptune: {:?}", neptune};

    let planets: [Parameters; 8] = [mercury, venus, earth, mars, jupiter, saturn, uranus, neptune];
    planets
}