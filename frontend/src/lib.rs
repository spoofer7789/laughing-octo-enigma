mod components;
mod routes;
mod views;
use yew::prelude::*;
use routes::router::Browserfunc;
#[function_component]
pub fn App() -> Html {
    html! {
        <>
<Browserfunc/>
        </>
    }
}