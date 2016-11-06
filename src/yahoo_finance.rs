extern crate hyper;

use std::io::Read;
use hyper::{Client, Url};

#[allow(dead_code)]
const YAHOO_API: &'static str = "http://query.yahooapis.com/v1/public/yql";
#[allow(dead_code)]
const DATATABLES: &'static str = "store://datatables.org/alltableswithkeys";

#[allow(dead_code)]
struct YqlQuote {
    name: String,
    symbol: String,
    volume: u32,
    average_daily_volume: u32,
    bid: f64,
    ask: f64,
    open: f64,
    previous_close: f64,
    last_trade_price: f64,

}


fn make_query(yahoo_quotes: String, quote: String) -> String {
    let select_from = String::from("SELECT * FROM ");
    let where_symbol = String::from(" WHERE symbol IN ");
    let query = select_from + &yahoo_quotes + &where_symbol + &quote;

    query 
}

fn construct_url(quote: String) -> hyper::Url{
    let yahoo_quotes = String::from("yahoo.finance.quotes");
    let format = String::from("json");
    let mut url = Url::parse(YAHOO_API).unwrap();
    let query = make_query(yahoo_quotes, quote);

    let pairs = vec![
        ("q", query),
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

fn request_url(url: hyper::Url) -> hyper::client::Response {
    let client = Client::new();
    let req = match client.get(url).send() {
        Ok(request) => request,
        Err(..) => panic!("Bad request"),
    };
    
    req
}

fn get_yahoo(quote: String) -> String {
    let url = construct_url(quote);
    let mut res = request_url(url);

    //assert_eq!(res.status, hyper::Ok);

    let mut buffer = String::new();
    match res.read_to_string(&mut buffer) {
        Ok(read) => read,
        Err(..) => panic!("Can't write request to string"),
    };

    buffer
}
