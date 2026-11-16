use macroquad::prelude::*;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 80.0;

#[derive(Debug)]
struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Player {
    fn update(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.y = self.y - self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Down) {
            self.y = self.y + self.speed * get_frame_time();
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
    fn update(&mut self) {
        self.x = self.x + self.vx * get_frame_time();
        self.y = self.y + self.vy * get_frame_time();

        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.vx = -self.vx;
        }

        if self.x + self.radius > screen_width() {
            self.x = screen_width() - self.radius;
            self.vx = -self.vx;
        }

        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = -self.vy;
        }

        if self.y + self.radius > screen_height() {
            self.y = screen_height() - self.radius;
            self.vy = -self.vy;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE)
    }

    fn bounce_off(&mut self, player: &Player){
        if ball_hits_player(&self, &player){

            if self.y < player.y{
                self.vy = -self.vy.abs();
                self.y = player.y - self.radius;
            }else if self.y > player.y + player.h{
                self.vy = self.vy.abs();
                self.y = player.y +player.h+self.radius;
            }else{
                self.vx = self.vx.abs();
                self.x = player.x +player.w +self.radius;
            }


        }
    }
}

fn ball_hits_player(ball: &Ball, player: &Player) -> bool {
    let closest_x = ball.x.clamp(player.x, player.x + player.w);
    let closest_y = ball.y.clamp(player.y, player.y + player.h);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    if dx * dx + dy * dy < ball.radius * ball.radius {
        return true;
    }

    false
}

#[macroquad::main("My Game")]
async fn main() {
    let mut player = Player {
        x: 20.0,
        y: (screen_height() - PLAYER_HEIGHT) / 2.0,
        w: PLAYER_WIDTH,
        h: PLAYER_HEIGHT,
        speed: 200.0,
    };

    let mut ball = Ball {
        x: 0.0,
        y: 0.0,
        vx: 150.0,
        vy: 100.0,
        radius: 10.0,
    };

    loop {
        player.update();
        ball.update();
        ball.bounce_off(&player);
        clear_background(BLACK);
        player.draw();
        ball.draw();

        next_frame().await;
    }
}
