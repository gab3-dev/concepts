use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        print_months();
        thread::sleep(Duration::from_millis(1));
    });

    print_days();
    thread::sleep(Duration::from_millis(1));
}

fn print_months() {
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

    for i in 0..12 {
        let month = months[i];
        println!("{month}");
    }
}

fn print_days() {
    let days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    for i in 0..7 {
        let day = days[i];
        println!("{day}");
    }
}
