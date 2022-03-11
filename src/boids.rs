use glam::Vec2;
use rand::Rng;

// Constants
pub const N_BOIDS: usize = 300;
const SPEED_LIMIT: f32 = 250.0;     // pixels per second
const MIN_DISTANCE: f32 = 9.0;     // pixels
const VISUAL_RANGE: f32 = 16.0;     // pixels         12 is good too

#[derive(Clone, Copy, PartialEq)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    pub fn new(win_width: f32, win_height: f32) -> Self {
        let mut rng = rand::thread_rng();

        let pos = Vec2::new(
            rng.gen_range(50.0..win_width-50.0),
            rng.gen_range(50.0..win_height-50.0),
        );
        let vel = Vec2::new(
            (rng.gen::<f32>() - 0.5) * SPEED_LIMIT,
            (rng.gen::<f32>() - 0.5) * SPEED_LIMIT,
        );

        Self { pos, vel }
    }

pub fn fly_to_center_of_mass(&mut self, boids: &[Boid]) { 
        let centering_factor = 0.05;
        let mut center = Vec2::new(0.0, 0.0);
        let mut num_neighbors = 0.0;
        for b in boids {
            if self.pos.distance(b.pos) < VISUAL_RANGE {
                center += b.pos;
                num_neighbors += 1.0;
            }
        }
        if num_neighbors > 0.0 {
            center /= num_neighbors;
            self.vel += (center - self.pos) * centering_factor;
        }

        // let mut perceived_center = Vec2::new(0.0,0.0);
        // for b in boids {
        //     if self == b {
        //         continue;
        //     }
        //     perceived_center += b.pos;
        // }
        // perceived_center /= (N_BOIDS as f32) - 1.0;

        // self.vel += (perceived_center - self.pos) * 0.0005;
    }

    pub fn avoid_others(&mut self, boids: &[Boid]) {
        let avoid_factor = 0.5;
        let mut diff = Vec2::new(0.0, 0.0);
        for b in boids {
            let distance = self.pos.distance(b.pos);
            if distance < MIN_DISTANCE && distance > 0.0 {
                diff += self.pos - b.pos;
            }
        }
        self.vel += diff * avoid_factor;

        // let mut c = Vec2::new(0.0, 0.0);
        // for b in boids {
        //     if self == b {
        //         continue;
        //     }
        //     if (b.pos - self.pos).length() < MIN_DISTANCE {
        //         c = c - (b.pos - self.pos);
        //     }
        // }
        // self.vel += c;
    }

    pub fn match_velocity(&mut self, boids: &[Boid]) {
        let matching_factor = 0.1;
        let mut avg_vel = Vec2::new(0.0, 0.0);
        let mut num_neighbors = 0.0;
        for b in boids {
            if self.pos.distance(b.pos) < VISUAL_RANGE {
                avg_vel += b.vel;
                num_neighbors += 1.0;
            }
        }
        if num_neighbors > 0.0 {
            avg_vel /= num_neighbors;
            self.vel += (avg_vel - self.vel) * matching_factor;
        }
    }

    pub fn keep_within_bounds(&mut self, win_width: f32, win_height: f32) {
        let edge_buffer = 40.0;
        let turn_factor = 16.0;
        let mut x_bounded = true;
        let mut y_bounded = true;

        if self.pos.x < win_width - edge_buffer {
            self.vel.x += turn_factor;
            x_bounded = !x_bounded;
        }
        if self.pos.x > edge_buffer {
            self.vel.x -= turn_factor;
            x_bounded = !x_bounded;
        }
        if self.pos.y < win_height - edge_buffer {
            self.vel.y += turn_factor;
            y_bounded = !y_bounded;
        }
        if self.pos.y > edge_buffer {
            self.vel.y -= turn_factor;
            y_bounded = !y_bounded;
        }
        if !x_bounded {
            self.vel.x *= 0.8;
        }
        if !y_bounded {
            self.vel.y *= 0.8;
        }
    }

    pub fn limit_speed(&mut self) {
        let speed = self.vel.length();
        if speed > SPEED_LIMIT {
            self.vel = (self.vel / speed) * SPEED_LIMIT;
        }
    }
}
