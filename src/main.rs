// http://www.vergenet.net/~conrad/boids/pseudocode.html
// https://github.com/eisendaniel/boids
use ggez::GameResult;


pub fn main() -> GameResult {
    boids::run()?;
    Ok(())
}
  