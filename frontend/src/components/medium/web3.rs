
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_ethereum_provider::UseEthereumHandle::{display_address, chain_id_hex};
use yew_ethereum_provider::{ 
    chain::{self, ethereum}, AccountLabel, EthereumContextProvider, ConnectButton, 
};

#[function_component]
pub fn Web3Login() -> Html {
   
let realaddress = ethereum.display_address();
    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton>
                    <button>{"Connect"}</button> // when the user connects 
                    //their ethereum account it will redirect them to choose a username.
                </ConnectButton>
                <AccountLabel />
            </EthereumContextProvider>
            {realaddress}
        </div>
    }
}
//publickey
//signature