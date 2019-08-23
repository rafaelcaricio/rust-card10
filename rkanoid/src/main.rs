#![no_std]
#![no_main]

use core::fmt::Write;
use card10_l0dable::*;

pub const BALL_RADIUS: u16 = 4;
pub const PADDLE_SPACE: u16 = 32;
pub const BLOCK_W: u16 = 16;
pub const BLOCK_H: u16 = 10;
pub const BLOCKS_X: u16 = Display::W / BLOCK_W;
pub const BLOCKS_Y: u16 = (Display::H - PADDLE_SPACE) / BLOCK_H;

pub struct Blocks {
    blocks: [[Option<Color>; BLOCKS_X as usize]; BLOCKS_Y as usize],
}

impl Blocks {
    pub fn new() -> Self {
        Blocks { blocks: [[None; BLOCKS_X as usize]; BLOCKS_Y as usize], }
    }

    pub fn generate(rand_max: u8) -> Self {
        let mut result = Self::new();
        let mut count = 0;
        for line in result.blocks.iter_mut() {
            for block in line.iter_mut() {
                let mut buf = [0, 0, 0, 0];
                trng::read(&mut buf);
                if buf[0] <= rand_max {
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

const PADDLE_HEIGHT: u16 = 6;
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

    fn is_up(&self) -> bool {
        use Direction::*;
        match self {
            UR => true,
            UL => true,
            _ => false,
        }
    }

    fn is_down(&self) -> bool {
        use Direction::*;
        match self {
            DR => true,
            DL => true,
            _ => false,
        }
    }

    fn is_left(&self) -> bool {
        use Direction::*;
        match self {
            UL => true,
            DL => true,
            _ => false,
        }
    }

    fn is_right(&self) -> bool {
        use Direction::*;
        match self {
            UR => true,
            DR => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Bounce {
    Vertical,
    Horizontal,
}

enum GameResult {
    Over(u32),
    LevelFinish(u32),
}

fn game(level: u16, mut score: u32) -> GameResult {
    let start_time = Seconds::time();
    let display = Display::open();

    let mut paddle = Display::W / 2;
    let paddle_size = 18 - (2 * level).min(14);
    let mut ball_x = paddle;
    let mut ball_y = Display::H - PADDLE_HEIGHT - BALL_RADIUS;
    let mut ball_direction = Direction::UR;
    let mut blocks = Blocks::generate((0x3F + 0x10 * level).min(0xff) as u8);
    for tick in 0.. {
        let input = Buttons::read();
        let old_paddle = paddle;
        if input.left_bottom() {
            paddle -= PADDLE_SPEED;
            paddle = paddle.max(paddle_size);
        }
        if input.right_bottom() {
            paddle += PADDLE_SPEED;
            paddle = paddle.min(Display::W - paddle_size)
        }
        let paddle_moving = paddle != old_paddle;
        if input.left_top() {
            exit(0);
        }
        let mut check_finish = false;
        let speed_steps = 2 + (Seconds::time() - start_time).0 / 15;
        let speed_steps = (speed_steps >> 1) +
            (speed_steps & tick & 1);
        for _ in 0..speed_steps {
            ball_direction.motion(&mut ball_x, &mut ball_y);

            if (ball_direction.is_left() && ball_x <= BALL_RADIUS) ||
                (ball_direction.is_right() && ball_x >= Display::W - BALL_RADIUS) {
                    // Bounce on left/right border
                    ball_direction.bounce(Bounce::Vertical);
                    vibra::vibrate(10);
            }
            if ball_direction.is_up() && ball_y <= BALL_RADIUS {
                // Bounce on top border
                ball_direction.bounce(Bounce::Horizontal);
                vibra::vibrate(10);
            }
            if ball_direction.is_down() &&
                ball_y >= Display::H - PADDLE_HEIGHT - BALL_RADIUS &&
                ball_x >= paddle - paddle_size &&
                ball_x <= paddle + paddle_size {
                    // Bounce on paddle
                    if paddle_moving && input.left_bottom() {
                        ball_direction = Direction::UL;
                        vibra::vibrate(50);
                    } else if paddle_moving && input.right_bottom() {
                        ball_direction = Direction::UR;
                        vibra::vibrate(50);
                    } else {
                        ball_direction.bounce(Bounce::Horizontal);
                        vibra::vibrate(20);
                    }
                }
            if ball_y >= Display::H - BALL_RADIUS {
                return GameResult::Over(score);
            }
            if blocks.collides(ball_x - BALL_RADIUS, ball_y) ||
                blocks.collides(ball_x + BALL_RADIUS, ball_y) {
                    ball_direction.bounce(Bounce::Vertical);
                    score += 3;
                    // paddle_size += 2;
                    check_finish = true;
                    vibra::vibrate(60);
                }
            if blocks.collides(ball_x, ball_y - BALL_RADIUS) ||
                blocks.collides(ball_x, ball_y + BALL_RADIUS) {
                    ball_direction.bounce(Bounce::Horizontal);
                    score += 2;
                    // paddle_size += 1;
                    check_finish = true;
                    vibra::vibrate(40);
                }
        }
        if check_finish && blocks.is_finished() {
            return GameResult::LevelFinish(score + 100);
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
    unreachable!()
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
    write!(FmtBuffer::new(&mut score_str), "Score {}\0", score).unwrap();
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

fn level_finish(level: u16, score: u32) -> bool {
    let display = Display::open();
    display.clear(Color::red());
    display.print(30, 0, b"Rkanoid\0", Color::white(), Color::red());
    let mut level_str = [0u8; 32];
    write!(FmtBuffer::new(&mut level_str), "Lvl {} done!\0", level + 1).unwrap();
    display.print(0, 25, &level_str, Color::white(), Color::red());
    let mut score_str = [0u8; 32];
    write!(FmtBuffer::new(&mut score_str), "Score {}\0", score).unwrap();
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
    let mut level = 0;
    let mut score = 0;
    while !quit {
        match game(level, score) {
            GameResult::LevelFinish(new_score) => {
                let again = level_finish(level, new_score);
                quit = !again;
                score = new_score;
                level += 1;
            }
            GameResult::Over(new_score) => {
                let again = game_over(new_score);
                quit = !again;
                score = 0;
                level = 0;
            }
        }
    }
}
