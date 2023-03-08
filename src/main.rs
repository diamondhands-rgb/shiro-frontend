use material_yew::MatIcon;
use yew::prelude::*;
use yew_router::prelude::*;

use self::components::balance_page::Page as BalancePage;
use self::components::bitcoin_page::Page as BitcoinPage;
use self::components::issue_asset_page::Page as IssueAssetPage;
use self::components::mnemonic_page::Page as MnemonicPage;
use self::components::utxos_page::Page as UtxosPage;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    BalancePageRoute,
    #[at("/bitcoin")]
    BitcoinPageRoute,
    #[at("/issue")]
    IssueAssetPageRoute,
    #[at("/mnemonic")]
    MnemonicPageRoute,
    #[at("/utxos")]
    UtxosPageRoute,
}

#[function_component(BalancePageRoute)]
fn balance_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <BalancePage/>
        </>
    }
}

#[function_component(BitcoinPageRoute)]
fn bitcoin_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <BitcoinPage/>
        </>
    }
}

#[function_component(MnemonicPageRoute)]
fn mnemonic_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <MnemonicPage/>
        </>
    }
}

#[function_component(IssueAssetPageRoute)]
fn issue_asset_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <IssueAssetPage/>
        </>
    }
}

#[function_component(UtxosPageRoute)]
fn utxos_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <UtxosPage/>
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::BalancePageRoute => html!( <BalancePageRoute/> ),
        Route::BitcoinPageRoute => html!( <BitcoinPageRoute/> ),
        Route::IssueAssetPageRoute => html!( <IssueAssetPageRoute/> ),
        Route::MnemonicPageRoute => html!( <MnemonicPageRoute/> ),
        Route::UtxosPageRoute => html!( <UtxosPageRoute/> ),
    }
}

fn inject_navbar() -> Html {
    html! {
                <nav class="navbar navbar-expand-lg">
                    <div class="container">
                        <a class="navbar-brand" href="javascript:void(0)">{"Shiro-wallet"}</a>
                        <button class="navbar-toggler ms-auto" type="button" data-bs-toggle="collapse" data-bs-target="#n_bar" aria-controls="navbarNavAltMarkup" aria-label="Toggle navigation">
                            <MatIcon>{"menu"}</MatIcon>
                        </button>
                        <div class="collapse navbar-collapse" id="n_bar">
                            <ul class="navbar-nav active">
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::MnemonicPageRoute}>{"Mnemonic"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::BalancePageRoute}>{"Balance"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::BitcoinPageRoute}>{"Bitcoin"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::IssueAssetPageRoute}>{"Issue"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::UtxosPageRoute}>{"UTXOs"}</Link<Route>></li>
                            </ul>
                        </div>
                    </div>
                </nav>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <div>
            <div class="navbar navbar-default">
                <a class="navbar-brand" href="/">{"Shiro-wallet"}</a>
                <a class="navbar-link" href="bitcoin">{"bitcoin"}</a>
                <a class="navbar-link" href="issue">{"issue"}</a>
                <a class="navbar-link" href="mnemonic">{"mnemonic"}</a>
                <a cless="navbar-link" href="utxos">{"utxos"}</a>
            </div>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
    wasm_logger::init(wasm_logger::Config::default());
}
