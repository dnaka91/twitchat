use yew::prelude::*;
use yewtil::NeqAssign;

use crate::models::Color;

pub struct Container {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub color: Color,
}

impl Component for Container {
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
        let color_class = match self.props.color {
            Color::White => "color-white",
            Color::Black => "color-black",
        };

        html! {
            <div class=("messages", color_class)>
                { for self.props.children.iter() }
            </div>
        }
    }
}
