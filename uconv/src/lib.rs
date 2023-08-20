const G_DENSITY_AIR: f64 = 1.2;
const G_VEL_OF_SOUND: f64 = 343.0;

enum Commands {
    MACH,     // Mach number
    VEL,      // Velocity in [m/s]
    DYNPRESS, // Dynamic pressure in [Pa]
}

pub struct Config {
    query: Commands,
    argument: f64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let input = args[1].clone();
        let query: Commands;

        if input.eq("MACH") {
            query = Commands::MACH;
        } else if input.eq("VEL") {
            query = Commands::VEL;
        } else if input.eq("DYNPRESS") {
            query = Commands::DYNPRESS;
        } else {
            return Err("Invalid input strings");
        }

        let argument: f64 = args[2].clone().parse().unwrap_or(0.0);

        Ok(Config { query, argument })
    }

    pub fn run(self) {
        match self.query {
            Commands::MACH => {
                from_mach(self.argument);
            }
            Commands::VEL => {
                from_vel(self.argument);
            }
            Commands::DYNPRESS => {
                from_dynpress(self.argument);
            }
        }
    }

    pub fn help() {
        println!(
            "The following commands with floating value arguments XX are supported are supported"
        );
        println!("uconv  VEL XX");
        println!("uconv  MACH XX");
        println!("uconv  DYNPRESS XX");

        println!("Thanks for taking help! All the best");
    }
}

fn from_mach(val: f64) {
    let vel = val * G_VEL_OF_SOUND;
    let dyn_press = 0.5 * G_DENSITY_AIR * vel * vel;
    println!("Velocity is {vel} [m/s] and Dynamic Pressure is {dyn_press} [pa]");
}

fn from_vel(val: f64) {
    let mach = val / G_VEL_OF_SOUND;
    let dyn_press = 0.5 * G_DENSITY_AIR * val * val;
    println!("Mach is {mach} [-] and Dynamic Pressure is {dyn_press} [pa]");
}

fn from_dynpress(val: f64) {
    let vel = (2.0_f64 * val / G_DENSITY_AIR).sqrt();
    let mach = vel / G_VEL_OF_SOUND;
    println!("Mach is {mach} [-] and Velocity is {vel} [m/s]");
}
