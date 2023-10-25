use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <div class={classes!("not_found")}>
            <h1 class={"not_found"}>{"Side ikke fundet"}</h1>
        </div>
    }
}
