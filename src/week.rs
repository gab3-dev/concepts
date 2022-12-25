pub mod day {
    pub fn print_days(i: usize) {
        let days: [&str; 7] = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];

        let day = days[i];
        println!("{day} and {i}");
    }
}
