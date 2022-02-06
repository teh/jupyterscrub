use clap::{arg, App};

mod nbformat;
mod scrub;


fn scrub_one(path: &str) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;
    let result = scrub::scrub(&input)?;

    if result.modified {
        std::fs::write(path,result.json)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("jupyterscrub").args(&[
        arg!(<input> ... "input files"),
    ]).get_matches();

    matches.values_of("input").unwrap().try_for_each(scrub_one)?;

    Ok(())
}
