#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::ZettelDetails;
use crate::ZettelSearch;
use reqwasm::http::Request;
use yew::prelude::*;
use zk_pub_models::Zettel;

#[function_component(App)]
pub fn app() -> Html {
    let zettel = use_state(|| vec![]);

    {
        let zettel = zettel.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched: Vec<Zettel> = Request::get("/static/zettel.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    zettel.set(fetched);
                });
                || ()
            },
            (),
        );
    }

    let selected_zettel = use_state(|| None);

    let on_zettel_select = {
        let selected_zettel = selected_zettel.clone();

        Callback::from(move |zettel: Zettel| selected_zettel.set(Some(zettel)))
    };

    let details = selected_zettel.as_ref().map(|z| {
        html! {
            <ZettelDetails zettel={z.clone()} />
        }
    });

    html! {
        <div class="main">
            <ZettelSearch zettel={(*zettel).clone()} on_click={on_zettel_select.clone()}/>
            { for details }
        </div>
    }
}
