pub mod utils;
mod week_01;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    week_01::run()?;
    Ok(())
}
