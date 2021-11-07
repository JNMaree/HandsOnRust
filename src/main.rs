#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;

//System Constants
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;
//Player Constants
const X_OFFSET: i32 = 4;
const X_VELOCITY_INCLINE:f32 = 2.0;
const Y_GRAV_VELOCITY: f32 = 1.0;
const Y_TERMINAL_VELOCITY: f32 = 1.8;
const Y_FLAP_VELOCITY: f32 = 4.0;
//Obstacle Constants
const OBS_MIN_SIZE: i32 = 4;
const OBS_MAX_SIZE: i32 = 20;
const OBS_SIZE_DECLINE: i32 = 2;

enum GameMode {
    Menu,
    Playing,
    End,
}

// State    -----------------------------------------------------------------------------------
struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
    speed: f32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player:Player::new(X_OFFSET,SCREEN_HEIGHT/2),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
            speed: 0.0,
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
        self.player = Player::new(X_OFFSET,SCREEN_HEIGHT/2);
        self.frame_time = 0.0;
        self.score = 0;
        self.speed = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > (FRAME_DURATION - self.speed) {   // Set new Frame
            self.frame_time = 0.0;
            self.player.gravity_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {  //Trigger flap function
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0, "Press [SPACE] to flap!");
        ctx.print(0,1, format!("SCore: {}", self.score));

        // Obstacle & Score Functions
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
            self.speed += X_VELOCITY_INCLINE;
        }

        // End Game Loop conditions
        if self.player.y > SCREEN_HEIGHT {  // If player reaches vertical screen limit
            self.mode = GameMode::End;
        }
        if self.obstacle.hit_obstacle(&self.player) { // If Obstacle hit
            self.mode = GameMode::End;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();  //Play again menu
        ctx.print_centered(4, "You Died!");
        ctx.print_centered(6, &format!("You Earned {} points!", self.score));
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



// Player   -----------------------------------------------------------------------------------
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Player {
    fn new(x: i32, y:i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW1, BLANCHED_ALMOND, to_cp437('@'));
    }
    fn gravity_move(&mut self) {
        if self.velocity < Y_TERMINAL_VELOCITY {    //Fall Velocity
            self.velocity += Y_GRAV_VELOCITY;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -Y_FLAP_VELOCITY;
    }
}

// Obstacles    -------------------------------------------------------------------------------
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(OBS_MIN_SIZE, OBS_MAX_SIZE - score/OBS_SIZE_DECLINE),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let sizeD2 = self.size/2;

        for y in 0..(self.gap_y - sizeD2){ // Draw TOP half
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in (self.gap_y + sizeD2)..SCREEN_HEIGHT { // Draw BOTTOM half
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

    }
    fn hit_obstacle(&self, player: &Player) -> bool {
        let sizeD2 = self.size/2;
        if player.x == self.x {
            if player.y > (self.gap_y + sizeD2) || player.y < (self.gap_y - sizeD2) {
                return true
            }
            return false
        }
        return false
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dargon").build()?;
    main_loop(context, State::new())
}
