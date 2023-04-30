use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, EthereumContextProvider, ConnectButton,
};


#[function_component]
pub fn Web3Login() -> Html {

    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton>
                    <button>{"Connect"}</button>
                </ConnectButton>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}
