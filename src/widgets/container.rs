use yew::prelude::*;
use yewtil::NeqAssign;

use crate::models::Color;

pub struct Container {
    props: Props,
    style: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub color: Color,
    pub font_size: f32,
}

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            style: String::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.style = format!("font-size:{:.1}px;", self.props.font_size);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let color_class = match self.props.color {
            Color::White => "color-white",
            Color::Black => "color-black",
        };

        html! {
            <div class=classes!("messages", color_class) style=self.style.clone()>
                { for self.props.children.iter() }
            </div>
        }
    }
}
