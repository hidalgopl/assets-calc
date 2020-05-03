use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

struct RespError<'a> {
	code: i32,
	msg: &'a str,
	info: &'a str,
}

struct MetalResp {
	gold_ounce: f64,
	silver_ounce: f64,
}

struct Cryptos {
	ADA: f64,
	BTC: f64,
	ETH: f64,
	XMR: f64,
}

struct CryptoResp<'a> {
	success: &'a bool,
	rates: Cryptos,
	error: RespError<'a>,
}

struct FiatResp {
	rates: Fiats,
}

struct Fiats {
	AUD: f64,
	CAD: f64,
}

#[derive(Deserialize, Debug)]
struct MyAssets {
	XMR: f64,
	ADA: f64,
	ETH: f64,
	BTC: f64,
	crypto_pln: f64,
	metal_pln: f64,
	gold_ounces: f64,
	silver_ounces: f64,
	fiat_pln: f64,
	AUD: f64,
	CAD: f64,
}

fn read_assets_from_file<P: AsRef<Path>>(path: P) -> Result<MyAssets, Box<Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let m = serde_json::from_reader(reader)?;
	Ok(m)
}

fn calc_metal_value(amount: &MyAssets, rates: MetalResp) -> f64 {
	amount.gold_ounces * rates.gold_ounce + amount.silver_ounces * rates.silver_ounce
}

fn calc_crypto_value(amount: &MyAssets, rates: CryptoResp) -> f64 {
	amount.XMR*rates.rates.XMR + amount.ETH*rates.rates.ETH + amount.BTC*rates.rates.BTC + amount.ADA*rates.rates.ADA
}

fn calc_fiat_value(amount: &MyAssets, rates: FiatResp) -> f64 {
	amount.CAD/rates.rates.CAD + amount.AUD/rates.rates.AUD
}

fn main() {
    // Read the file contents into a string, returns `io::Result<usize>`
    let my_assets = read_assets_from_file("assets.json").unwrap();
    let fake_metal_resp = MetalResp {
		gold_ounce: 7775.00,
		silver_ounce: 100.00,
	};
	let fake_crypto_rates = Cryptos {
		ADA: 5.23,
		BTC: 3.12,
		ETH: 5234.1,
		XMR: 12.543,
	};
	let fake_crypto_err = RespError {
		code: 200,
		msg: "",
		info: "",
	};
	let fake_crypto_resp = CryptoResp {
		success: &true,
		rates: fake_crypto_rates,
		error: fake_crypto_err,
	};
	let fake_fiats = Fiats {
		AUD: 2.99,
		CAD: 3.11,
	};
	let fake_fiat_resp = FiatResp {
		rates: fake_fiats,
	};
	println!("{}", calc_metal_value(&my_assets, fake_metal_resp));
    //println!("{:#?}", my_assets);
    println!("{}", calc_crypto_value(&my_assets, fake_crypto_resp));
	println!("{}", calc_fiat_value(&my_assets, fake_fiat_resp));
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
