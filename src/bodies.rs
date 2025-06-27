extern crate vsop87;
use crate::integration::Parameters;

static DATE_1: f64 = 2159345.0;
static DATE_2: f64 = DATE_1 + 1.0;

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
    
    let mercury_velocity: [f64; 3] = [mercury_data_2.x - mercury_data_1.x,  mercury_data_2.y - mercury_data_1.y, mercury_data_2.z - mercury_data_1.z];
    let mercury_position: [f64; 3] = [mercury_data_2.x, mercury_data_2.y, mercury_data_2.z];

    let venus_velocity: [f64; 3] = [venus_data_2.x - venus_data_1.x,  venus_data_2.y - venus_data_1.y, venus_data_2.z - venus_data_1.z];     
    let venus_position: [f64; 3] = [venus_data_2.x, venus_data_2.y, venus_data_2.z];

    let earth_velocity: [f64; 3] = [earth_data_2.x - earth_data_1.x,  earth_data_2.y - earth_data_1.y, earth_data_2.z - earth_data_1.z];
    let earth_position: [f64; 3] = [earth_data_2.x, earth_data_2.y, earth_data_2.z];

    let mars_velocity: [f64; 3] = [mars_data_2.x - mars_data_1.x,  mars_data_2.y - mars_data_1.y, mars_data_2.z - mars_data_1.z];
    let mars_position: [f64; 3] = [mars_data_2.x, mars_data_2.y, mars_data_2.z];

    let jupiter_velocity: [f64; 3] = [jupiter_data_2.x - jupiter_data_1.x,  jupiter_data_2.y - jupiter_data_1.y, jupiter_data_2.z - jupiter_data_1.z];
    let jupiter_position: [f64; 3] = [jupiter_data_2.x, jupiter_data_2.y, jupiter_data_2.z];

    let saturn_velocity: [f64; 3] = [saturn_data_2.x - saturn_data_1.x,  saturn_data_2.y - saturn_data_1.y, saturn_data_2.z - saturn_data_1.z];
    let saturn_position: [f64; 3] = [saturn_data_2.x, saturn_data_2.y, saturn_data_2.z];

    let uranus_velocity: [f64; 3] = [uranus_data_2.x - uranus_data_1.x,  uranus_data_2.y - uranus_data_1.y, uranus_data_2.z - uranus_data_1.z];
    let uranus_position: [f64; 3] = [uranus_data_2.x, uranus_data_2.y, uranus_data_2.z];

    let neptune_velocity: [f64; 3] = [neptune_data_2.x - neptune_data_1.x,  neptune_data_2.y - neptune_data_1.y, neptune_data_2.z - neptune_data_1.z];
    let neptune_position: [f64; 3] = [neptune_data_2.x, neptune_data_2.y, neptune_data_2.z];


    println!("mercury position: {:?}", mercury_position);
    println!("mercury velocity: {:?}", mercury_velocity);

    println!("venus position: {:?}", venus_position);
    println!("venus velocity: {:?}", venus_velocity);

    println!("earth position: {:?}", earth_position);
    println!("earth velocity: {:?}", earth_velocity);

    println!("mars position: {:?}", mars_position);
    println!("mars velocity: {:?}", mars_velocity);

    println!("jupiter position: {:?}", jupiter_position);
    println!("jupiter velocity: {:?}", jupiter_velocity);

    println!("saturn position: {:?}", saturn_position);
    println!("saturn velocity: {:?}", saturn_velocity);

    println!("uranus position: {:?}", uranus_position);
    println!("uranus velocity: {:?}", uranus_velocity);

    println!("neptune position: {:?}", neptune_position);
    println!("neptune velocity: {:?}", neptune_velocity);

    let planet_position: [[f64; 3]; 8] = [mercury_position, venus_position, earth_position, mars_position, jupiter_position, saturn_position, neptune_position, uranus_position];

    let planet_velocity: [[f64; 3]; 8] = [mercury_velocity, venus_velocity, earth_velocity, mars_velocity, jupiter_velocity, saturn_velocity, neptune_velocity, uranus_velocity];

    (planet_position, planet_velocity)
}

pub fn planets() {

    let (positions, velocities) = data_extraction();

    println!("positions: {:?}", positions);
    println!("velocities: {:?}", velocities);
    
    let sun = Parameters {
        position: [0.0, 0.0, 0.0],
        velocity: [0.0, 0.0, 0.0],
        //acceleration: [0.0, 0.0, 0.0], 
        mass: 1_988_400e24,
    };
    let mercury = Parameters {
        position: positions[0],
        velocity: velocities[0],
        //acceleration: , 
        mass: 0.33e24,
    };
    let venus = Parameters {
        position: positions[1],
        velocity: velocities[1],
        //acceleration: , 
        mass: 4.87e24,
    };
    let earth = Parameters {
        position: positions[2],
        velocity: velocities[2],
        //acceleration: , 
        mass: 5.97e24,
    };
    let mars = Parameters {
        position: positions[3],
        velocity: velocities[3],
        //acceleration: , 
        mass: 0.642,
    };
    let jupiter = Parameters {
        position: positions[4],
        velocity: velocities[4],
        //acceleration: , 
        mass: 1898e24,
    };
    let saturn = Parameters {
        position: positions[5],
        velocity: velocities[5],
        //acceleration: , 
        mass: 568e24,
    };
    let uranus = Parameters {
        position: positions[6],
        velocity: velocities[6],
        //acceleration: , 
        mass: 86.8e24,
    };
    let neptune = Parameters {
        position: positions[7],
        velocity: velocities[7],
        //acceleration: , 
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
}