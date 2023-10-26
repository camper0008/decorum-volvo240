use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="index">
            <div class="list">
                <h3 class="list-title">{"Seneste indlæg"}</h3>
                <div class="list-item list-header">
                    <div class="list-item-time">{"Dato"}</div>
                    <div class="list-item-category">
                        {"Kategori"}
                    </div>
                    <div class="list-item-title">
                        {"Overskrift"}
                    </div>
                </div>

                <div class="list-item">
                    <time class="list-item-time">{"d. 20/07-2004"}</time>
                    <div class="list-item-category">
                        {"Kategori"}
                    </div>
                    <div class="list-item-title">
                        <Link<Route> to={Route::NotFound}>{ "Test Titel" }</Link<Route>>
                    </div>
                </div>
            </div>
            <img src="/static/volvo240.jpg" />
            <div class="list">
                <h3 class="list-title">{"Seneste Tips & Tricks"}</h3>
                <div class="list-item list-header">
                    <div class="list-item-time">{"Dato"}</div>
                    <div class="list-item-title">
                        {"Overskrift"}
                    </div>
                </div>

                <div class="list-item">
                    <time class="list-item-time">{"d. 20/07-2004"}</time>
                    <div class="list-item-title">
                        <Link<Route> to={Route::NotFound}>{ "Test Titel" }</Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}
