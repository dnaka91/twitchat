use std::rc::Rc;

use yew::prelude::*;

use crate::models::Privmsg;

pub struct Line;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub message: Rc<Privmsg>,
}

impl Component for Line {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg = &ctx.props().message;
        let mut pos = (0, 0);

        let parts = msg.emotes.iter().map(|e| {
            let id = e.id;
            let srcset = format!(
                "\
                https://static-cdn.jtvnw.net/emoticons/v1/{id}/1.0 1x,\
                https://static-cdn.jtvnw.net/emoticons/v1/{id}/2.0 2x,\
                https://static-cdn.jtvnw.net/emoticons/v1/{id}/3.0 4x\
                ",
            );

            // BetterTTV
            // List comes from https://api.betterttv.net/2/channels/{channel_id}
            //
            // response:
            // {
            //   "emotes": [
            //     {
            //       "id": "55fb4827b4ccd58c2f985c03",
            //       "channel": "crream",
            //       "code": "FBYeti",
            //       "imageType": "png"
            //     },
            //   ]
            // }
            //
            //
            // srcset:
            // https://cdn.betterttv.net/emote/59f27b3f4ebd8047f54dee29/1x 1x,
            // https://cdn.betterttv.net/emote/59f27b3f4ebd8047f54dee29/2x 2x,
            // https://cdn.betterttv.net/emote/59f27b3f4ebd8047f54dee29/3x 4x

            let (start_index, start_char) = msg
                .message
                .char_indices()
                .enumerate()
                .skip(pos.0)
                .take_while(|(i, _)| *i <= e.location)
                .map(|(i, (ci, _))| (i, ci))
                .last()
                .unwrap_or_default();
            let (end_index, end_char) = msg
                .message
                .char_indices()
                .enumerate()
                .skip(start_index)
                .take_while(|(_, (_, c))| !c.is_whitespace())
                .map(|(i, (ci, _))| (i + 1, ci + 1))
                .last()
                .unwrap_or_default();

            let msg = msg.message.get(pos.1..start_char).unwrap_or(&msg.message);

            let h = html! {
                <>
                <span>{msg}</span>
                <img srcset={srcset} />
                </>
            };

            pos = (end_index, end_char);

            h
        });

        let color_style = msg
            .color
            .as_deref()
            .map(|c| format!("color:{};", c))
            .unwrap_or_default();

        html! {
            <div class="msg">
                <span style={color_style}>{&msg.username}</span>
                <span>{":"}</span>
                {for parts}
                <span>{&msg.message[pos.1..]}</span>
            </div>
        }
    }
}
