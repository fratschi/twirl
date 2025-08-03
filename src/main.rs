use anyhow::Result;
pub mod tw;


fn main()-> Result<()> {
    tw::init::check()?;
    let cfg = tw::conf::load_config()?;

    println!("Configuration loaded successfully:");
    println!("Config: {:?}", cfg);
    
    Ok(())
}
