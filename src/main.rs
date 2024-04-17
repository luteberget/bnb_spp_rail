mod problem;

fn main() -> anyhow::Result<()> {
    for i in glob::glob("instances/TCSIN*")? {
        let name = i?.to_string_lossy().to_string();
        println!("Reading {}", name);
        let p = problem::read(&name)?;
        println!("Read problem with {} trains", p.status.trains.len());
    }
    Ok(())
}
