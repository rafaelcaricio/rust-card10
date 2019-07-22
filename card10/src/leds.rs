#[link(name = "card10")]
extern {
    fn leds_init();
    fn leds_update();
    fn leds_set_dim(led: u8,dim: u8);
    fn leds_set(led: u8,r: u8,g: u8,b: u8);
    fn leds_set_hsv(led: u8, h: f32, s: f32, v: f32);
}

pub struct Leds;

impl Leds {
    pub fn new() -> Self {
        unsafe { leds_init(); }
        Leds
    }

    pub fn len(&self) -> usize {
        15
    }

    pub fn led(&self, idx: usize) -> Led {
        Led { idx }
    }

    pub fn iter(&self) -> impl Iterator<Item=Led> {
        (0..self.len()).map(|idx| Led { idx })
    }

    pub fn update(&self) {
        unsafe { leds_update(); }
    }
}

pub struct Led {
    idx: usize,
}

impl Led {
    pub fn set_dim(&self, dim: u8) {
        unsafe { leds_set_dim(self.idx as u8, dim); }
    }

    pub fn set(&self, r: u8, g: u8, b: u8) {
        unsafe { leds_set(self.idx as u8, r, g, b); }
    }

    pub fn set_hsv(&self, h: f32, s: f32, v: f32) {
        unsafe { leds_set_hsv(self.idx as u8, h, s, v); }
    }
}
