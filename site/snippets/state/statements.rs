use hypersig::html::button;
use hypersig::{Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Player {
    score: i32,
    lives: i32,
    over: bool,
}

let score = Player::score();
let lives = Player::lives();
let over = Player::over();

let hit = lives
    .set(lives - 1)
    .then(Stmt::when(lives.le(0), over.set(true)));
let bonus = Stmt::if_else(over, Stmt::none(), score.set(score + 100));

let play = button::<Player>().on_click(bonus.then(hit)).text("Play");
# let _ = play;
