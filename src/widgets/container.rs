use std::{collections::VecDeque, rc::Rc};

use leptos::{component, view, For, IntoView, Signal, SignalGet};

use crate::{
    models::{Color, Privmsg},
    widgets::Line,
};

#[component]
pub fn container(
    #[prop(into)] messages: Signal<VecDeque<Rc<Privmsg>>>,
    color: Color,
    font_size: f32,
) -> impl IntoView {
    view! {
        <div
            class="messages"
            class:color-white=color == Color::White
            class:color-black=color == Color::Black
            style:font-size=move || format!("{font_size:.1}px")
        >
            <For
                each=move || messages.get()
                key=move |msg| Rc::clone(&msg.id)
                children=move |msg| view! { <Line message=msg/> }
            />
        </div>
    }
}
