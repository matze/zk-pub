#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::ZettelDetails;
use crate::ZettelSearch;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use zk_pub_models::ZettelMap;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Zettel { id: String },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <>
                <ZettelSearch />
                <div class="row hint">
                    <p>{"search for a note"}</p>
                </div>
                </>
            }
        }
        Route::Zettel { id } => {
            html! {
                <>
                <ZettelSearch />
                <ZettelDetails id={id.clone()}/>
                </>
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let map = use_state(|| ZettelMap::new());

    {
        let map = map.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: proper error handling
                    let fetched: ZettelMap = Request::get("/static/zettel.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    map.set(fetched);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <ContextProvider<ZettelMap> context={(*map).clone()}>
            <BrowserRouter>
                <div class="main">
                <Switch<Route> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        </ContextProvider<ZettelMap>>
    }
}
