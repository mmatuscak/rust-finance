extern crate serde_json;
extern crate hyper;
extern crate itertools;

use std::io::Read;
use std::error::Error;
use itertools::join;
use serde_json::Value;
use hyper::{Client, Url};

const YAHOO_API: &'static str = "http://query.yahooapis.com/v1/public/yql";
const DATATABLES: &'static str = "store://datatables.org/alltableswithkeys";

#[derive(Deserialize, Debug)]
pub struct YahooHist {
    #[serde(rename="Date")]
    date: Option<String>,
    #[serde(rename="High")]
    high: Option<f64>,
    #[serde(rename="Low")]
    low: Option<f64>,
    #[serde(rename="Close")]
    close: Option<f64>,
    #[serde(rename="Volume")]
    volume: Option<u64>,
    #[serde(rename="Adj_Close")]
    adj_close: Option<f64>,
}


#[derive(Deserialize, Debug)]
pub struct YahooQuote {
    #[serde(rename="Name")]
    name: Option<String>,
    #[serde(rename="Symbol")]
    symbol: Option<String>,
    #[serde(rename="Volume")]
    volume: Option<u64>,
    #[serde(rename="AverageDailyVolume")]
    average_daily_volume: Option<u64>,
    #[serde(rename="Bid")]
    bid: Option<f64>,

    #[serde(rename="Ask")]
    ask: Option<f64>,
    #[serde(rename="Open")]
    open: Option<f64>,
    #[serde(rename="PreviousClose")]
    previous_close: Option<f64>,
    #[serde(rename="LastTradePriceOnly")]
    last_trade_price: Option<f64>,
    #[serde(rename="FiftydayMovingAverage")]
    ma50: Option<f64>,

    #[serde(rename="TwoHundreddayMovingAverage")]
    ma200: Option<f64>,
    #[serde(rename="EBITDA")]
    ebitda: Option<String>,
    #[serde(rename="DaysLow")]
    day_low: Option<f64>,
    #[serde(rename="DaysHigh")]
    day_high: Option<f64>,
    #[serde(rename="YearLow")]
    year_low: Option<f64>,

    #[serde(rename="YearHigh")]
    year_high: Option<f64>,
    #[serde(rename="DividendShare")]
    dividend_share: Option<f64>,
    #[serde(rename="DividendYield")]
    dividend_yield: Option<f64>, 
    #[serde(rename="EarningsShare")]
    earnings_share: Option<f64>,
    #[serde(rename="BookValue")]
    book_value: Option<f64>,

    #[serde(rename="PERatio")]
    pe_ratio: Option<f64>,
    #[serde(rename="PriceSales")]
    price_sales: Option<f64>,
    #[serde(rename="PriceBook")]
    price_book: Option<f64>,
    #[serde(rename="PEGRatio")]
    peg: Option<f64>,
    #[serde(rename="ShortRatio")]
    short_ratio: Option<f64>,

}
impl YahooHist {
    fn deserialize_hist(quote: &str) -> YahooQuote {
        let yql_hist: YahooHist = match serde_json::from_str(quote) {
            Ok(yql) => yql,
            Err(err) => panic!("Yql panic: {:?}", err),
        };

        yql_hist
    }
}

impl YahooQuote {
    pub fn name(&self) -> Option<String> { self.name.clone() }
    pub fn symbol(&self) -> Option<String> { self.symbol.clone() }
    pub fn volume(&self) -> Option<u64> { self.volume.clone() }
    pub fn average_daily_volume(&self) -> Option<u64> { self.average_daily_volume.clone() }
    pub fn bid(&self) -> Option<f64> { self.bid.clone() }
    pub fn ask(&self) -> Option<f64> { self.ask.clone() }
    pub fn open(&self) -> Option<f64> { self.open.clone() }
    pub fn previous_close(&self) -> Option<f64> { self.previous_close.clone() }
    pub fn last_trade_price(&self) -> Option<f64> { self.last_trade_price.clone() }
    pub fn ma50(&self) -> Option<f64> { self.ma50.clone() }
    pub fn ma200(&self) -> Option<f64> { self.ma200.clone() }
    pub fn ebitda(&self) -> Option<String> { self.ebitda.clone() }
    pub fn day_low(&self) -> Option<f64> { self.day_low.clone() }
    pub fn day_high(&self) -> Option<f64>  { self.day_high.clone() }
    pub fn year_low(&self) -> Option<f64>  { self.year_low.clone() }
    pub fn year_high(&self) -> Option<f64>  { self.year_high.clone() }
    pub fn dividend_share(&self) -> Option<f64>  { self.dividend_share.clone() }
    pub fn dividend_yield(&self) -> Option<f64>  { self.dividend_yield.clone() }
    pub fn earnings_share(&self) -> Option<f64>  { self.earnings_share.clone() }
    pub fn book_value(&self) -> Option<f64>  { self.book_value.clone() }
    pub fn pe_ratio(&self) -> Option<f64>  { self.pe_ratio.clone() }
    pub fn price_sales(&self) -> Option<f64> { self.price_sales.clone() }
    pub fn price_book(&self) -> Option<f64> { self.price_book.clone() }
    pub fn peg(&self) -> Option<f64> { self.peg.clone() }
    pub fn short_ratio(&self) -> Option<f64> { self.short_ratio.clone() }

    fn deserialize_quote(quote: &str) -> YahooQuote {
        let yql_quote: YahooQuote = match serde_json::from_str(quote) {
            Ok(yql) => yql,
            Err(err) => panic!("Yql panic: {:?}", err),
        };

        yql_quote
    }

    fn json_quote(query: &str) -> String {

         let data2: Value = serde_json::from_str(query).expect("Query Value Error");
         let obj2 = data2.as_object().unwrap();
         let results = obj2.get("results").unwrap().to_string();
         let count = obj2.get("count").unwrap();

         let data3: Value = serde_json::from_str(&results).expect("Result Value Error");
         let obj3 = data3.as_object().unwrap();
         let quote = &obj3.get("quote").unwrap();

         quote.to_string()


    }

    fn json_query(json: &str) -> String {
     let data: Value = serde_json::from_str(json).expect("Bad JSON Value");
     let obj = data.as_object().unwrap();
     let query = obj.get("query").unwrap();

     query.to_string()
    }

    pub fn new(symbol: &str) -> Result<YahooQuote, Box<Error>> {
        if symbol.is_empty() {
            panic!("No symbol provided.");
        }

        //let url = YahooQuote::construct_url(&YahooQuote::vec_to_string(symbols));
        let url = construct_url(symbol);
        let mut res = try!(request_url(url));

        let mut buffer = String::new();
        match res.read_to_string(&mut buffer) {
            Ok(read) => read,
            Err(..) => panic!("Can't write request to string"),
        };

        let query = YahooQuote::json_query(&buffer);
        //let (quote, count) = json_quote(&query);
        let quote = YahooQuote::json_quote(&query);

        Ok(YahooQuote::deserialize_quote(&quote))
    }

}

#[allow(dead_code)]
fn vec_to_string(quotes: Vec<&str>) -> String {
    let quotes_str = join(quotes, ",");

    quotes_str
}

pub fn make_query(quote: &str) -> String {
    let select_from = "SELECT * FROM ";
    let where_symbol = " WHERE symbol IN ";
    let yahoo_quotes = "yahoo.finance.quotes";
    let query = select_from.to_string() + &yahoo_quotes + &where_symbol + "(\"" + &quote + "\")";

    query
}

// Yahoo YQL
fn construct_url(quotes: &str) -> hyper::Url {
    let format = String::from("json");
    let mut url = match Url::parse(YAHOO_API) {
        Ok(url) => url,
        Err(..) => panic!("Can't parse url"),
    };

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

fn request_url(url: hyper::Url) -> Result<hyper::client::Response, hyper::error::Error> {
    let client = Client::new();
    client.get(url)
        .send()
        .map(|req| req)

}

#[test]
fn test_json() {
    let share = YahooQuote::new("GOOG").unwrap();

    assert_eq!(share.name().unwrap(), "Alphabet Inc.".to_string());
    assert_eq!(share.symbol().unwrap(), "GOOG".to_string());
    assert!(share.last_trade_price().unwrap() > 0.0);
}

#[test]
#[should_panic]
fn test_empty() {
    YahooQuote::new("").unwrap();
}


#[test]
fn test_url() {
    let quotes = "YHOO";

    let url = construct_url(quotes);
    let request = request_url(url).unwrap();

    assert_eq!(request.status, hyper::Ok);
}

#[test]
fn test_query() {
    let quotes = "MSFT";
    let query = make_query(quotes);

    assert_eq!(query, "SELECT * FROM yahoo.finance.quotes WHERE symbol IN (\"MSFT\")");
}
