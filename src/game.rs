pub mod game {
    pub struct Game {
        pub name: String,
        pub price: f64,
        pub grade: i32,
    }

    impl Game {
        pub fn print_name(self){
            let name = &self.name;
            println!("{name}");
        }
    }
}
