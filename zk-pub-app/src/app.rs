#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::ZettelDetails;
use crate::ZettelSearch;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use zk_pub_models::Zettel;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Zettel { id: String },
}

#[derive(Properties, PartialEq)]
pub struct ZettelProps {
    pub id: String,
}

#[function_component(ZettelFor)]
pub fn zettel_for(ZettelProps { id }: &ZettelProps) -> Html {
    use_context::<Vec<Zettel>>()
        .map(|z| {
            z.iter().find(|z| &z.anchor == id).map(|zettel| {
                html! {
                    <ZettelDetails zettel={zettel.clone()} />
                }
            })
        })
        .flatten()
        .unwrap_or_else(|| html! {})
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {},
        Route::Zettel { id } => {
            html! { <ZettelFor id={id.clone()}/> }
        }
    }
}

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
            <ContextProvider<Vec<Zettel>> context={(*zettel).clone()}>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                { for details }
            </ContextProvider<Vec<Zettel>>>
        </div>
    }
}
