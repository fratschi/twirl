pub mod tw;

use anyhow::Result;
use crate::tw::mainloop;


fn main() -> Result<()> {
    tw::init::check()?;
    let cfg = tw::conf::load_config()?;

    let tws = tw::md::load(std::path::Path::new("src/tw/sample.md"));

    let tw = tws.expect("Failed to load markdown");

    let (mut rl, thread) = raylib::init().size(0, 0).fullscreen().undecorated().build();
    rl.set_window_monitor(1);
    mainloop::render_loop(&mut rl, &thread, tw);

    Ok(())
}
