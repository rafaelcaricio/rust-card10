#![no_std]
#![no_main]

use core::fmt::Write;
use l0dable::*;

pub const BALL_RADIUS: u16 = 3;
pub const PADDLE_SPACE: u16 = 24;
pub const BLOCK_W: u16 = 8;
pub const BLOCK_H: u16 = 4;
pub const BLOCKS_X: u16 = Display::W / BLOCK_W;
pub const BLOCKS_Y: u16 = (Display::H - PADDLE_SPACE) / BLOCK_H;

pub struct Blocks {
    blocks: [[Option<Color>; BLOCKS_X as usize]; BLOCKS_Y as usize],
}

impl Blocks {
    pub fn new() -> Self {
        Blocks { blocks: [[None; BLOCKS_X as usize]; BLOCKS_Y as usize], }
    }

    pub fn generate() -> Self {
        let mut result = Self::new();
        let mut count = 0;
        for line in result.blocks.iter_mut() {
            for block in line.iter_mut() {
                let mut buf = [0, 0, 0, 0];
                trng::read(&mut buf);
                if buf[0] > 0xBF {
                    *block = Some(Color::rgb8(
                       0x80 | buf[1],
                       0x80 | buf[2],
                       0x80 | buf[3],
                    ));
                    count += 1;
                }
            }
        }
        writeln!(UART, "Generated {} blocks\r", count).unwrap();
        result
    }

    pub fn collides(&mut self, x: u16, y: u16) -> bool {
        let col = (x / BLOCK_W) as usize;
        let line = (y / BLOCK_H) as usize;
        if line >= BLOCKS_Y.into() || col >= BLOCKS_X.into() {
            return false;
        }
        match self.blocks[line][col] {
            Some(_) => {
                self.blocks[line][col] = None;
                true
            }
            None => false,
        }
    }

    fn is_finished(&self) -> bool {
        self.blocks.iter().all(|line| line.iter().all(|block| block.is_none()))
    }
}

const PADDLE_HEIGHT: u16 = 4;
const PADDLE_SPEED: u16 = 4;

#[derive(Clone, Copy, Debug)]
enum Direction {
    /// Up Right
    UR,
    /// Up Left
    DR,
    /// Down Left
    DL,
    /// Down Right
    UL,
}

#[derive(Clone, Copy, Debug)]
enum Bounce {
    Vertical,
    Horizontal,
}

impl Direction {
    fn motion(&self, x: &mut u16, y: &mut u16) {
        use Direction::*;
        match self {
            UR => {
                *x += 1;
                *y -= 1;
            }
            DR => {
                *x += 1;
                *y += 1;
            }
            DL => {
                *x -= 1;
                *y += 1;
            }
            UL => {
                *x -= 1;
                *y -= 1;
            }
        }
    }

    fn bounce(&mut self, bounce: Bounce) {
        use Direction::*;
        use Bounce::*;
        match (*self, bounce) {
            (UR, Vertical) => *self = UL,
            (UL, Vertical) => *self = UR,
            (DR, Vertical) => *self = DL,
            (DL, Vertical) => *self = DR,
            (UR, Horizontal) => *self = DR,
            (UL, Horizontal) => *self = DL,
            (DR, Horizontal) => *self = UR,
            (DL, Horizontal) => *self = UL,
        }
    }
}


fn game() -> u32 {
    let start_time = time();
    let display = Display::open();

    let mut running = true;
    let mut score = 0;
    let mut paddle = Display::W / 2;
    let paddle_size = 10;
    let mut ball_x = paddle;
    let mut ball_y = Display::H - PADDLE_HEIGHT - BALL_RADIUS;
    let mut ball_direction = Direction::UR;
    let mut blocks = Blocks::generate();
    while running {
        let input = Buttons::read();
        if input.left_bottom() {
            paddle -= PADDLE_SPEED;
            paddle = paddle.max(paddle_size);
        }
        if input.right_bottom() {
            paddle += PADDLE_SPEED;
            paddle = paddle.min(Display::W - paddle_size)
        }
        if input.left_top() {
            exit(0);
        }
        let mut check_finish = false;
        let speed_steps = 1 + (time() - start_time) / 10;
        for _ in 0..speed_steps {
            ball_direction.motion(&mut ball_x, &mut ball_y);

            if ball_x <= BALL_RADIUS || ball_x >= Display::W - BALL_RADIUS {
                // Bounce on left/right border
                ball_direction.bounce(Bounce::Vertical);
            }
            if ball_y <= BALL_RADIUS {
                // Bounce on top border
                ball_direction.bounce(Bounce::Horizontal);
            }
            if ball_y == Display::H - PADDLE_HEIGHT - BALL_RADIUS &&
                ball_x >= paddle - paddle_size &&
                ball_x <= paddle + paddle_size {
                    // Bounce on paddle
                    ball_direction.bounce(Bounce::Horizontal);
                }
            if ball_y >= Display::H - BALL_RADIUS {
                // Game over
                running = false;
            }
            if blocks.collides(ball_x - BALL_RADIUS, ball_y) ||
                blocks.collides(ball_x + BALL_RADIUS, ball_y) {
                    ball_direction.bounce(Bounce::Vertical);
                    score += 100;
                    // paddle_size += 2;
                    check_finish = true;
                    vibra::vibrate(60);
                }
            if blocks.collides(ball_x, ball_y - BALL_RADIUS) ||
                blocks.collides(ball_x, ball_y + BALL_RADIUS) {
                    ball_direction.bounce(Bounce::Horizontal);
                    score += 60;
                    // paddle_size += 1;
                    check_finish = true;
                    vibra::vibrate(40);
                }
        }
        if check_finish && blocks.is_finished() {
            return score + 1000;
        }
        
        display.clear(Color::black());
        // Paddle
        display.rect(
            paddle - paddle_size, Display::H - PADDLE_HEIGHT,
            paddle + paddle_size, Display::H - 1,
            Color::rgb8(0x7f, 0xff, 0x7f), FillStyle::Filled, 1
        );
        // Ball
        display.circ(
            ball_x, ball_y, BALL_RADIUS,
            Color::rgb8(0x7f, 0x7f, 0xff), FillStyle::Filled, 1
        );
        // Blocks
        for (lineno, line) in blocks.blocks.iter_mut().enumerate() {
            let lineno = lineno as u16;
            for (colno, block) in line.iter_mut().enumerate() {
                block.map(|color| {
                    let colno = colno as u16;
                    display.rect(
                        colno * BLOCK_W,
                        lineno * BLOCK_H,
                        (colno + 1) * BLOCK_W - 1,
                        (lineno + 1) * BLOCK_H - 1,
                        color, FillStyle::Filled, 1
                    );
                });
            }
        }
        display.update();
    }

    score
}

fn title_screen() {
    let display = Display::open();
    display.clear(Color::red());
    display.print(30, 15, b"Rkanoid\0", Color::white(), Color::red());
    display.print(30, 30, b"in Rust\0", Color::white(), Color::red());
    display.print(20, 45, b"by Astro\0", Color::white(), Color::red());
    display.print(Display::W - 2 * Display::FONT_W, Display::H - Display::FONT_H, b"Go\0", Color::yellow(), Color::blue());
    display.update();
    while !Buttons::read().right_bottom() {}
}

fn game_over(score: u32) -> bool {
    let display = Display::open();
    display.clear(Color::red());
    display.print(30, 0, b"Rkanoid\0", Color::white(), Color::red());
    display.print(0, 25, b"Game over!\0", Color::white(), Color::red());
    let mut score_str = [0u8; 32];
    write!(FmtBuffer::new(&mut score_str), "Score: {}\0", score).unwrap();
    display.print(0, 60, &score_str, Color::white(), Color::red());
    display.print(0, 0, b"Q\0", Color::yellow(), Color::blue());
    display.print(Display::W - Display::FONT_W, Display::H - 2 * Display::FONT_H, b"S\0", Color::yellow(), Color::blue());
    display.update();
    loop {
        let buttons = Buttons::read();
        if buttons.left_top() {
            return false
        }
        if buttons.right_top() {
            return true
        }
    }
}

main!(main);
fn main() {
    title_screen();
    
    let mut quit = false;
    while !quit {
        let score = game();
        let again = game_over(score);
        quit = !again;
    }
}
