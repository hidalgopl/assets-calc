use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Config {
    fiat_url: String,
    crypto_url: String,
}

#[derive(Deserialize, Debug)]
struct RespError {
    code: i32,
    msg: String,
    info: String,
}

#[derive(Debug, Deserialize)]
struct Cryptos {
    #[serde(rename = "ADA")]
    ada: f64,
    #[serde(rename = "BTC")]
    btc: f64,
    #[serde(rename = "ETH")]
    eth: f64,
    #[serde(rename = "XMR")]
    xmr: f64,
}

#[derive(Debug, Deserialize)]
struct CryptoResp {
    success: bool,
    rates: Cryptos,
}

#[derive(Debug, Deserialize)]
struct FiatResp {
    rates: Fiats,
}

#[derive(Debug, Deserialize)]
struct Fiats {
    #[serde(rename = "AUD")]
    aud: f64,
    #[serde(rename = "CAD")]
    cad: f64,
}

#[derive(Deserialize, Debug)]
struct MyAssets {
    #[serde(rename = "XMR")]
    xmr: f64,
    #[serde(rename = "ADA")]
    ada: f64,
    #[serde(rename = "ETH")]
    eth: f64,
    #[serde(rename = "BTC")]
    btc: f64,
    crypto_pln: f64,
    metal_pln: f64,
    fiat_pln: f64,
    #[serde(rename = "AUD")]
    aud: f64,
    #[serde(rename = "CAD")]
    cad: f64,
}

struct FinancialReport {
    crypto_invested: f64,
    fiat_invested: f64,
    crypto_current: f64,
    fiat_current: f64,
}

impl std::fmt::Display for FinancialReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let dt = chrono::Utc::now();
        let total_invested = self.crypto_invested + self.fiat_invested;
        let current = self.crypto_current + self.fiat_current;
        write!(f, "------------------------------------------------\n\
        | -------------- Financial Report ------------ |\n\
        | ------------ {} ----------- |\n\
        ------------------------------------------------\n\
        | ------ Crypto current value: {:.2} ------- |\n\
        | ------ Crypto invested value: {:.2} ------ |\n\
        | ---- Crypto percentage value: {:.2} % ------ |\n\
        ------------------------------------------------\n\
        | ------- Fiat current value: {:.2} ------- |\n\
        | ------- Fiat invested value: {:.2} ------ |\n\
        | ----- Fiat percentage value: {:.2}  % ----- |\n\
        ------------------------------------------------\n\
        | ---------- Current value: {:.2} --------- |\n\
        | ------ Percent of invested: {:.2} % ------- |\n\
        ------------------------------------------------\n",
               dt.format("%Y-%m-%d %H:%M:%S").to_string(),
               self.crypto_current,
               self.crypto_invested,
               (self.crypto_current * 100.0) / self.crypto_invested,
               self.fiat_current,
               self.fiat_invested,
               (self.fiat_current * 100.0) / self.fiat_invested,
               current,
               (current * 100.0) / total_invested)
    }
}

fn read_assets_from_file<P: AsRef<Path>>(path: P) -> Result<MyAssets, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let m = serde_json::from_reader(reader)?;
    Ok(m)
}

fn calc_crypto_value(amount: &MyAssets, rates: CryptoResp) -> f64 {
    amount.xmr * rates.rates.xmr + amount.eth * rates.rates.eth + amount.btc * rates.rates.btc + amount.ada * rates.rates.ada
}

fn calc_fiat_value(amount: &MyAssets, rates: FiatResp) -> f64 {
    amount.cad / rates.rates.cad + amount.aud / rates.rates.aud
}

fn main() -> Result<(), Box<dyn Error>> {
    smol::run(async {
        let config = match envy::prefixed("ASSETS_").from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("{:#?}", error)
        };
        // Read the file contents into a string, returns `io::Result<usize>`
        let my_assets = read_assets_from_file("assets.json").unwrap();
        let fiat_body: FiatResp = reqwest::get(&config.fiat_url).await?.json().await?;
        let fiat_value: f64 = calc_fiat_value(&my_assets, fiat_body);
        let crypto_body: CryptoResp = reqwest::get(&config.crypto_url).await?.json().await?;
        let crypto_value: f64 = calc_crypto_value(&my_assets, crypto_body);
        let fr: FinancialReport = FinancialReport {
            crypto_current: crypto_value,
            crypto_invested: my_assets.crypto_pln,
            fiat_current: fiat_value,
            fiat_invested: my_assets.fiat_pln,
        };
        println!("{}", fr);
        Ok(())
    })
}
