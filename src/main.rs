use rand::prelude::*;
#[derive(Debug)]
struct Player {
    cash: u32,
}

impl Player {
    fn astronaut() -> Self {
        Self { cash: 2 }
    }
}

#[derive(Debug)]
struct Spaceship {
    multiplier: u32,
    crash: u32,
    crashed: bool,
}

impl Spaceship {
    fn ship() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            multiplier: 0,
            crash: rng.gen_range(1..50),
            crashed: false,
        }
    }

    fn increment_multiplier(&mut self) {
        if self.multiplier != self.crash && self.multiplier < self.crash {
            self.multiplier += 1;
        } else {
            self.crashed = true;
        }
    }

    fn exit(&mut self, player: &mut Player) {
        player.cash = self.multiplier * player.cash;
        self.multiplier = 0;
    }
}
fn main() {
    let mut spaceship = Spaceship::ship();
    let mut astronaut = Player::astronaut();
    let mut game_running = true;

    while game_running {
        spaceship.increment_multiplier();
        println!("Multiplier: {:?}", spaceship.multiplier);

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        println!("Exit the ship? Y/N");
        let _ = stdin.read_line(&mut buffer);
        match buffer.trim() {
            "Y" => {
                spaceship.exit(&mut astronaut);
                println!("Cash: {:?}", astronaut.cash);
            }
            "N" => {
                if !spaceship.crashed {
                    println!("Cash: {:?}", astronaut.cash);
                    continue;
                } else {
                    astronaut.cash = 0;
                    println!("You've crashed and lost all your doubloons!");
                    println!("Cash: {:?}", astronaut.cash);
                    game_running = false;
                }
            }
            _ => println!("Y/N!"),
        }
    }
}
