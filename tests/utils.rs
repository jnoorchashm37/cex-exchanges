use cex_exchanges::{
    clients::ws::{MutliWsStreamBuilder, WsStream},
    exchanges::Exchange
};
use futures::StreamExt;

pub async fn stream_util<E: Exchange + Send + Unpin + 'static>(exchange: E, iterations: usize) {
    let mut stream = WsStream::new(exchange);
    stream.connect().await.unwrap();

    let mut i = 0;
    while let Some(val) = stream.next().await {
        if val.is_err() {
            println!("ERROR: {:?}", val);
        }

        assert!(val.is_ok());

        let normalized = val.clone().normalize();
        assert_eq!(val, normalized);

        if i == iterations {
            break;
        }
        i += 1;
    }
}

pub async fn mutlistream_util<E: Exchange + Send + Unpin + 'static>(builder: MutliWsStreamBuilder<E>, iterations: usize) {
    let mut stream = builder.build_multistream().await.unwrap();
    println!("CONNECTED STREAM");

    let mut i = 0;
    while let Some(val) = stream.next().await {
        if val.is_err() {
            println!("ERROR: {:?}", val);
        }

        println!("VAL");

        assert!(val.is_ok());

        let normalized = val.clone().normalize();
        assert_eq!(val, normalized);

        if i == iterations {
            break;
        }
        i += 1;
    }
}
