use curl::easy::Easy;
use std::io::{stdin};
use scraper::{Html, Selector};
use prettytable::{Table, Row, Cell, cell, row};

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

    // println!("{}", request_url);

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

// Parse HTML document 
// * `html_string` - html string
fn parse_html(html_string: String) -> Vec<Vec<String>> {
    let doc_str: String = format!("{}", html_string);
    let document = Html::parse_document(&doc_str);
    let row_selector: Selector = Selector::parse(r#"tr.standing-table__row"#).unwrap();

    let mut result: Vec<Vec<String>> = Vec::new();

    for element in document.select(&row_selector) {
        let col_selector: Selector = Selector::parse("td.standing-table__cell").unwrap();
        let columns: Vec<&str> = element
            .select(&col_selector)
            .flat_map(|x| x.text())
            .collect::<Vec<&str>>();
        
        if columns.len() == 0 {
            continue;
        };
        
        let mut tmp_cols: Vec<String> = Vec::new();
        let mut tmp_idx = 0;
        let allow_row_idx: [u8; 7] = [0, 2, 4, 5, 6, 7, 11];

        for col in &columns {
            if allow_row_idx.contains(&tmp_idx) {
                tmp_cols.push(col.to_string());
            }
            tmp_idx += 1;
        }
        
        result.push(tmp_cols);
    }

    return result
}

// Print table in terminal
fn show_table(rows: Vec<Vec<String>>) {
    let mut table = Table::new();

    table.add_row(row!["#", "Team", "PL", "W", "D", "L", "Pts"]);

    for row in &rows {
        let table_cells = row
            .into_iter()
            .map(|c| Cell::new(c))
            .collect();

        table.add_row(Row::new(table_cells));
    }

    table.printstd();
}

fn main() {
    let mut is_stopped: bool = false;

    loop {

        if is_stopped {
            break
        }

        println!(r#"---Type 'exit' to quit from CLI---
        Footable CLI Menu:
        1 - Premier league
        2 - Bundesliga
        3 - La Liga
        -------------------
        "#);

        let mut buf = String::new();

        stdin()
            .read_line(&mut buf)
            .expect("Input error");

        is_stopped = buf.trim().eq("exit");

        if !is_stopped {
            match buf.trim().parse::<i8>() {
                Ok(n) => show_table(parse_html(request_data(n))),
                Err(_) => {
                    println!("You entered wrong league code. Please, ")
                }
            }
        }
    }
}
