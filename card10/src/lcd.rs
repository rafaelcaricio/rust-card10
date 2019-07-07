#[link(name = "card10")]
extern {
    fn LCD_SetBacklight(brightness: usize);
    fn LCD_Clear(color: usize);
    fn LCD_ClearWindow(xstart: usize, ystart: usize, xend: usize, yend: usize, color: usize);
    fn LCD_SetWindowColor(xstart: usize, ystart: usize, xend: usize, yend: usize, color: usize);
    fn LCD_SetUWORD(x: usize, y: usize, color: usize);
    fn LCD_Update();
}

pub const W: usize = 160;
pub const H: usize = 80;

pub fn set_backlight(brightness: usize) {
    unsafe {
        LCD_SetBacklight(brightness);
    }
}

pub fn put_pixel(x: usize, y: usize, color: usize) {
    unsafe {
        LCD_SetUWORD(x, y, color);
    }
}

pub fn update() {
    unsafe {
        LCD_Update();
    }
}
