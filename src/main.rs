use bracket_lib::prelude::*;

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

struct State {
    mode: GameMode,
    player: Player
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5,20)
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
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
        ctx.cls();
        ctx.print(10,10, "Playing the game");

        self.render_player(ctx);

        // apply gravity to dragon
        self.gravity_and_move();

        // apply horizontal movement to walls
        // we use player.x to simulate horizontal movement
        self.player.x += 1;

        let mut colliding = false;
        // check for collisions with walls
        // check for collision with ground
        if self.player.y > 40 {
            colliding = true;
        }

        if colliding == true {
            self.mode = GameMode::End;
        }

        // handle key presses
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.thrust_up(),
                VirtualKeyCode::P => self.pause(),
                _ => {}
            }
        }
    }
    
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(40,10, "Game over, man");
    }

    fn restart(&mut self) {
        self.player = Player::new(5,20)
        self.mode = GameMode::Playing;
    }

    fn render_player(&mut self, ctx: &mut BTerm) {
        ctx.set(
            5,
            self.player.y,
            YELLOW,
            BLACK,
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
        self.player.velocity -= 2.0;
        if self.player.velocity < -2.0 {
            self.player.velocity = -2.0;
        }
    }

    fn pause(&mut self) {
        // todo apply a pause feature
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
