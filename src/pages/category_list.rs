use crate::{models::Category, route::Route};
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct AllCategoriesResponse {
    ok: bool,
    data: Vec<Category>,
}

#[function_component(Categories)]
fn categories() -> Html {
    let categories = use_state(|| Vec::new());

    {
        let categories = categories.clone();
        use_effect_with((), move |_| {
            let categories = categories.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: AllCategoriesResponse = Request::get("/api/posts/all_categories")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                categories.set(response.data);
            });
            || ()
        });
    };

    html! {
        categories
            .iter()
            .map(|category| {
                let category = category.clone();
                html!{
                    <div class="list-item">
                        <div class="list-item-title">
                            <Link<Route> to={Route::PostList { category_title: category.title.clone(), category_id: category.id }}>{ category.title.clone() }</Link<Route>>
                        </div>
                    </div>
                }
            })
            .collect::<Html>()
    }
}

#[function_component(CategoryListPage)]
pub fn forum_list() -> Html {
    html! {
        <div class="forum_list">
            <div class="list">
                <div class="list-item list-header">
                    <div class="list-item-title">
                        {"Kategorier"}
                    </div>
                </div>

                <Categories />
            </div>
        </div>
    }
}
