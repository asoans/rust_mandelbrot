use actix_ws::Message;
use futures_util::{
    StreamExt as _,
};

use serde_json;
use crate::mandelbrot::{MandelbrotParams, mandelbrot_generate};

pub async fn mandelbrot_ws(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
    log::info!("connected");

    let close_reason = loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        let mandelbrot_params: MandelbrotParams = serde_json::from_str(&text).unwrap();
                        let data = mandelbrot_generate(&mandelbrot_params);
                        session.binary(data).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {}

                    Message::Continuation(_) => {
                        log::warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // error or end of stream
            _ => break None,
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;

    log::info!("disconnected");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn serde_test() {
        #[derive(Debug, Deserialize)]
        struct JStruct {
            pub cx: f32,
         }
        
        let data = r#"
        {
            "cx": 1.58
        }"#;
        let jstruct: JStruct = serde_json::from_str(data).unwrap();
        println!("{:?}", jstruct);

        assert!(jstruct.cx == 1.58);
    }
}