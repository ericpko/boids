use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

mod boids;
use boids::{Boid, N_BOIDS};
use glam::Vec2;


// Constants
// Time step
// const DT: f64 = 1.0 / 60.0;           // 60 fps
// Window
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = WIDTH / ASPECT_RATIO;   // 1280x720


enum PlayState {
    Play,
    // Pause
}

struct BoidState {
    state: PlayState,
    sprite: graphics::Image,
    dt: std::time::Duration,
    boids: Vec<Boid>
}

impl BoidState {
    fn new(ctx: &mut Context) -> GameResult<BoidState> {
        let s = BoidState {
            state: PlayState::Play,
            sprite: graphics::Image::new(ctx, "/wabbit_alpha.png").unwrap(),
            dt: std::time::Duration::new(0, 0),
            boids: Self::init_boids()
        };
        Ok(s)
    }

    fn init_boids() -> Vec<Boid> {
        std::iter::repeat_with(|| Boid::new(WIDTH, HEIGHT))
                .take(N_BOIDS)
                .collect()
    }
}

impl event::EventHandler<ggez::GameError> for BoidState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ggez::timer::delta(ctx);
        let tick = self.dt.as_secs_f32();
        
        match self.state {
            PlayState::Play => {
                for i in 0..self.boids.len() {
                    let mut b = self.boids[i];
                    b.fly_to_center_of_mass(&self.boids);
                    b.avoid_others(&self.boids);
                    b.match_velocity(&self.boids);
                    b.keep_within_bounds(WIDTH, HEIGHT);
                    b.limit_speed();

                    // update position
                    b.pos += b.vel * tick;
                    self.boids[i] = b;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(30, 30, 30));

        for b in &self.boids {
            let params = graphics::DrawParam::new()
                .dest(b.pos)
                .scale(Vec2::new(0.2, 0.2))
                .rotation(b.vel.x.atan2(-b.vel.y));
            graphics::draw(ctx, &self.sprite, params)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}


pub fn run() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("boids_ggez", "Eric Koehli");

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, event_loop) = cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT)
            .resizable(true))
        .window_setup(ggez::conf::WindowSetup::default().title("Boids!")
            .samples(ggez::conf::NumSamples::Eight)
            .vsync(true))
        .build()?;
    let state = BoidState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
