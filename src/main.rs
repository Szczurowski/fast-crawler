#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod bank_client;
mod tokio_playground;

use self::bank_client::{Currency, get_currency};


// add some logging, preferably via log@0.4.6, log4rs or !simplelog.rs

#[get("/<name>/<age>")]
fn hello(name: String, age: i32) -> String {
    let eur_value = get_currency(Currency::EUR);
    let message = match eur_value {
        Ok(currency) => currency.to_string(),
        Err(error) => error
    };

    format!("Hello, {} year old named {}! Eur jest po: {}", age, name, message)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}

extern crate futures;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests { 

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn main3() {
        use futures::Future;
        
        let future = futures::future::ok::<i32, i32>(10);
        let mapped = future.map(|i| i * 3);

        let expected = Ok(30);
        assert_eq!(mapped.wait(), expected);
    }
}