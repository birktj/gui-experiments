use ::render::RenderContext;

use std::path::PathBuf;
use std::collections::HashMap;

pub struct State {
    pub path: PathBuf,
    pub active: usize,
    pub scroll: usize,
    pub vspace: usize,
    pub elements: Vec<PathBuf>,
    view_state: HashMap<PathBuf, (usize, usize)>,
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            path: PathBuf::new(),
            active: 0,
            scroll: 0,
            vspace: 0,
            elements: Vec::new(),
            view_state: HashMap::new(),
        };

        state.navigate_to(std::env::current_dir().unwrap());

        state
    }

    pub fn navigate_to_with_focus<P1: Into<PathBuf>, P2: Into<PathBuf>>(&mut self, path: P1, focus: P2) {
        let path = path.into();
        if path.is_dir() {
            self.path = path;
            self.active = 0;
            self.scroll = 0;
            if let Some((active, scroll)) = self.view_state.get(&self.path) {
                self.active = *active;
                self.scroll = *scroll;
            }
            self.update();
        }
    
    }

    pub fn navigate_to<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if path.is_dir() {
            self.path = path;
            self.active = 0;
            self.scroll = 0;
            if let Some((active, scroll)) = self.view_state.get(&self.path) {
                self.active = *active;
                self.scroll = *scroll;
            }
            self.update();
        }
    }

    pub fn update(&mut self) {
	    self.elements = std::fs::read_dir(&self.path).unwrap()
            .map(|d| d.unwrap().path().into())
            .collect();

        self.elements.sort();

        self.active = std::cmp::min(self.active, std::cmp::max(self.elements.len() as i64 - 1, 0) as usize);
        self.scroll = std::cmp::min(self.scroll, std::cmp::max(self.elements.len() as i64 - self.vspace as i64, 0) as usize);
        //self.scroll = std::cmp::min(self.scroll, self.elements.len()-self.vspace);
    }
    
    pub fn layout(&mut self, width: f32, height: f32) {
        self.vspace = ((height - 60.0) / 50.0) as usize;
    }

    pub fn render(&self, ctx: &mut RenderContext) {
        ctx.clear([0.02; 3]);
        ctx.text(10.0, 30.0, &self.path.display().to_string(), [0.8; 3]);
        
        let start = self.scroll;
        let end   = self.scroll + self.vspace;

        for (y, (i, elem)) in self.elements.iter().enumerate().skip(start).take(end - start).enumerate() {
            let y = 60.0 + y as f32 * 50.0;
            let w = ctx.width() - 100.0;
            if i == self.active {
                ctx.rectangle(50.0, y, w, 35.0, [0.03, 0.04, 0.07]);
            }
            else {
                ctx.rectangle(50.0, y, w, 35.0, [0.04; 3]);
            }
            ctx.text(70.0, y + 23.0, &elem.display().to_string(), [0.8; 3]);
        }
    }
    
    pub fn up(&mut self) {
        if self.active > 0 {
            self.active -= 1;
        }
        if self.active - self.scroll < 5 {
            self.scroll = std::cmp::max(self.scroll as i64 - 1, 0) as usize;
        }

        self.view_state.insert(self.path.clone(), (self.active, self.scroll));
    }

    pub fn down(&mut self) {
        if self.active < self.elements.len()-1 {
            self.active += 1;
        }
        if self.active - self.scroll >= self.vspace - 5 {
            self.scroll = std::cmp::min(self.scroll as i64 +1, self.elements.len() as i64 - self.vspace as i64) as usize;
        }

        self.view_state.insert(self.path.clone(), (self.active, self.scroll));
    }

    pub fn right(&mut self) {
        let to = self.elements[self.active].clone();
        self.navigate_to(to);
    }

    pub fn left(&mut self) {
        let mut path = self.path.clone();;
        path.pop();
        self.navigate_to(path);
    }
}
