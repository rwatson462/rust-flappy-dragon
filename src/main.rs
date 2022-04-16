use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1, "Hello, Flappy Dragon!");
    }

    fn playing(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(10,10, "Playing the game");
    }
    
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(40,10, "Game over, man");
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
