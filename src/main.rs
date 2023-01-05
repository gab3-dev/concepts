use std::thread;
use std::time::Duration;

mod game;
mod months;
mod week;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..12 {
            months::month::print_months(i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..7 {
        week::day::print_days(i);
        thread::sleep(Duration::from_millis(1));
    }

    let game = game::game::Game{
        name: "Call of duty".to_string(),
        price: 249.99,
        grade: 5,
    };

    game.print_name();

    handle.join().unwrap();
}
