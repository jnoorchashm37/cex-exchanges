use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    exchanges::normalized::types::NormalizedInstrument,
    kucoin::KucoinTradingPair,
    normalized::{rest_api::NormalizedRestApiDataTypes, types::NormalizedTradingType},
    CexExchange
};

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct KucoinAllSymbols {
    pub symbols: Vec<KucoinSymbol>
}
impl KucoinAllSymbols {
    pub fn normalize(self) -> Vec<NormalizedInstrument> {
        self.symbols
            .into_iter()
            .flat_map(KucoinSymbol::normalize)
            .collect()
    }
}

impl PartialEq<NormalizedRestApiDataTypes> for KucoinAllSymbols {
    fn eq(&self, other: &NormalizedRestApiDataTypes) -> bool {
        match other {
            NormalizedRestApiDataTypes::AllInstruments(other_instrs) => {
                let this_symbols = self
                    .symbols
                    .iter()
                    .map(|instr| (instr.base_currency.clone(), instr.quote_currency.clone(), instr.symbol.normalize()))
                    .collect::<HashSet<_>>();

                let others_symbols = other_instrs
                    .iter()
                    .map(|instr| (instr.base_asset_symbol.clone(), instr.quote_asset_symbol.clone(), instr.trading_pair.clone()))
                    .collect::<HashSet<_>>();

                others_symbols
                    .into_iter()
                    .all(|instr| this_symbols.contains(&instr))
            }
            _ => false
        }
    }
}

impl<'de> Deserialize<'de> for KucoinAllSymbols {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let val = Value::deserialize(deserializer)?;

        let symbols_value = val
            .get("symbols")
            .ok_or(eyre::ErrReport::msg("could not find 'symbols' field in kucoin symbols response".to_string()))
            .map_err(serde::de::Error::custom)?;

        let symbols = serde_json::from_value(symbols_value.clone()).map_err(serde::de::Error::custom)?;

        Ok(KucoinAllSymbols { symbols })
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct KucoinSymbol {
    pub symbol:            KucoinTradingPair,
    pub name:              String,
    #[serde(rename = "baseCurrency")]
    pub base_currency:     String,
    #[serde(rename = "quoteCurrency")]
    pub quote_currency:    String,
    #[serde(rename = "feeCurrency")]
    pub fee_currency:      String,
    #[serde(rename = "market")]
    pub market:            String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "baseMinSize")]
    pub base_min_size:     f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "quoteMinSize")]
    pub quote_min_size:    f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "baseMaxSize")]
    pub base_max_size:     f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "quoteMaxSize")]
    pub quote_max_size:    f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "baseIncrement")]
    pub base_increment:    f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "quoteIncrement")]
    pub quote_increment:   f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "priceIncrement")]
    pub price_increment:   f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "priceLimitRate")]
    pub price_limit_rate:  f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "minFunds")]
    pub min_funds:         f64,
    #[serde(rename = "isMarginEnabled")]
    pub is_margin_enabled: bool,
    #[serde(rename = "enableTrading")]
    pub enable_trading:    bool
}

impl KucoinSymbol {
    pub fn normalize(self) -> Vec<NormalizedInstrument> {
        let mut vals = vec![NormalizedInstrument {
            exchange:              CexExchange::Binance,
            trading_pair:          self.symbol.normalize(),
            trading_type:          NormalizedTradingType::Spot,
            base_asset_symbol:     self.base_currency.clone(),
            quote_asset_symbol:    self.quote_currency.clone(),
            active:                self.enable_trading,
            exchange_ranking:      0 as i64,
            exchange_ranking_kind: "".to_string(),
            futures_expiry:        None
        }];

        if self.is_margin_enabled {
            vals.push(NormalizedInstrument {
                exchange:              CexExchange::Binance,
                trading_pair:          self.symbol.normalize(),
                trading_type:          NormalizedTradingType::Margin,
                base_asset_symbol:     self.base_currency.clone(),
                quote_asset_symbol:    self.quote_currency.clone(),
                active:                self.enable_trading,
                exchange_ranking:      0 as i64,
                exchange_ranking_kind: "".to_string(),
                futures_expiry:        None
            });
        }

        vals
    }
}

impl PartialEq<NormalizedInstrument> for KucoinSymbol {
    fn eq(&self, other: &NormalizedInstrument) -> bool {
        let equals = other.exchange == CexExchange::Binance
            && other.trading_pair == self.symbol.normalize()
            && other.trading_type == NormalizedTradingType::Spot
            && other.base_asset_symbol == *self.base_currency
            && other.quote_asset_symbol == *self.quote_currency
            && other.active == self.enable_trading
            && other.exchange_ranking == 0 as i64
            && other.exchange_ranking_kind == "".to_string()
            && other.futures_expiry == None;

        if !equals {
            println!("SELF: {:?}", self);
            println!("NORMALIZED: {:?}", other);
        }

        equals
    }
}
