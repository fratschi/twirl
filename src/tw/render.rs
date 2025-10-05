use raylib::{
    RaylibThread,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub const START: u8 = 0;
pub const END: u8 = 255;

pub const VR0: u8 = 0;
pub const VR1: u8 = 1;
pub const VR2: u8 = 2;
pub const VR3: u8 = 3;
pub const VR4: u8 = 4;
pub const VR5: u8 = 5;
pub const VR6: u8 = 6;
pub const VR7: u8 = 7;
pub const VR8: u8 = 8;
pub const VR9: u8 = 9;
pub const VR10: u8 = 10;
pub const VR11: u8 = 11;
pub const VR12: u8 = 12;
pub const VR13: u8 = 13;
pub const VR14: u8 = 14;
pub const VR15: u8 = 15;

pub const FR0: u8 = 0;
pub const FR1: u8 = 1;
pub const FR2: u8 = 2;
pub const FR3: u8 = 3;
pub enum OpCode {
    // Label
    LABEL(u8),

    // Clear Vector Register
    CV1(u8),
    CV2(u8),
    CV4(u8),

    // Move Vector Register to Vector Register
    MV1(u8, u8),
    MV2(u8, u8),
    MV4(u8, u8),

    // Load Value to Vector Register
    LV1(u8, u16),
    LV2(u8, u16, u16),
    LV4(u8, u16, u16, u16, u16),

    // Load Font
    LF(u8, u8),

    // Draw Text font:u8, color:u8, xy:u8, scale:u8, text:String
    DT(u8, u8, u8, u8, String),

    // Draw Line color:u8, xy:u8, thick:u8
    DL(u8, u8, u8),

    // Alpha Blend of alpha part of vector register. Positive alpha fades in, negative alpha fades out.
    AB(u8, f32), 
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    x: u16,
    y: u16,
    z: u16,
    w: u16,
}

struct Reg {
    // Program Counter
    pc: u64,

    // Font Registers
    font: [u8; 4],

    // General Purpose Registers
    vec: [Vec4; 16],

    // Projection Registers
    px: u16,
    py: u16,

    // Time Register
    time: f32,
}

pub struct Renderer {
    reg: Reg,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            reg: Reg {
                pc: 0,
                font: [0; 4],
                vec: [Vec4 {
                    x: 0,
                    y: 0,
                    z: 0,
                    w: 0,
                }; 16],
                px: 0,
                py: 0,
                time: 0.0,
            },
        }
    }

    pub fn render(&mut self, dh: &mut RaylibDrawHandle, ft: f32, ops: Vec<OpCode>) {
        self.reg.time += ft;
        for op in ops {
            match op {
                OpCode::LABEL(n) => {
                    // Handle label
                }
                OpCode::CV1(r) => {
                    self.reg.vec[r as usize].x = 0;
                }
                OpCode::CV2(r) => {
                    let vec = &mut self.reg.vec[r as usize];
                    vec.x = 0;
                    vec.y = 0;
                }
                OpCode::CV4(r) => {
                    let vec = &mut self.reg.vec[r as usize];
                    vec.x = 0;
                    vec.y = 0;
                    vec.z = 0;
                    vec.w = 0;
                }
                OpCode::MV1(dst, src) => {
                    self.reg.vec[dst as usize].x = self.reg.vec[src as usize].x;
                }
                OpCode::MV2(dst, src) => {
                    self.reg.vec[dst as usize].x = self.reg.vec[src as usize].x;
                    self.reg.vec[dst as usize].y = self.reg.vec[src as usize].y;
                }
                OpCode::MV4(dst, src) => {
                    self.reg.vec[dst as usize] = self.reg.vec[src as usize];
                }
                OpCode::LV1(r, v) => {
                    self.reg.vec[r as usize].x = v;
                }
                OpCode::LV2(r, v1, v2) => {
                    let vec = &mut self.reg.vec[r as usize];
                    vec.x = v1;
                    vec.y = v2;
                }
                OpCode::LV4(r, v1, v2, v3, v4) => {
                    let vec = &mut self.reg.vec[r as usize];
                    vec.x = v1;
                    vec.y = v2;
                    vec.z = v3;
                    vec.w = v4;
                }
                OpCode::LF(font_reg, font_id) => {
                    self.reg.font[font_reg as usize] = font_id;
                }
                OpCode::DT(font_reg, color_reg, xy_reg, scale_reg, text) => {
                    let color = self.reg.vec[color_reg as usize];
                    let pos = self.reg.vec[xy_reg as usize];
                    dh.draw_text_ex(
                        dh.get_font_default(),
                        &text,
                        raylib::math::Vector2 {
                            x: pos.x as f32,
                            y: pos.y as f32,
                        },
                        self.reg.vec[scale_reg as usize].x as f32,
                        1.0,
                        raylib::color::Color {
                            r: color.x as u8,
                            g: color.y as u8,
                            b: color.z as u8,
                            a: color.w as u8,
                        },
                    );
                }
                OpCode::DL(color_reg, xy_reg, thick_reg) => {
                    let color = self.reg.vec[color_reg as usize];
                    let pos = self.reg.vec[xy_reg as usize];
                    let thick = self.reg.vec[thick_reg as usize].x;
                    dh.draw_line_ex(
                        raylib::math::Vector2 {
                            x: pos.x as f32,
                            y: pos.y as f32,
                        },
                        raylib::math::Vector2 {
                            x: pos.z as f32,
                            y: pos.w as f32,
                        },
                        thick as f32,
                        raylib::color::Color {
                            r: color.x as u8,
                            g: color.y as u8,
                            b: color.z as u8,
                            a: color.w as u8,
                        },
                    );
                }
                OpCode::AB(r, alpha) => {
                    if alpha > 0.0 {
                        let vec = &mut self.reg.vec[r as usize];
                        let a = self.reg.time * alpha;
                        vec.w = (a.clamp(0.0, 1.0) * 255.0) as u16;
                    } else {
                        let vec = &mut self.reg.vec[r as usize];
                        let a = (self.reg.time * -alpha * 255.0).clamp(0.0, 255.0);
                        vec.w = 255 - (a as u16);
                    }
                }
            }
        }
    }
}
