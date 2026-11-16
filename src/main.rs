use macroquad::prelude::*;

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
struct Ball{
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}


impl Ball  {
    fn update(&mut self){
        self.x = self.x + self.vx * get_frame_time();
        self.y = self.y +  self.vy * get_frame_time();

        if self.x - self.radius < 0.0{
            self.x = self.radius;
            self.vx  = -self.vx;
        }

        if self.x + self.radius > screen_width(){
            self.x = screen_width() - self.radius;
            self.vx = -self.vx;
        }

        if self.y - self.radius < 0.0{
            self.y = self.radius;
            self.vy = -self.vy;
        }

        if self.y + self.radius > screen_height(){
            self.y = screen_height() - self.radius;
            self.vy = -self.vy;
        }
    }

    fn draw(&self){
        draw_circle(self.x,self.y,self.radius, WHITE)
    }

}

fn ball_hits_player(ball: &Ball, player: &Player)->bool{
    let closest_x = ball.x.clamp(player.x,player.x + player.w);
    let closest_y = ball.y.clamp(player.y, player.y + player.h);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    if dx* dx + dy *dy < ball.radius * ball.radius{
       return true;
    }

    false
}

#[macroquad::main("My Game")]
async fn main() {
    let mut player = Player {
        x: 20.0,
        y: (screen_height() - 80.0) / 2.0,
        w: 15.0,
        h: 80.0,
        speed: 200.0,
    };

    let mut ball = Ball{
        x: 0.0,
        y: 0.0,
        vx: 150.0,
        vy: 100.0,
        radius: 10.0
    };

    loop {
        player.update();
        ball.update();

        clear_background(BLACK);
        player.draw();
        ball.draw();
        if ball_hits_player(&ball, &player){
            draw_text("HIT", 20.0, 40.0, 30.0, WHITE);
        }
        next_frame().await;
    }
}
