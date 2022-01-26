use web_sys::Element;
use yew::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

/// Component to mount raw HTML passed as string property `inner_html`.
pub struct RawHtml {
    node_ref: NodeRef,
}

impl Component for RawHtml {
    type Message = ();

    type Properties = RawHtmlProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()}/>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _render: bool) {
        self.node_ref
            .cast::<Element>()
            .unwrap()
            .set_inner_html(&ctx.props().inner_html);
    }
}
