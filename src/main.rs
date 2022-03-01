use curl::easy::Easy;
use std::io::{stdin};
use scraper::{Html, Selector};

/// Get part of request url by provided league
/// # Examples: 
/// - 1 - Premier league
/// - 2 - Bundesliga
/// - 3 - La Liga
fn get_league_path(league: i8) -> String {
    match league {
        1 => String::from("premier-league-table"),
        2 => String::from("bundesliga-table"),
        3 => String::from("la-liga-table"),
        _ => panic!("Invalid value")
    }
}

/// Request data from skysports.com.
/// * `league` - 1, 2 or 3.
fn request_data(league: i8) -> String {

    let league_path = get_league_path(league);
    let request_url: String = format!("https://www.skysports.com/{}", league_path);

    println!("{}", request_url);

    let mut easy = Easy::new();
    easy.url(&request_url).unwrap();

    let mut html_data: String = String::new();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html_data.push_str(&String::from_utf8(Vec::from(data)).unwrap());
            Ok(data.len())
        }).unwrap();
        
        transfer.perform().unwrap();
    }

    return html_data;
}

fn parse_html(html_string: String) {
    let doc_str: String = format!("{}", html_string);
    let document = Html::parse_document(&doc_str);
    let row_selector = Selector::parse(r#"tr.standing-table__row"#).unwrap();

    for element in document.select(&row_selector) {
        println!("{:?}", element);
    }
}

fn main() {
    let mut is_stopped: bool = false;

    loop {

        if is_stopped {
            break
        }

        println!("---Type 'exit' to quit from CLI---");

        let mut buf = String::new();

        stdin()
            .read_line(&mut buf)
            .expect("Input error");

        is_stopped = buf.trim().eq("exit");

        if !is_stopped {
            match buf.trim().parse::<i8>() {
                Ok(n) => parse_html(request_data(n)),
                Err(_) => {
                    println!("You entered wrong league code. Please, ")
                }
            }
        }

    }


}
