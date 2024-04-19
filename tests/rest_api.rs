use serde::Serialize;

#[cfg(feature = "us")]
#[cfg(test)]
mod coinbase_tests {
    use cex_exchanges::{clients::rest_api::ExchangeApi, coinbase::Coinbase};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_all_currencies() {
        let exchange_api = ExchangeApi::new();
        let all_currencies = exchange_api.all_currencies::<Coinbase>().await;
        all_currencies.as_ref().unwrap();
        assert!(all_currencies.is_ok());

        {
            let all_currencies = all_currencies.unwrap();
            let test_length = all_currencies
                .clone()
                .take_coinbase_currencies()
                .unwrap()
                .len();
            assert!(test_length > 10);

            let normalized = all_currencies.clone().normalize();
            let test_length = normalized.clone().take_currencies().unwrap().len();
            assert!(test_length > 10);

            assert_eq!(all_currencies, normalized);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_all_instruments() {
        let exchange_api = ExchangeApi::new();
        let all_instruments = exchange_api.all_instruments::<Coinbase>().await;
        all_instruments.as_ref().unwrap();
        assert!(all_instruments.is_ok());

        {
            let all_instruments = all_instruments.unwrap();
            let normalized = all_instruments.clone().normalize();
            assert_eq!(all_instruments, normalized);
        }
    }
}

#[cfg(feature = "non-us")]
#[cfg(test)]
mod binance_tests {
    use cex_exchanges::{binance::Binance, clients::rest_api::ExchangeApi};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_all_symbols() {
        let exchange_api = ExchangeApi::new();
        let all_symbols = exchange_api.all_currencies::<Binance>().await;
        assert!(all_symbols.is_ok());

        {
            let all_symbols = all_symbols.unwrap();
            let test_length = all_symbols.clone().take_binance_currencies().unwrap().len();
            assert!(test_length > 10);

            let normalized = all_symbols.clone().normalize();
            let test_length = normalized.clone().take_currencies().unwrap().len();
            assert!(test_length > 10);

            assert_eq!(all_symbols, normalized);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_all_instruments() {
        let exchange_api = ExchangeApi::new();
        let all_instruments = exchange_api.all_instruments::<Binance>().await;
        all_instruments.as_ref().unwrap();
        assert!(all_instruments.is_ok());

        {
            let all_instruments = all_instruments.unwrap();
            let test_length = all_instruments
                .clone()
                .take_binance_instruments()
                .unwrap()
                .len();
            assert!(test_length > 10);

            let normalized = all_instruments.clone().normalize();

            let test_length = normalized.clone().take_instruments().unwrap().len();
            assert!(test_length > 10);

            assert_eq!(all_instruments, normalized);
        }
    }
}

#[cfg(feature = "non-us")]
#[cfg(test)]
mod okex_tests {
    use cex_exchanges::{clients::rest_api::ExchangeApi, okex::Okex};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_all_symbols() {
        let exchange_api = ExchangeApi::new();
        let all_symbols = exchange_api.all_currencies::<Okex>().await;
        assert!(all_symbols.is_ok());

        {
            let all_symbols = all_symbols.unwrap();
            let test_length = all_symbols.clone().take_okex_currencies().unwrap().len();
            assert!(test_length > 10);

            let normalized = all_symbols.clone().normalize();
            let test_length = normalized.clone().take_currencies().unwrap().len();
            assert!(test_length > 10);

            assert_eq!(all_symbols, normalized);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_all_instruments() {
        let exchange_api = ExchangeApi::new();
        let all_instruments = exchange_api.all_instruments::<Okex>().await;
        all_instruments.as_ref().unwrap();
        assert!(all_instruments.is_ok());

        {
            let all_instruments = all_instruments.unwrap();
            let test_length = all_instruments
                .clone()
                .take_okex_instruments()
                .unwrap()
                .len();
            assert!(test_length > 10);

            let normalized = all_instruments.clone().normalize();
            let test_length = normalized.clone().take_instruments().unwrap().len();
            assert!(test_length > 10);
        }
    }
}

pub fn write_json<D>(a: D)
where
    D: Serialize
{
    use std::io::Write;

    let mut f0 = std::fs::File::create("/Users/josephnoorchashm/Desktop/SorellaLabs/GitHub/cex-exchanges/t.json").unwrap();

    writeln!(f0, "{}", serde_json::to_string(&a).unwrap()).unwrap();
}
