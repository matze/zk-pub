use crate::RawHtml;
use yew::prelude::*;
use zk_pub_models::Zettel;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub zettel: Zettel,
}

#[function_component(ZettelDetails)]
pub fn zettel_details(Props { zettel }: &Props) -> Html {
    let z = zettel.clone();

    html! {
        <div class="row zettel">
            <RawHtml inner_html={z.inner_html}/>
        </div>
    }
}
