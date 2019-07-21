#[link(name = "card10")]
extern {
    fn leds_init();
    fn leds_update();
    fn leds_set_dim(led: u8,dim: u8);
    fn leds_set(led: u8,r: u8,g: u8,b: u8);
    fn leds_set_hsv(led: u8, h: f32, s: f32, v: f32);
}

pub const LEDS: u8 = 15;

pub fn init() {
    unsafe { leds_init(); }
}

pub fn update() {
    unsafe { leds_update(); }
}

pub fn set_dim(led: u8,dim: u8) {
    unsafe { leds_set_dim(led, dim); }
}

pub fn set(led: u8,r: u8,g: u8,b: u8) {
    unsafe { leds_set(led, r, g, b); }
}

pub fn set_hsv(led: u8, h: f32, s: f32, v: f32) {
    unsafe { leds_set_hsv(led, h, s, v); }
}
