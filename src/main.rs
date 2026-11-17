use macroquad::prelude::*;

// Paddles
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_SPEED: f32 = 300.0; // your paddle
const OPPONENT_SPEED: f32 = 240.0; // a bit slower, so late in a rally it can miss
const PADDLE_MARGIN: f32 = 20.0; // gap between a paddle and the screen edge
const AI_DEAD_ZONE: f32 = 5.0; // bigger = lazier opponent

// Ball
const BALL_RADIUS: f32 = 10.0;
const BALL_START_SPEED_X: f32 = 150.0;
const BALL_MIN_SPEED_Y: f32 = 60.0;
const BALL_MAX_START_SPEED_Y: f32 = 140.0;
const BALL_SPEEDUP: f32 = 1.05; // multiplier on every paddle hit
const BALL_MAX_SPEED: f32 = 350.0; // per axis; stays catchable and stops tunnelling

// Rules and text
const WIN_SCORE: u32 = 5;
const FONT_SIZE: f32 = 40.0;
const LINE_GAP: f32 = 50.0;
const SCORE_OFFSET_X: f32 = 200.0;
const SCORE_Y: f32 = 40.0;

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Paddle {
    fn new(x: f32, speed: f32) -> Self {
        Paddle {
            x,
            y: (screen_height() - PADDLE_HEIGHT) / 2.0,
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
            speed,
        }
    }

    fn reset(&mut self) {
        self.y = (screen_height() - self.h) / 2.0;
    }

    fn update(&mut self, up: KeyCode, down: KeyCode) {
        if is_key_down(up) {
            self.y -= self.speed * get_frame_time();
        }

        if is_key_down(down) {
            self.y += self.speed * get_frame_time();
        }

        self.y = self.y.clamp(0.0, screen_height() - self.h);
    }

    fn follow(&mut self, target_y: f32) {
        let center = self.y + self.h / 2.0;
        let distance = (target_y - center).abs();

        if distance > AI_DEAD_ZONE {
            if target_y < center {
                self.y -= self.speed * get_frame_time();
            } else {
                self.y += self.speed * get_frame_time();
            }
        }

        self.y = self.y.clamp(0.0, screen_height() - self.h);
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, RED);
    }
}

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

impl Ball {
    fn new() -> Self {
        let mut ball = Ball {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            radius: BALL_RADIUS,
        };
        ball.reset(1.0);
        ball
    }

    fn update(&mut self, player_score: &mut u32, opponent_score: &mut u32) {
        self.x += self.vx * get_frame_time();
        self.y += self.vy * get_frame_time();

        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = -self.vy;
        }

        if self.y + self.radius > screen_height() {
            self.y = screen_height() - self.radius;
            self.vy = -self.vy;
        }

        if self.x > screen_width() {
            *player_score += 1;
            self.reset(1.0);
        }

        if self.x < 0.0 {
            *opponent_score += 1;
            self.reset(-1.0);
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE)
    }

    fn bounce_off(&mut self, paddle: &Paddle) {
        if ball_hits_paddle(self, paddle) {
            if self.y < paddle.y {
                self.vy = -self.vy.abs();
                self.y = paddle.y - self.radius;
            } else if self.y > paddle.y + paddle.h {
                self.vy = self.vy.abs();
                self.y = paddle.y + paddle.h + self.radius;
            } else if self.x < paddle.x + paddle.w / 2.0 {
                self.vx = -self.vx.abs();
                self.x = paddle.x - self.radius;
            } else {
                self.vx = self.vx.abs();
                self.x = paddle.x + paddle.w + self.radius;
            }

            // Speed up on every hit, but never past the limit.
            self.vx = (self.vx * BALL_SPEEDUP).clamp(-BALL_MAX_SPEED, BALL_MAX_SPEED);
            self.vy = (self.vy * BALL_SPEEDUP).clamp(-BALL_MAX_SPEED, BALL_MAX_SPEED);
        }
    }

    /// Put the ball in the centre and serve it. `direction` is 1.0 (right) or -1.0 (left).
    fn reset(&mut self, direction: f32) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.vx = BALL_START_SPEED_X * direction;

        // Random vertical speed, and a random up/down choice.
        let speed_y: f32 = rand::gen_range(BALL_MIN_SPEED_Y, BALL_MAX_START_SPEED_Y);
        let up_or_down: f32 = if rand::gen_range(0, 2) == 0 {
            1.0
        } else {
            -1.0
        };
        self.vy = speed_y * up_or_down;
    }
}

fn ball_hits_paddle(ball: &Ball, paddle: &Paddle) -> bool {
    let closest_x = ball.x.clamp(paddle.x, paddle.x + paddle.w);
    let closest_y = ball.y.clamp(paddle.y, paddle.y + paddle.h);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    dx * dx + dy * dy < ball.radius * ball.radius
}

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

/// Draw text horizontally centred, with its baseline at `y`.
fn draw_centered_text(text: &str, y: f32) {
    let size = measure_text(text, None, FONT_SIZE as u16, 1.0);
    draw_text(
        text,
        (screen_width() - size.width) / 2.0,
        y,
        FONT_SIZE,
        WHITE,
    );
}

/// A title in the middle of the screen with a hint line below it.
fn draw_message(title: &str, hint: &str) {
    let title_height = measure_text(title, None, FONT_SIZE as u16, 1.0).height;
    let title_y = (screen_height() + title_height) / 2.0;
    draw_centered_text(title, title_y);
    draw_centered_text(hint, title_y + LINE_GAP);
}

#[macroquad::main("Pong")]
async fn main() {
    // Without a seed, the "random" numbers are the same every run.
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut paddle = Paddle::new(PADDLE_MARGIN, PADDLE_SPEED);
    let mut opponent = Paddle::new(
        screen_width() - PADDLE_WIDTH - PADDLE_MARGIN,
        OPPONENT_SPEED,
    );
    let mut ball = Ball::new();

    let mut player_score: u32 = 0;
    let mut opponent_score: u32 = 0;
    let mut state = GameState::Menu;

    loop {
        let pause_pressed = is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape);

        if state == GameState::Playing {
            if pause_pressed {
                state = GameState::Paused;
            } else {
                paddle.update(KeyCode::W, KeyCode::S);
                opponent.follow(ball.y);
                ball.update(&mut player_score, &mut opponent_score);
                ball.bounce_off(&paddle);
                ball.bounce_off(&opponent);

                if player_score >= WIN_SCORE || opponent_score >= WIN_SCORE {
                    state = GameState::GameOver;
                }
            }
        } else if state == GameState::Paused {
            if pause_pressed {
                state = GameState::Playing;
            } else if is_key_pressed(KeyCode::Q) {
                // End the game and go back to the menu.
                state = GameState::Menu;
            }
        } else if is_key_pressed(KeyCode::Space) {
            // From the menu or the game-over screen: start a fresh game.
            player_score = 0;
            opponent_score = 0;
            paddle.reset();
            opponent.reset();
            ball.reset(1.0);
            state = GameState::Playing;
        }

        clear_background(BLACK);

        draw_text(
            player_score.to_string(),
            SCORE_OFFSET_X,
            SCORE_Y,
            FONT_SIZE,
            WHITE,
        );
        draw_text(
            opponent_score.to_string(),
            screen_width() - SCORE_OFFSET_X,
            SCORE_Y,
            FONT_SIZE,
            WHITE,
        );
        paddle.draw();
        opponent.draw();

        // The ball shows while playing or paused, never behind the menu text.
        if state == GameState::Playing || state == GameState::Paused {
            ball.draw();
        }

        match state {
            GameState::Menu => draw_message("PONG", "Press SPACE to start"),
            GameState::GameOver => draw_message("GAME OVER", "Press SPACE to restart"),
            GameState::Paused => {
                // Dim the frozen game so the pause text is easy to read.
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.0, 0.0, 0.0, 0.7),
                );
                draw_message("PAUSED", "P: resume    Q: end game");
            }
            GameState::Playing => {}
        }

        next_frame().await;
    }
}
