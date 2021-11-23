use super::Request;

pub struct Builder {
    requests: Vec<Request>,
}

impl Builder {
    pub fn new() -> Self {
        ParticleBuilder {
            requests: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: &Request) {
        self.requests.push(request);
    }

    pub fn add_request(&mut self, position: &Position, renderable: &Renderable, lifetime: f32) {
        self.add_request(&Request {
          position,
          renderable,
          lifetime,
        });
    }

    pub fn add_request(&mut self, (x, y): (usize, usize), renderable: Renderable, lifetime: f32) {
        self.add_request(Position {x, y}, renderable, lifetime);
    }

    pub fn add_request(&mut self, (x, y): (usize, usize), renderable: Renderable, lifetime: f32) {
        self.requests.push(Request {
            position: Position {
              x,
              y,
            },
            renderable,
            lifetime,
        });
    }
}
