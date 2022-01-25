use std::{collections::VecDeque, rc::Rc};

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use futures_util::{SinkExt, StreamExt};
use irc_proto::{error::ProtocolError, Message};
use log::error;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message as WsMessage;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    models::{self, Options, Privmsg, TwitchMessage},
    widgets::{Container, Line, Status, WebSocketStatus},
};

const BUFFER_SIZE: usize = 50;

pub struct App {
    messages: VecDeque<Rc<Privmsg>>,
    status: WebSocketStatus,
    tx: Option<UnboundedSender<String>>,
    scroll: bool,
    options: Options,
}

pub enum Msg {
    Connect,
    Ready(String),
    StatusChange(WebSocketStatus),
    Scroll,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let options = gloo_utils::window()
            .location()
            .search()
            .ok()
            .as_ref()
            .map(|s| s.strip_prefix('?').unwrap_or(s))
            .and_then(|s| serde_qs::from_str(s).ok())
            .unwrap_or_default();

        Self {
            messages: VecDeque::with_capacity(BUFFER_SIZE),
            status: WebSocketStatus::Closed,
            tx: None,
            scroll: false,
            options,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                let callback = ctx.link().callback(Msg::Ready);
                let notification = ctx.link().callback(Msg::StatusChange);
                let channels = std::mem::take(&mut self.options.channels);

                let ws = WebSocket::open("wss://irc-ws.chat.twitch.tv:443").unwrap();
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
                            .send(WsMessage::Text(format!("JOIN #{}", channel)))
                            .await
                            .unwrap();
                    }

                    while let Some(msg) = rx.next().await {
                        write.send(WsMessage::Text(msg)).await.unwrap();
                    }
                });

                spawn_local(async move {
                    notification.emit(WebSocketStatus::Open);

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(WsMessage::Text(text)) => callback.emit(text),
                            Ok(WsMessage::Bytes(_)) => {}
                            Err(e) => {
                                error!("websocket error: {:?}", e);
                                notification.emit(WebSocketStatus::Error);
                                return;
                            }
                        }
                    }

                    notification.emit(WebSocketStatus::Closed);
                });

                self.tx = Some(tx);
            }
            Msg::Ready(data) => {
                data.lines()
                    .filter_map(|l| l.parse::<Message>().map(models::parse_message).transpose())
                    .for_each(|res: Result<_, ProtocolError>| match res {
                        Ok(TwitchMessage::Privmsg(pm)) => {
                            while self.messages.len() >= BUFFER_SIZE {
                                self.messages.pop_front();
                            }

                            self.messages.push_back(Rc::new(pm));
                            self.scroll = true;
                        }
                        Ok(TwitchMessage::Ping(sender)) => {
                            self.tx
                                .as_mut()
                                .unwrap()
                                .unbounded_send(format!("PONG {}", sender))
                                .unwrap();
                        }

                        Err(e) => error!("message error: {}", e),
                    });
            }
            Msg::StatusChange(status) => {
                self.status = status;
            }
            Msg::Scroll => {
                if self.scroll {
                    if let Ok(Some(element)) =
                        gloo_utils::document().query_selector(".messages .msg:last-child")
                    {
                        element.scroll_into_view();
                        self.scroll = false;
                    }
                }

                // Don't need to render anything, we just told the browser to scroll down.
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.link().send_message(Msg::Scroll);

        let lines = self.messages.iter().map(|msg| {
            html! {
                <Line message={Rc::clone(msg)} key={Rc::clone(&msg.id)} />
            }
        });

        html! {
            <>
                <Status status={self.status} />
                <Container
                    color={self.options.color}
                    font_size={self.options.font_size}
                >
                    { for lines }
                </Container>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::Connect);
        }
    }
}
