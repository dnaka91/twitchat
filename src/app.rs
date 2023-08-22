use std::{collections::VecDeque, rc::Rc};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage};
use irc_proto::{error::ProtocolError, Message};
use leptos::{
    component, create_signal, view, IntoView, SignalSet, SignalUpdate, WriteSignal,
};
use log::error;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

use crate::{
    models::{self, Options, Privmsg, TwitchMessage},
    widgets::{Container, Status, WebSocketStatus},
};

const BUFFER_SIZE: usize = 50;

#[component]
pub fn app() -> impl IntoView {
    let options = gloo_utils::window()
        .location()
        .search()
        .ok()
        .as_ref()
        .map(|s| s.strip_prefix('?').unwrap_or(s))
        .and_then(|s| serde_qs::from_str::<Options>(s).ok())
        .unwrap_or_default();

    let (status, set_status) = create_signal( WebSocketStatus::Closed);
    let (messages, set_messages) = create_signal(VecDeque::with_capacity(BUFFER_SIZE));

    connect(options.channels, set_status, set_messages);

    view! {
        <Status status={status.into()} />
        <Container
            messages={messages.into()}
            color={options.color}
            font_size={options.font_size}
        />
    }
}

fn connect(
    channels: Vec<String>,
    status: WriteSignal<WebSocketStatus>,
    messages: WriteSignal<VecDeque<Rc<Privmsg>>>,
) {
    let ws = WebSocket::open("wss://irc-ws.chat.twitch.tv:443").unwrap_throw();
    let (mut write, mut read) = ws.split();
    let (tx, mut rx) = futures_channel::mpsc::unbounded();

    spawn_local(async move {
        write
            .send(WsMessage::Text("NICK justinfan12345".to_owned()))
            .await
            .unwrap();
        write
            .send(WsMessage::Text("CAP REQ :twitch.tv/tags".to_owned()))
            .await
            .unwrap();

        for channel in channels {
            write
                .send(WsMessage::Text(format!("JOIN #{channel}")))
                .await
                .unwrap();
        }

        while let Some(msg) = rx.next().await {
            write.send(WsMessage::Text(msg)).await.unwrap();
        }
    });

    spawn_local(async move {
        status.set(WebSocketStatus::Open);

        while let Some(msg) = read.next().await {
            match msg {
                Ok(WsMessage::Text(text)) => {
                    text.lines()
                        .filter_map(|l| l.parse::<Message>().map(models::parse_message).transpose())
                        .for_each(|res: Result<_, ProtocolError>| match res {
                            Ok(TwitchMessage::Privmsg(pm)) => {
                                messages.update(|messages| {
                                    while messages.len() >= BUFFER_SIZE {
                                        messages.pop_front();
                                    }

                                    messages.push_back(Rc::new(pm));
                                    log::info!("msg size is {}", messages.len());
                                });

                                if let Ok(Some(element)) = gloo_utils::document()
                                    .query_selector(".messages .msg:last-child")
                                {
                                    element.scroll_into_view();
                                }
                            }
                            Ok(TwitchMessage::Ping(sender)) => {
                                tx.unbounded_send(format!("PONG {sender}")).unwrap();
                            }

                            Err(e) => error!("message error: {e}"),
                        });
                }
                Ok(WsMessage::Bytes(_)) => {}
                Err(e) => {
                    error!("websocket error: {:?}", e);
                    status.set(WebSocketStatus::Error);
                    return;
                }
            }
        }

        status.set(WebSocketStatus::Closed);
    });
}
