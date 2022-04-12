use yew::prelude::*;

use crate::models::Color;

pub struct Container {
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: String::new(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.style = format!("font-size:{:.1}px;", ctx.props().font_size);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let color_class = match ctx.props().color {
            Color::White => "color-white",
            Color::Black => "color-black",
        };

        html! {
            <div class={classes!("messages", color_class)} style={self.style.clone()}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
