pub mod month {
    pub fn print_months(i: usize) {
        let months: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month = months[i];
        println!("{month} and {i}");
    }
}
