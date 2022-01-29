use crate::RawHtml;
use yew::prelude::*;
use zk_pub_models::ZettelMap;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(ZettelDetails)]
pub fn zettel_details(Props { id }: &Props) -> Html {
    use_context::<ZettelMap>()
        .map(|m| {
            m.get(id).map(|zettel| {
                let inner_html = zettel.inner_html.clone();

                html! {
                    <div class="row zettel">
                        <RawHtml inner_html={inner_html}/>
                    </div>
                }
            })
        })
        .flatten()
        .unwrap_or_else(|| {
            html! {
                {"not found"}
            }
        })
}
