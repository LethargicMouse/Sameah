use std::{array, mem, ops::AddAssign};

use macroquad::{
    color::{Color, BLACK, WHITE},
    input::{get_char_pressed, is_key_pressed, KeyCode},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, next_frame},
};
use rand::{rng, Rng};

#[macroquad::main("Sameah")]
async fn main() {
    App::run_new().await
}

struct Menu {}

impl Menu {
    fn new() -> Self {
        Self {}
    }

    fn draw(&self, w: f32, h: f32) {
        draw_text("Sameah", w * 0.1, h * 0.1, h * 0.1, WHITE);
        draw_text("[N]ew game", w * 0.15, h * 0.25, h * 0.075, WHITE);
        draw_text("[Q]uit", w * 0.15, h * 0.35, h * 0.075, WHITE);
    }

    fn update(self, char_pressed: Option<char>) -> Scene {
        if is_key_pressed(KeyCode::Escape) {
            return Scene::End;
        }
        match char_pressed {
            Some('q') => return Scene::End,
            Some('n') => return Scene::Game(Game::new()),
            _ => {}
        }
        Scene::Menu(self)
    }
}

struct Tile {
    color: Color,
}

impl Tile {
    fn new() -> Self {
        let c = rng().random_range(0.5..0.8);
        Self {
            color: Color::new(0., c, 0., 1.),
        }
    }

    fn draw(&self, w: f32, h: f32, xpad: f32, ypad: f32, camera: &Camera) {
        let sz = h.min(w) * 0.05;
        draw_rectangle(
            (w - sz) * 0.5 + sz * xpad - w * camera.pos.x,
            (h - sz) * 0.5 + sz * ypad - h * camera.pos.y,
            sz,
            sz,
            self.color,
        );
    }
}

struct Chunk {
    tiles: [[Tile; 20]; 20],
}

impl Chunk {
    fn new() -> Self {
        let tiles = array::from_fn(|_| array::from_fn(|_| Tile::new()));
        Self { tiles }
    }

    fn draw(&self, w: f32, h: f32, camera: &Camera) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tile.draw(w, h, j as f32 - 9.5, i as f32 - 9.5, camera);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct V {
    x: f32,
    y: f32,
}

impl AddAssign for V {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Camera {
    pos: V,
    speed: V,
}

impl Camera {
    const ACC: f32 = 0.2;

    fn new() -> Self {
        Self {
            pos: V { x: 0., y: 0. },
            speed: V { x: 0., y: 0. },
        }
    }

    fn update(&mut self, char_pressed: Option<char>, dt: f32) {
        self.pos += self.speed;
        match char_pressed {
            Some('w') => self.speed.y -= Self::ACC * dt,
            _ => {}
        }
    }
}

struct Game {
    chunk: Chunk,
    camera: Camera,
}

impl Game {
    fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            camera: Camera::new(),
        }
    }

    fn update(mut self, char_pressed: Option<char>, dt: f32) -> Scene {
        self.camera.update(char_pressed, dt);
        if is_key_pressed(KeyCode::Escape) {
            return Scene::Menu(Menu::new());
        }

        Scene::Game(self)
    }

    fn draw(&self, w: f32, h: f32) {
        self.chunk.draw(w, h, &self.camera)
    }
}

enum Scene {
    Menu(Menu),
    End,
    Game(Game),
}

impl Scene {
    fn begin() -> Self {
        Self::Menu(Menu::new())
    }

    fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }

    fn draw(&self, w: f32, h: f32) {
        match self {
            Scene::Menu(menu) => menu.draw(w, h),
            Scene::End => {}
            Scene::Game(game) => game.draw(w, h),
        }
    }

    fn update(self, char_pressed: Option<char>, dt: f32) -> Self {
        match self {
            Scene::Menu(menu) => menu.update(char_pressed),
            Scene::End => self,
            Scene::Game(game) => game.update(char_pressed, dt),
        }
    }
}

struct App {
    scene: Scene,
}

impl App {
    async fn run_new() {
        Self::new().run().await
    }

    fn new() -> Self {
        Self {
            scene: Scene::begin(),
        }
    }

    async fn run(&mut self) {
        while !self.scene.is_end() {
            self.update();
            self.draw();
            next_frame().await
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        let (w, h) = screen_size();
        self.scene.draw(w, h)
    }

    fn update(&mut self) {
        let char_pressed = get_char_pressed();
        let dt = get_frame_time();
        let scene = mem::replace(&mut self.scene, Scene::End);
        self.scene = scene.update(char_pressed, dt);
    }
}
