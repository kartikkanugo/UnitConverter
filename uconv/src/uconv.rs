use clap::{Parser, Subcommand};

const G_DENSITY_AIR: f64 = 1.2;
const G_VEL_OF_SOUND: f64 = 343.0;
const G_PSI_PASCAL: f64 = 6894.7572932;

#[derive(Parser, Debug)]
#[command(name = "uconv", about = "Unit conversion tool", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Calculate velocity and dynamic pressure from MACH
    Mach {
        /// MACH value
        #[arg(short, long)]
        value: f64,
    },
    /// Calculate MACH and dynamic pressure from velocity
    Vel {
        /// Velocity value in m/s
        #[arg(short, long)]
        value: f64,
    },
    /// Calculate MACH and velocity from dynamic pressure
    DynPress {
        /// Dynamic pressure value
        #[arg(short, long)]
        value: f64,
        /// Units of dynamic pressure (Pa or PSI)
        #[arg(short, long)]
        units: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
enum Units {
    Pa,
    Psi,
    NoInp,
}

impl From<Option<String>> for Units {
    fn from(opt: Option<String>) -> Self {
        match opt.as_deref() {
            Some("PSI") => Units::Psi,
            Some("Pa") => Units::Pa,
            _ => Units::NoInp,
        }
    }
}

pub fn run(args: Args) {
    match args.command {
        Commands::Mach { value } => from_mach(value),
        Commands::Vel { value } => from_vel(value),
        Commands::DynPress { value, units } => from_dynpress(value, units.into()),
    }
}

fn from_mach(mach: f64) {
    let velocity = mach * G_VEL_OF_SOUND;
    let dynamic_pressure = 0.5 * G_DENSITY_AIR * velocity * velocity;
    println!(
        "Velocity is {:.2} m/s and Dynamic Pressure is {:.2} Pa",
        velocity, dynamic_pressure
    );
}

fn from_vel(velocity: f64) {
    let mach = velocity / G_VEL_OF_SOUND;
    let dynamic_pressure = 0.5 * G_DENSITY_AIR * velocity * velocity;
    println!(
        "Mach is {:.2} and Dynamic Pressure is {:.2} Pa",
        mach, dynamic_pressure
    );
}

fn from_dynpress(mut pressure: f64, units: Units) {
    if units == Units::Psi {
        pressure *= G_PSI_PASCAL;
    }
    let velocity = (2.0 * pressure / G_DENSITY_AIR).sqrt();
    let mach = velocity / G_VEL_OF_SOUND;
    println!(
        "Mach is {:.2}, Velocity is {:.2} m/s, and Dynamic Pressure is {:.2} Pa",
        mach, velocity, pressure
    );
}
