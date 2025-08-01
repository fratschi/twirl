use anyhow::Result;
pub mod tw;


fn main()-> Result<()> {
    tw::init::check()?;
    Ok(())
}
