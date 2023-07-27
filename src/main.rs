mod pac_man;
use pac_man::game::PacMan;

fn main() {
    let mut pac_man = PacMan::new();
    pac_man.setup();
    pac_man.run();
}
