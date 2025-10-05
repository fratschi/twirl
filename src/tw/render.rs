

pub enum Cmd {
    MOV,
    Serve,
    Build,
}

struct Renderer {}


struct Vec4 {
    x: u16,
    y: u16,
    z: u16,
    w: u16,
}



struct Reg {

    // Program Counter
    pc :  u64,

    // Font Registers
    font: [u8;4],

    // General Purpose Registers
    vec:  [Vec4;16],
    
    // Projection Registers
    px:  u16,
    py:  u16,



}

impl Renderer {
    fn new() -> Self {
        Renderer {}
    }

   
}