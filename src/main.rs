pub mod utils;

mod week_01;
mod week_02;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    week_01::run()?;
    week_02::run()?;
    Ok(())
}
