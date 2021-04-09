use yew::{prelude::*, services::websocket::WebSocketStatus};
use yewtil::NeqAssign;

pub struct Status {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: WebSocketStatus,
}

impl Component for Status {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let status_color = match self.props.status {
            WebSocketStatus::Closed => "is-warning",
            WebSocketStatus::Opened => "is-success",
            WebSocketStatus::Error => "is-danger",
        };

        html! {
            <div class=("status", status_color)></div>
        }
    }
}
