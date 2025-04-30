use std::{array, mem};

use macroquad::{
    color::{Color, BLACK, WHITE},
    input::{get_char_pressed, is_key_pressed, KeyCode},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::draw_text,
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

    fn update(self) -> Scene {
        if is_key_pressed(KeyCode::Escape) {
            return Scene::End;
        }
        match get_char_pressed() {
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
        let c = rng().random_range(0.0..1.);
        Self {
            color: Color::new(c, c, c, 1.),
        }
    }

    fn draw(&self, w: f32, h: f32, xpad: f32, ypad: f32) {
        let sz = h.min(w) * 0.05;
        draw_rectangle(
            (w - sz) * 0.5 + sz * xpad,
            (h - sz) * 0.5 + sz * ypad,
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

    fn draw(&self, w: f32, h: f32) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tile.draw(w, h, j as f32 - 9.5, i as f32 - 9.5);
            }
        }
    }
}

struct Game {
    chunk: Chunk,
}

impl Game {
    fn new() -> Self {
        Self {
            chunk: Chunk::new(),
        }
    }

    fn update(self) -> Scene {
        if is_key_pressed(KeyCode::Escape) {
            return Scene::Menu(Menu::new());
        }

        Scene::Game(self)
    }

    fn draw(&self, w: f32, h: f32) {
        self.chunk.draw(w, h)
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

    fn update(self) -> Self {
        match self {
            Scene::Menu(menu) => menu.update(),
            Scene::End => self,
            Scene::Game(game) => game.update(),
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
        let scene = mem::replace(&mut self.scene, Scene::End);
        self.scene = scene.update();
    }
}
