use std::{collections::VecDeque, rc::Rc};

use anyhow::Result;
use irc_proto::{error::ProtocolError, Message};
use log::error;
use yew::{
    prelude::*,
    services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask},
};

use crate::{
    models::{self, Options, Privmsg, TwitchMessage},
    widgets::{Container, Line, Status},
};

const BUFFER_SIZE: usize = 50;

pub struct App {
    link: ComponentLink<Self>,
    task: Option<WebSocketTask>,
    messages: VecDeque<Privmsg>,
    status: WebSocketStatus,
    scroll: bool,
    options: Options,
}

pub enum Msg {
    Connect,
    Ready(Result<String>),
    StatusChange(WebSocketStatus),
    Scroll,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let options = yew::utils::window()
            .location()
            .search()
            .ok()
            .as_ref()
            .map(|s| s.strip_prefix('?').unwrap_or(s))
            .and_then(|s| serde_qs::from_str(s).ok())
            .unwrap_or_default();

        link.send_message(Msg::Connect);

        Self {
            link,
            task: None,
            messages: VecDeque::with_capacity(BUFFER_SIZE),
            status: WebSocketStatus::Closed,
            scroll: false,
            options,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
                let callback = self.link.callback(Msg::Ready);
                let notification = self.link.callback(Msg::StatusChange);

                let task = WebSocketService::connect_text(
                    "wss://irc-ws.chat.twitch.tv:443",
                    callback,
                    notification,
                )
                .unwrap();

                self.task = Some(task);
            }
            Msg::Ready(data) => {
                if let Ok(data) = data {
                    data.lines()
                        .filter_map(|l| l.parse::<Message>().map(models::parse_message).transpose())
                        .for_each(|res: Result<_, ProtocolError>| match res {
                            Ok(TwitchMessage::Privmsg(pm)) => {
                                while self.messages.len() >= BUFFER_SIZE {
                                    self.messages.pop_front();
                                }

                                self.messages.push_back(pm);
                                self.scroll = true;
                            }
                            Ok(TwitchMessage::Ping(sender)) => self
                                .task
                                .as_mut()
                                .unwrap()
                                .send(Ok(format!("PONG {}", sender))),

                            Err(e) => error!("message error: {}", e),
                        });
                }
            }
            Msg::StatusChange(status) => {
                if status == WebSocketStatus::Opened {
                    if let Some(task) = &mut self.task {
                        task.send(Ok("NICK justinfan12345".to_owned()));
                        task.send(Ok("CAP REQ :twitch.tv/tags".to_owned()));
                        for channel in &self.options.channels {
                            task.send(Ok(format!("JOIN #{}", channel)));
                        }
                    }
                }
                self.status = status;
            }
            Msg::Scroll => {
                if self.scroll {
                    if let Ok(Some(element)) =
                        yew::utils::document().query_selector(".messages .msg:last-child")
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.link.send_message(Msg::Scroll);

        let lines = self.messages.iter().map(|msg| {
            html! {
                <Line message=msg key=Rc::clone(&msg.id) />
            }
        });

        html! {
            <div>
                <Status status=self.status.clone() />
                <Container color=self.options.color>
                    { for lines }
                </Container>
            </div>
        }
    }
}
