pub struct Vehicle {
    pub position: [f64; 2],
    pub velocity: f64,
    pub angle: f64,
}

impl Vehicle {
    pub fn new(position: [f64; 2], velocity: f64) -> Self {
        Self {
            position,
            velocity,
            angle: 0.0,
        }
    }

    pub fn update_position(&mut self, delta_time: f64) {
        self.position[0] += self.velocity * delta_time;
        self.position[1] += self.velocity * delta_time;
    }
    
    pub fn turn(&mut self, angle: f64) {
        self.angle += angle;
    }
}