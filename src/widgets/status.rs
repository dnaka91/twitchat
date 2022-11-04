use yew::prelude::*;

pub struct Status;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub status: WebSocketStatus,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WebSocketStatus {
    Open,
    Closed,
    Error,
}

impl Component for Status {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let status_color = match ctx.props().status {
            WebSocketStatus::Open => "is-success",
            WebSocketStatus::Closed => "is-warning",
            WebSocketStatus::Error => "is-danger",
        };

        html! {
            <div class={classes!("status", status_color)}></div>
        }
    }
}
