#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(4, "Play Flappy Dargon");
        ctx.print_centered(8, "[P] - Play Game");
        ctx.print_centered(10, "[Q] - Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => self.quit(ctx),
                _ => {}
            }
        }
    }
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
    fn play(&mut self, ctx: &mut BTerm) {
        //TODO
        self.mode = GameMode::End;
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(4, "You Died!");
        ctx.print_centered(8, "[P] to Play again");
        ctx.print_centered(10, "[Q] to Quit ...");

        let key = ctx.key;
        match key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => self.quit(ctx),
            _ => {}
        }
    }
    fn quit(&mut self, ctx: &mut BTerm) {
        ctx.quit();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dargon")
        .build()?;
    main_loop(context, State::new())
}
