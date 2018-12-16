
extern crate hyper;

use hyper::Client;
use hyper::rt::{self, Future, Stream};

pub enum Currency {
    EUR
}

pub fn get_currency(_currency: Currency) -> Result<f64, String> {
    let _line = get_request()?;
    // TODO: extract the currency
    let currency_part = "3.4123";
    let result = currency_part.parse().unwrap();

    Ok(result)
}

fn get_request() -> Result<String, String> {
    let client = Client::new();
    let url = "http://httpbin.org/ip".parse().unwrap();
    let fut = client

        // Make a GET /ip to 'http://httpbin.org'
        .get(url)

        // And then, if the request gets a response...
        .and_then(|res| {
            println!("status: {}", res.status());

            // Concatenate the body stream into a single buffer...
            // This returns a new future, since we must stream body.
            res.into_body().concat2()
        })

        // And then, if reading the full body succeeds...
        .and_then(|body| {
            // The body is just bytes, but let's print a string...
            let s = std::str::from_utf8(&body)
                .expect("httpbin sends utf-8 JSON");

            println!("body: {}", s);

            // and_then requires we return a new Future, and it turns
            // out that Result is a Future that is ready immediately.
            Ok(())
        })

        // Map any errors that might have happened...
        .map_err(|err| {
            println!("error: {}", err);
        });

    // A runtime is needed to execute our asynchronous code. In order to
    // spawn the future into the runtime, it should already have been
    // started and running before calling this code.
    rt::spawn(fut);
    Ok(String::from(""))
}
