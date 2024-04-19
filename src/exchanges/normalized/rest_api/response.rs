use serde::Serialize;

use super::NormalizedRestApiDataTypes;
#[cfg(feature = "non-us")]
use crate::exchanges::binance::rest_api::BinanceRestApiResponse;
#[cfg(feature = "us")]
use crate::exchanges::{coinbase::rest_api::CoinbaseRestApiResponse, okex::rest_api::OkexRestApiResponse};
use crate::{
    binance::rest_api::{BinanceCompleteInstrument, BinanceSymbol},
    coinbase::rest_api::{CoinbaseCompleteInstrument, CoinbaseCurrency},
    normalized::types::NormalizedCurrency,
    okex::rest_api::OkexCompleteInstrument
};

#[derive(Debug, Clone, Serialize)]
pub enum CombinedRestApiResponse {
    #[cfg(feature = "us")]
    Coinbase(CoinbaseRestApiResponse),
    #[cfg(feature = "us")]
    Okex(OkexRestApiResponse),
    #[cfg(feature = "non-us")]
    Binance(BinanceRestApiResponse)
}

impl CombinedRestApiResponse {
    pub fn normalize(self) -> NormalizedRestApiDataTypes {
        match self {
            #[cfg(feature = "us")]
            CombinedRestApiResponse::Coinbase(c) => c.normalize(),
            #[cfg(feature = "us")]
            CombinedRestApiResponse::Okex(c) => c.normalize(),
            #[cfg(feature = "non-us")]
            CombinedRestApiResponse::Binance(c) => c.normalize()
        }
    }

    pub fn take_coinbase(self) -> Option<CoinbaseRestApiResponse> {
        match self {
            CombinedRestApiResponse::Coinbase(vals) => Some(vals),
            _ => None
        }
    }

    pub fn take_coinbase_instruments(self) -> Option<Vec<CoinbaseCompleteInstrument>> {
        self.take_coinbase().map(|v| v.take_instruments()).flatten()
    }

    pub fn take_coinbase_currencies(self) -> Option<Vec<CoinbaseCurrency>> {
        self.take_coinbase().map(|v| v.take_currencies()).flatten()
    }

    pub fn take_binance(self) -> Option<BinanceRestApiResponse> {
        match self {
            CombinedRestApiResponse::Binance(vals) => Some(vals),
            _ => None
        }
    }

    pub fn take_binance_currencies(self) -> Option<Vec<BinanceSymbol>> {
        self.take_binance().map(|v| v.take_currencies()).flatten()
    }

    pub fn take_binance_instruments(self) -> Option<Vec<BinanceCompleteInstrument>> {
        self.take_binance().map(|v| v.take_instruments()).flatten()
    }

    pub fn take_okex(self) -> Option<OkexRestApiResponse> {
        match self {
            CombinedRestApiResponse::Okex(vals) => Some(vals),
            _ => None
        }
    }

    pub fn take_okex_instruments(self) -> Option<Vec<OkexCompleteInstrument>> {
        self.take_okex()
            .map(|v| v.take_instruments().map(|instr| instr.instruments))
            .flatten()
    }

    pub fn take_okex_currencies(self) -> Option<Vec<NormalizedCurrency>> {
        self.take_okex().map(|v| v.take_currencies()).flatten()
    }
}

macro_rules! combined_rest {
    ($exchange:ident) => {
        paste::paste! {
            impl From<[<$exchange RestApiResponse>]> for CombinedRestApiResponse {
                fn from(value: [<$exchange RestApiResponse>]) -> Self {
                    Self::$exchange(value)
                }
            }
        }
    };
}

#[cfg(feature = "us")]
combined_rest!(Coinbase);

#[cfg(feature = "us")]
combined_rest!(Okex);

#[cfg(feature = "non-us")]
combined_rest!(Binance);

impl PartialEq<NormalizedRestApiDataTypes> for CombinedRestApiResponse {
    fn eq(&self, other: &NormalizedRestApiDataTypes) -> bool {
        match self {
            #[cfg(feature = "us")]
            CombinedRestApiResponse::Coinbase(vals) => vals == other,
            #[cfg(feature = "us")]
            CombinedRestApiResponse::Okex(vals) => vals == other,
            #[cfg(feature = "non-us")]
            CombinedRestApiResponse::Binance(vals) => vals == other
        }
    }
}
