use macroquad::prelude::*;

const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;
const BALL_RADIUS: f32 = 10.0;

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Paddle {
    fn update(&mut self, up: KeyCode, down: KeyCode) {
        if is_key_down(up) {
            self.y = self.y - self.speed * get_frame_time();
        }

        if is_key_down(down) {
            self.y = self.y + self.speed * get_frame_time();
        }

        self.y = self.y.clamp(0.0, screen_height() - self.h);
    }

    fn follow(&mut self, target_y: f32){
        let center = self.y + self.h / 2.0;
        let distance = (target_y - center).abs();
        let dead_zone = 5.0;
        if distance < dead_zone {

        }else if target_y < center {
            self.y = self.y - self.speed * get_frame_time();
        }else{
            self.y = self.y + self.speed * get_frame_time();
        }

        self.y =self.y.clamp( 0.0, screen_height() - self.h);
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
    fn update(&mut self, player_score: &mut u32, opponent_score: &mut u32) {
        self.x = self.x + self.vx * get_frame_time();
        self.y = self.y + self.vy * get_frame_time();


        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = -self.vy;
        }

        if self.y + self.radius > screen_height() {
            self.y = screen_height() - self.radius;
            self.vy = -self.vy;
        }

        if self.x > screen_width(){
            *player_score += 1;
            self.reset();
        }

        if self.x < 0.0{
            *opponent_score += 1;
            self.reset();
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE)
    }

    fn bounce_off(&mut self, paddle: &Paddle){
        if ball_hits_paddle(&self, &paddle){

            if self.y < paddle.y{
                self.vy = -self.vy.abs();
                self.y = paddle.y - self.radius;
            }else if self.y > paddle.y + paddle.h{
                self.vy = self.vy.abs();
                self.y = paddle.y +paddle.h+self.radius;
            }else{

                if self.x < (paddle.x +paddle.w /2.0){
                    self.vx = -self.vx.abs();
                    self.x = paddle.x - self.radius;
                }else{
                    self.vx = self.vx.abs();
                    self.x = paddle.x + paddle.w + self.radius;
                }

            }

            self.vx = self.vx * 1.1;
            self.vy = self.vy * 1.1;

        }
    }

    fn reset(&mut self){
        self.x = (screen_width() - self.radius)/2.0;
        self.y = (screen_height() - self.radius)/2.0;
        self.vx = 150.0;
        self.vy = 100.0;
    }
}

fn ball_hits_paddle(ball: &Ball, paddle: &Paddle) -> bool {
    let closest_x = ball.x.clamp(paddle.x, paddle.x + paddle.w);
    let closest_y = ball.y.clamp(paddle.y, paddle.y + paddle.h);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    if dx * dx + dy * dy < ball.radius * ball.radius {
        return true;
    }

    false
}

#[macroquad::main("My Game")]
async fn main() {
    let mut paddle = Paddle {
        x: 20.0,
        y: (screen_height() - PADDLE_HEIGHT) / 2.0,
        w: PADDLE_WIDTH,
        h: PADDLE_HEIGHT,
        speed: 200.0,
    };

    let mut ball = Ball {
        x : (screen_width() - BALL_RADIUS)/2.0,
        y : (screen_height() - BALL_RADIUS)/2.0,
        vx: 150.0,
        vy: 100.0,
        radius: BALL_RADIUS,
    };

    let mut opponent = Paddle{
        x: screen_width() - PADDLE_WIDTH - 20.0,
        y: (screen_height() - PADDLE_HEIGHT) /2.0,
        w: PADDLE_WIDTH,
        h: PADDLE_HEIGHT,
        speed: 200.0
    };

    let mut player_score: u32 = 0;
    let mut opponent_score: u32 = 0;


    loop {
        paddle.update(KeyCode::W, KeyCode::S);
        opponent.follow(ball.y);
        ball.update(&mut player_score, &mut opponent_score);
        ball.bounce_off(&paddle);
        ball.bounce_off(&opponent);

        clear_background(BLACK);
        draw_text(format!("{}",player_score),   200.0, 40.0, 40.0, WHITE);
        draw_text(format!("{}",opponent_score),   screen_width() - 200.0, 40.0, 40.0, WHITE);
        paddle.draw();
        opponent.draw();
        ball.draw();
        next_frame().await;
    }
}
