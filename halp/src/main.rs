
use chrono::{NaiveDate, NaiveDateTime};
use std::io;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn convert_date(ts : &str) {
    let timestamp = ts.parse::<i64>();
    if timestamp.is_err() {
        println!("Error parsing");
    } else {

        println!("{}", NaiveDateTime::from_timestamp(timestamp.unwrap(), 0));
    }
}

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 09, 19).and_hms(3,3,3);
    println!("{}", date_time);
    let mut ctx = ClipboardContext::new().unwrap();
    let the_string = "16000000000";
    ctx.set_contents(the_string.to_owned()).unwrap(); 

    let mut timestamp = String::new();
    match io::stdin().read_line(&mut timestamp) {
        Ok(n) => {
            println!("chars read {}", n);
            println!("String read {}", timestamp);
            let trimmed = timestamp.trim();
            convert_date(trimmed);
        }
        Err(error) => {
            println!("dfsdfsdf");
            println!("error: {}", error);
        }
    }

    println!("Hello, world!");
}
