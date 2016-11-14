extern crate hyper;
extern crate itertools;
extern crate serde_json;

use std::io::Read;
use itertools::join;
//use serde_json::Value;
use hyper::{Client, Url};

const YAHOO_API: &'static str = "http://query.yahooapis.com/v1/public/yql";
const DATATABLES: &'static str = "store://datatables.org/alltableswithkeys";

#[allow(dead_code)]
struct YqlQuote {
    name: String,
    symbol: String,
    currency: String,
    stock_exchange: String,

    volume: u64,
    average_daily_volume: f64,

    bid: f64,
    ask: f64,

    open: f64,
    previous_close: f64,
    last_trade_price: f64,

    ma50: f64,
    ma200: f64,
    ebitda: String,

    day_low: f64,
    day_high: f64,
    year_low: f64,
    year_high: f64,

    dividend_share: f64,
    dividend_yield: f64,
    earnings_share: f64,
    book_value: f64,
    pe_ratio: f64,
    price_sales: f64,
    price_book: f64,
    peg: f64,
    short_ratio: f64,
    one_yr_target: f64,
}

fn vec_to_string(quotes: Vec<String>) -> String {
    let quotes_str = join(quotes, ",");

    quotes_str
}

pub fn make_query(quote: String) -> String {
    let select_from = String::from("SELECT * FROM ");
    let where_symbol = String::from(" WHERE symbol IN ");
    let yahoo_quotes = String::from("yahoo.finance.quotes");
    let query = select_from + &yahoo_quotes + &where_symbol +"(\"" +  &quote + "\")";

    query 
}

// Yahoo YQL 
pub fn construct_url(quotes: String) -> hyper::Url{
    let format = String::from("json");
    let mut url = Url::parse(YAHOO_API).unwrap();

    let pairs = vec![
        ("q", make_query(quotes)),
        ("format", format),
        ("env", DATATABLES.to_string()),
    ];
    
    url.query_pairs_mut()
        .clear()
        .extend_pairs(pairs
                      .iter()
                      .cloned()
                      );
    url
}

pub fn request_url(url: hyper::Url) -> hyper::client::Response {
    let client = Client::new();
    let req = match client.get(url).send() {
        Ok(request) => request,
        Err(..) => panic!("Bad request"),
    };
    
    req
}

fn yahoo_json(quote: Vec<String>) -> String {
    let url = construct_url(vec_to_string(quote));
    let mut res = request_url(url);

    let mut buffer = String::new();
    match res.read_to_string(&mut buffer) {
        Ok(read) => read,
        Err(..) => panic!("Can't write request to string"),
    };

    buffer
}

#[cfg(test)]
mod tests {
    extern crate hyper;
    use super::*;

#[test]
    fn test_url() {
        let quotes = String::from("AAPL");

        let url = construct_url(quotes);
        let request = request_url(url);

        assert_eq!(request.status, hyper::Ok);
    }

#[test]
    fn test_query() {
        //let url = "http://query.yahooapis.com/v1/public/yql?q=SELECT+*+FROM+yahoo.finance.quotes+WHERE+symbol+IN+(\"AAPL,\")&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys";

        let quotes = String::from("AAPL");

        let query = make_query(quotes);

        assert_eq!(query, "SELECT * FROM yahoo.finance.quotes WHERE symbol IN (\"AAPL\")");
    }
}
