use raylib::prelude::*;
use rand::Rng;

struct Particle<'a> {
    
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alpha: u8,
    rl: &'a raylib::RaylibHandle
}

impl<'a> Particle<'a> {
    
    pub fn show(&self) {
        
        self.rl.draw_circle_v(vec2(self.x, self.y), 10., Color::new(200, 200, 200, self.alpha));
    }
        
    pub fn update(&mut self) {
        
        self.x += self.vx;
        self.y += self.vy;
        
        if self.alpha > 0 {
            
            self.alpha -= 1;
        }
    }
}


fn main() {

    // initialize raylib
    let rl = raylib::init().size(800, 600).title("Simple Particle").msaa_4x().build();
    let mut rng = rand::thread_rng();

    // vector of particles
    let mut particles: Vec<Particle> = Vec::new();

    // limit frames per second to 60
    rl.set_target_fps(60);

    while !rl.window_should_close() {
    
        for _ in 0..5 {

            // add particles to vector
            particles.push(Particle{
                x: (rl.get_screen_width() / 2) as f32,
                y: (rl.get_screen_height()) as f32,
                vx: rng.gen_range(-1., 1.),
                vy: rng.gen_range(-5., -1.),
                alpha: 255,
                rl: &rl
            });
        }
    
        rl.begin_drawing();
        rl.clear_background(Color::new(20, 20, 20, 255));
    
        // update an show all particles
        for p in &mut particles {
            p.update();
            p.show();
        }
    
        // retain particles as long as alpha value is greater than one
        particles.retain(|x| x.alpha > 1);
    
        rl.draw_text(&particles.len().to_string(), 10, 10, 18, Color::new(200, 200, 200, 255));

        rl.end_drawing();
    }
}
