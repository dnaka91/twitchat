use std::rc::Rc;

use irc_proto::{Command, Message, Prefix};
use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    White,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

#[derive(Default, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub color: Color,
    #[serde(default, deserialize_with = "crate::de::string_list")]
    pub channels: Vec<String>,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
}

#[inline]
const fn default_font_size() -> f32 {
    16.0
}

#[derive(Clone, PartialEq)]
pub enum TwitchMessage {
    Privmsg(Privmsg),
    Ping(String),
}

#[derive(Clone, PartialEq)]
pub struct Privmsg {
    pub id: Rc<str>,
    pub username: String,
    pub message: String,
    pub color: Option<String>,
    pub emotes: Vec<Emote2>,
}

pub fn parse_message(message: Message) -> Option<TwitchMessage> {
    let tags = message.tags.unwrap_or_default();

    match message.command {
        Command::PRIVMSG(_, content) => {
            if content.contains(char::is_control) {
                return None;
            }

            Some(TwitchMessage::Privmsg(Privmsg {
                id: match tags.iter().find(|t| t.0 == "id").and_then(|t| t.1.clone()) {
                    Some(id) => Rc::from(id),
                    None => return None,
                },
                username: match message.prefix? {
                    Prefix::Nickname(_, username, _) => username,
                    Prefix::ServerName(_) => return None,
                },
                message: content,
                color: tags
                    .iter()
                    .find(|t| t.0 == "color")
                    .and_then(|t| t.1.clone())
                    .filter(|t| !t.is_empty()),
                emotes: tags
                    .iter()
                    .find(|t| t.0 == "emotes")
                    .and_then(|t| t.1.as_deref())
                    .map(parse_emotes)
                    .unwrap_or_default(),
            }))
        }
        Command::PING(sender, _) => Some(TwitchMessage::Ping(sender)),
        _ => None,
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Emote2 {
    pub id: u32,
    pub location: usize,
}

fn parse_emotes(tag: &str) -> Vec<Emote2> {
    let mut emotes = tag
        .split('/')
        .filter_map(|e| {
            let mut parts = e.splitn(2, ':');
            let id = parts.next()?.parse().ok()?;
            let ranges = parts.next()?;

            let emote = ranges.split(',').filter_map(move |range| {
                let mut parts = range.splitn(2, '-');
                let location = parts.next()?.parse().ok()?;
                Some(Emote2 { id, location })
            });

            Some(emote)
        })
        .flatten()
        .collect::<Vec<_>>();

    emotes.sort_by_key(|e| e.location);
    emotes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_emotes() {
        let expect = vec![Emote2 { id: 5, location: 1 }];

        assert_eq!(expect, parse_emotes("5:1-2"));

        let expect = vec![Emote2 { id: 5, location: 1 }, Emote2 { id: 5, location: 3 }];

        assert_eq!(expect, parse_emotes("5:1-2,3-4"));

        let expect = vec![
            Emote2 { id: 5, location: 1 },
            Emote2 { id: 5, location: 3 },
            Emote2 {
                id: 6,
                location: 10,
            },
        ];

        assert_eq!(expect, parse_emotes("5:1-2,3-4/6:10-15"));
    }
}
