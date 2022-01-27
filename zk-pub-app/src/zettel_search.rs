use crate::TextInput;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;
use zk_pub_models::Zettel;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub zettel: Vec<Zettel>,
    pub on_click: Callback<Zettel>,
}

pub enum Msg {
    SetSearchTerm(String),
}

#[derive(Default)]
pub struct ZettelSearch {
    search_term: String,
}

impl Component for ZettelSearch {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::SetSearchTerm(term) => self.search_term = term,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click = ctx.props().on_click.clone();
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        let list = if self.search_term.is_empty() {
            vec![]
        } else {
            ctx.props()
                .zettel
                .iter()
                .filter(|z| matcher.fuzzy_match(&z.title, &self.search_term).is_some())
                .map(|z| {
                    let on_select = {
                        let on_click = on_click.clone();
                        let z = z.clone();
                        Callback::from(move |_| on_click.emit(z.clone()))
                    };

                    html! {
                        <li onclick={on_select}>{format!("{}", z.title)}</li>
                    }
                })
                .collect::<Vec<_>>()
        };

        let hint = if self.search_term.is_empty() {
            html! {
                <div class="row hint">
                    <p>{"↑"}</p>
                    <p>{"Search for a note"}</p>
                </div>
            }
        } else {
            html! {}
        };

        let on_change = ctx.link().callback(Self::Message::SetSearchTerm);

        html! {
            <>
            <div class="row">
                <TextInput on_change={on_change} value={self.search_term.clone()} />
            </div>
            {hint}
            <div class="row">
                <ul class="search">
                    {list}
                </ul>
            </div>
            </>
        }
    }
}
