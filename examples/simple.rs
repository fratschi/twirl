use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use twirl::tw::render::{OpCode::*, *};

fn main() {
    let (mut rl, thread) = raylib::init().size(0, 0).fullscreen().undecorated().build();
    let mut renderer = Renderer::new();
    rl.set_target_fps(60);

    while rl.window_should_close() == false {
        let mut rdh: RaylibDrawHandle = rl.begin_drawing(&thread);
        rdh.clear_background(Color::BLACK);
        let time = rdh.get_frame_time();
        renderer.render(
            &mut rdh,
            time, 
            vec![
                LABEL(START),                 // Label for the start of the program
                
                LV4(VR0, 100, 100, 200, 255), // Load Color LightBlue into vector register 0
                LV4(VR1, 100, 125, 280, 125), // Load Positions into vector register 1
                LV1(VR2, 5),                  // Load Thickness 5 into vector register 2
                DL(VR0, VR1, VR2),            // Draw Line with color from VR0, position from VR1, thickness from VR2.1

                LV4(VR1, 200, 200, 200, 255), // Load Color LightGray into vector register 0
                AB(VR1, 0.2),                 // Fade in of Alpha Part of Color in VR1 by factor of screen time
                LV2(VR2, 100, 100),           // Load Position (100, 100) into vector register 2
                LV1(VR3, 25),                 // Load Font Scale 25 into vector register 3
                DT(FR0, VR1, VR2, VR3, "Hello from Twirl".into()), // Display Text, TODO Font Register 0 is raylib defaut font for now
            ],
        );
    }
}
