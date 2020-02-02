mod experiments;

fn main() -> Result<(), String> {

    experiments::run()?;

    Ok(())
}