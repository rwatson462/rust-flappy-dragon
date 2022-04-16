use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const WINDOW_BG: (u8,u8,u8) = NAVY;

enum GameMode {
    Menu,
    Playing,
    End
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0
        }
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
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

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    paused: bool,
    score: i32,
    obstacles: Vec<Obstacle>
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5,20),
            frame_time: 0.0,
            paused: false,
            score: 0,
            obstacles: vec!()
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WINDOW_BG);
        ctx.print(1,1, "Hello, Flappy Dragon!");
        // todo: wait for user input before starting a game
        ctx.print(2,5, "(P) Play");
        ctx.print(2,7, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn playing(&mut self, ctx: &mut BTerm) {
        if !self.paused {
            self.frame_time += ctx.frame_time_ms;
            if self.frame_time > FRAME_DURATION {
                self.update();
            }
        }

        // handle key presses
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.thrust_up(),
                VirtualKeyCode::P => self.pause(),
                _ => {}
            }
        }

        ctx.cls_bg(WINDOW_BG);
        self.render_obstacles(ctx);
        self.render_player(ctx);
        ctx.print(1,1, "Press [SPACE] to fly!");
    }

    fn update(&mut self) {
        self.frame_time = 0.0;

        // add a new obstable if required
        self.add_obstacle();
            
        // apply gravity to dragon
        self.gravity_and_move();

        // apply horizontal movement to walls
        // we use player.x to simulate horizontal movement
        self.player.x += 1;

        if self.is_collision() {
            self.mode = GameMode::End;
        }
    }

    fn is_collision(&mut self) -> bool {
        // check for collisions with walls
        for obstacle in &self.obstacles {
            if self.player.x != obstacle.x {
                continue;
            }
            
            let half_size = obstacle.size / 2;
            if self.player.y < obstacle.gap_y - half_size {
                return true;
            }

            if self.player.y > obstacle.gap_y + half_size {
                return true;
            }
        }

        // check for collision with ground
        if self.player.y > SCREEN_HEIGHT {
            return true;
        }

        false
    }
    
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WINDOW_BG);
        ctx.print(10,10, "Game over, man");
        ctx.print(10,12, "Press [SPACE] to return to the menu");

        // handle key presses
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.mode = GameMode::Menu,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5,20);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn add_obstacle(&mut self) {
        if self.obstacles.len() < 2 {
            self.obstacles.push(Obstacle::new(SCREEN_WIDTH+self.player.x, 10))
        }
    }

    fn render_obstacles(&mut self, ctx: &mut BTerm) {
        let player_x = self.player.x;
        for obstacle in &self.obstacles {
            let screen_x = obstacle.x - player_x;
            let half_size = obstacle.size / 2;
            
            for y in 0..obstacle.gap_y - half_size {
                ctx.set(
                    screen_x,
                    y,
                    RED,
                    WINDOW_BG,
                    to_cp437('|')
                );
            }

            for y in obstacle.gap_y + half_size..SCREEN_HEIGHT {
                ctx.set(
                    screen_x,
                    y,
                    RED,
                    WINDOW_BG,
                    to_cp437('|')
                );
            }

        }
    }

    fn render_player(&mut self, ctx: &mut BTerm) {
        ctx.set(
            5,
            self.player.y,
            YELLOW,
            WINDOW_BG,
            to_cp437('@')
        );
    }

    fn gravity_and_move(&mut self) {
        // apply gravity if less than terminal velocity
        if self.player.velocity < 2.0 {
            self.player.velocity += 0.2;
        }

        // apply velocity to player
        self.player.y += self.player.velocity as i32;

        // make sure we don't fly off the top of the screen
        if self.player.y < 0 {
            self.player.y = 0;
        }
    }

    fn thrust_up(&mut self) {
        self.player.velocity = -2.0;
    }

    fn pause(&mut self) {
        // todo apply a pause feature
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
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context,State::new())
}
