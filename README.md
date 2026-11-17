---
category: Game
learned:
  - A game loop reads input, updates the state, then draws, once every frame.
  - Multiplying speeds by the frame time keeps movement the same on any machine.
  - Circle against rectangle collision works by finding the closest point on the rectangle.
  - A game state enum decides what the loop updates and what it draws.
  - A number passed to a function is copied, so changing the caller's value needs a mutable reference.
broke:
  - The ball got trapped behind the right paddle because my bounce code assumed every paddle faced right.
  - The scores stayed at zero because I passed them by value and only changed a copy.
  - The ball passed through the top and bottom of a paddle because I only handled its front face.
  - The game over text sat off centre because draw_text places text by its left edge and baseline.
---

Pong written in Rust with macroquad. You play the left paddle against a computer opponent on the right, and the first side to five points wins. The ball gets ten percent faster on every paddle hit, up to a limit, so long rallies get harder.

## Run it

    cargo run --release

Play in the browser: https://sks-0212.github.io/pong/

Controls:

- **W** and **S** move your paddle.
- **Space** starts a game from the menu, and starts a new one after game over.
- **P** or **Escape** pauses the game. Press either one again to resume.
- **Q** while paused ends the game and goes back to the menu.
- To quit the program, close the window. In the browser, close or leave the tab.

To build the browser version yourself, add the web target, build, and serve the `web` folder with the compiled file saved as `game.wasm`:

    rustup target add wasm32-unknown-unknown
    cargo build --release --target wasm32-unknown-unknown

## How it works

The game is one loop in `main`. Each frame it updates the paddles and the ball, checks for a winner, clears the screen, draws everything, and waits for the next frame. Movement is multiplied by the frame time, so speed is in pixels per second and does not depend on the frame rate.

Two structs hold the data: `Paddle` and `Ball`. The player paddle reads the keyboard. The opponent follows the ball's height and stands still inside a small dead zone, which stops it from shaking. That dead zone is also the difficulty setting. A bigger one makes the opponent lazier.

Collision treats the ball as a circle and the paddle as a rectangle. It finds the point on the rectangle closest to the ball's centre and compares the squared distance with the squared radius. The side that was hit decides which velocity flips, and the ball is pushed out of the paddle so it cannot hit twice.

A `GameState` enum with `Menu`, `Playing` and `GameOver` controls what runs. Updates only happen while playing, and the same Space key starts a game from the menu or the game over screen.

## What I learned

I learned to think of a game as state that changes once per frame. The loop is always the same shape, and every feature is a new piece of state plus one place that updates it and one place that draws it. Using the frame time, clamping positions after moving them, and picking the side of a hit from the ball's centre all came from running the game and watching it fail.

## What broke

Most bugs came from assumptions that were true for one paddle and false for the other. The first bounce code only worked on the front of the left paddle, and when I added the opponent the ball got stuck in the gap behind it until the code checked which side of the paddle the ball was on. The score bug was a lesson in copies: a `u32` passed to a function is copied, so the scores in `main` never changed until the function took `&mut u32`.
