use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 0..12 {
            print_months(i);
        }
        thread::sleep(Duration::from_millis(1));
    });

    for i in 0..7 {
        print_days(i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn print_months(i: usize) {
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
    println!("{month}");
}

fn print_days(i: usize) {
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
    println!("{day}");
}
