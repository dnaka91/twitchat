use std::rc::Rc;

use leptos::{component, create_memo, view, For, IntoView, SignalGet};

use crate::models::Privmsg;

#[component]
pub fn line(message: Rc<Privmsg>) -> impl IntoView {
    let msg = Rc::clone(&message);
    let parts = create_memo(move |_| {
        let mut pos = (0, 0);

        let parts = msg
            .emotes
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let msg = Rc::clone(&msg);
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

                let msg = msg
                    .message
                    .get(pos.1..start_char)
                    .unwrap_or(&msg.message)
                    .to_owned();

                pos = (end_index, end_char);

                (i, msg, srcset)
            })
            .collect::<Vec<_>>();

        let tail = msg.message[pos.1..].to_owned();

        (parts, tail)
    });

    let msg = Rc::clone(&message);

    view! {
        <div class="msg">
            <span style:color=msg.color.clone()>{message.username.clone()}</span>
            <span>{":"}</span>
            <For
                each={move || parts.get().0}
                key={move |part| part.0}
                children={move |(_, msg, srcset)| {
                    view! {
                        <span>{msg}</span>
                        <img srcset={srcset} />
                    }
                }}
            />
            <span>{move || parts.get().1}</span>
        </div>
    }
}
