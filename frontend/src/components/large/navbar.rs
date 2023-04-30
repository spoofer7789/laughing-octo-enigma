use yew::prelude::*;
use yew_router::{prelude::use_navigator, navigator};
use crate::routes::{*, router::MainRoute, search::SearchRoute};

//some type of switch statement, that will change when the user is loggedin.
#[function_component]
pub fn Navbar() -> Html {
    let navigator = use_navigator().unwrap();
         let home = {
         let navigator = navigator.clone();
         let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html! {
         <button{onclick}>{"Home"}</button>
         }
        };
    // let search = {
    //         let navigator = use_navigator().clone();
    //         let onenter = Callback::from(move |_| navigator.push(&SearchRoute::Search));
    //      html! {   
    //             <input type="text"/>
    //           }
    //         };
    let account = {
        let navigator = use_navigator().clone();

        html! {

        }
    };
    //create a dropdown for account
    html! {
        <div id="navigation">
         {home}
        //  {search}       
         
        </div>
    }
}