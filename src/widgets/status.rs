use leptos::{component, view, IntoView, Signal, SignalGet};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WebSocketStatus {
    Open,
    Closed,
    Error,
}

#[component]
pub fn status(status: Signal<WebSocketStatus>) -> impl IntoView {
    view! {
        <div
            class="status"
            class:is-success={move || status.get() == WebSocketStatus::Open}
            class:is-warning={move || status.get() == WebSocketStatus::Closed}
            class:is-danger={move || status.get() == WebSocketStatus::Error}
        >
        </div>
    }
}
