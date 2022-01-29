use crate::app::Route;
use crate::TextInput;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;
use yew_router::prelude::*;
use zk_pub_models::ZettelMap;

#[function_component(ZettelSearch)]
pub fn zettel_search() -> Html {
    let current = use_state(|| String::new());
    let zettel_map = use_context::<ZettelMap>().expect("no ZettelMap found");
    let callback_current = current.clone();

    let on_change = Callback::from(move |value: String| {
        callback_current.set(value.clone());
    });

    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    let list = if *current == "" {
        vec![]
    } else {
        zettel_map
            .iter()
            .filter(|(_, zettel)| matcher.fuzzy_match(&zettel.title, &current).is_some())
            .map(|(id, zettel)| {
                let history = use_history().unwrap();
                let id = id.clone();

                let current = current.clone();
                let on_select = {
                    Callback::from(move |_| {
                        // Clear the search bar.
                        current.set("".to_string());
                        history.push(Route::Zettel { id: id.clone() })
                    })
                };

                html! {
                    <li onclick={on_select}>{format!("{}", zettel.title)}</li>
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <>
        <div class="row">
            <TextInput on_change={on_change} value={(*current).clone()}/>
        </div>
        <div class="row">
            <ul class="search">
            {list}
            </ul>
        </div>
        </>
    }
}
