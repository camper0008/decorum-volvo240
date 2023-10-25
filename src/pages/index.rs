use yew::prelude::*;

#[function_component(IndexPage)]
pub fn index() -> Html {
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
                        <a href="#">{"test-titel"}</a>
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
                        <a href="#">{"test-titel"}</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
