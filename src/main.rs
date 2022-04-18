use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const WINDOW_BG: (u8,u8,u8) = NAVY;
const X_DRAW_OFFSET: i32 = 5;
const TERMINAL_VELOCITY: f32 = 2.0;
const GRAVITY: f32 = 0.2;

enum GameMode {
    Menu,
    Playing,
    End
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    frame_number: i32
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

struct State {
    mode: GameMode,
    player: Player,
    paused: bool,
    score: i32,
    obstacle: Obstacle
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            frame_number: 1
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0+X_DRAW_OFFSET,
            self.y,
            YELLOW,
            WINDOW_BG,
            to_cp437(self.frame_number.to_string().chars().nth(0).unwrap())
        );
        
        self.advance_animation();
    }

    fn update(&mut self) {
        self.apply_gravity_to_velocity();
        self.apply_velocity_to_player();
        self.bind_player_to_screen();
        self.horizontally_advance_player();
    }

    fn advance_animation(&mut self) {
        if self.velocity < 0.0 || self.frame_number > 1 {
            self.frame_number += 1;
            if self.frame_number > 4 {
                self.frame_number = 1;
            }
        }
    }

    fn apply_gravity_to_velocity(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }
    }

    fn apply_velocity_to_player(&mut self) {
        self.y += self.velocity as i32;
    }

    fn bind_player_to_screen(&mut self) {
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn horizontally_advance_player(&mut self) {
        self.x += 1;
    }
    
    fn flap(&mut self) {
        self.velocity = -TERMINAL_VELOCITY;
    }
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10,40),
            size: i32::max(2, 20-score)
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(0,20),
            paused: false,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0)
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.render_menu(ctx);
        self.poll_for_menu_input(ctx);
    }

    fn playing(&mut self, ctx: &mut BTerm) {
        self.update();
        self.poll_for_ingame_input(ctx);
        self.render_ingame(ctx);
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        self.player.update();
        self.end_game_if_colliding();
        self.check_score();
        self.spawn_new_obstacle();
    }

    fn poll_for_ingame_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::P => self.pause(),
                _ => {}
            }
        }
    }

    fn render_ingame(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WINDOW_BG);
        self.render_obstacle(ctx);
        self.player.render(ctx);
        ctx.print(1,1, "Press [SPACE] to fly!");
        ctx.print(1,3, &format!("Score: {}", self.score));
    }

    fn check_score(&mut self) {
        if self.player.x == self.obstacle.x {
            self.score += 1;
        }
    }

    fn end_game_if_colliding(&mut self) {
        if self.is_colliding_with_floor() {
            self.mode = GameMode::End;
            return;
        }

        if self.is_colliding_with_obstacle() {
            self.mode = GameMode::End;
        }
    }

    fn spawn_new_obstacle(&mut self) {
        // applying X_DRAW_OFFSET here gives a more natural appearance to the obstacles appearing
        if self.player.x - X_DRAW_OFFSET > self.obstacle.x {
            self.obstacle = Obstacle::new(SCREEN_WIDTH+self.player.x-X_DRAW_OFFSET, self.score);
        }
    }

    fn is_colliding_with_floor(&mut self) -> bool {
        if self.player.y > SCREEN_HEIGHT {
            return true;
        }

        false
    }

    fn is_colliding_with_obstacle(&mut self) -> bool {
        if self.player.x != self.obstacle.x {
            return false;
        }
        
        let half_size = self.obstacle.size / 2;
        if self.player.y < self.obstacle.gap_y - half_size {
            return true;
        }

        if self.player.y > self.obstacle.gap_y + half_size {
            return true;
        }

        false
    }
    
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WINDOW_BG);
        ctx.print(10,10, "Game over, man");
        ctx.print(10,12, &format!("Your score: {}", self.score));
        ctx.print(10,16, "Press [SPACE] to return to the menu");

        // handle key presses
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.mode = GameMode::Menu,
                _ => {}
            }
        }
    }

    fn render_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WINDOW_BG);
        ctx.print(1,1, "Hello, Flappy Dragon!");
        ctx.print(2,5, "(P) Play");
        ctx.print(2,7, "(Q) Quit");
    }

    fn poll_for_menu_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.score = 0;
        self.player = Player::new(0,20);
        self.obstacle = Obstacle::new(SCREEN_WIDTH+self.player.x-X_DRAW_OFFSET, self.score);
        self.mode = GameMode::Playing;
    }

    fn render_obstacle(&mut self, ctx: &mut BTerm) {
        let screen_x = self.obstacle.x - self.player.x + X_DRAW_OFFSET;
        let half_size = self.obstacle.size / 2;
        
        for y in 0..self.obstacle.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RED,
                WINDOW_BG,
                to_cp437('#')
            );
        }

        for y in self.obstacle.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RED,
                WINDOW_BG,
                to_cp437('#')
            );
        }
    }

    fn pause(&mut self) {
        self.paused = !self.paused;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::End => self.game_over(ctx)
        };
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Flappy Dragon")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(32,32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
