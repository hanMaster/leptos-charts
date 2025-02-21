use leptos_charts::chart::gen_candlestick_svg;
use leptos::prelude::*;
use leptos_charts::error::Error;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

async fn get_chart(symbol: &str, interval: &str) -> String {
    gen_candlestick_svg(symbol, interval)
        .await
        .unwrap_or_else(|e| match e {
            Error::Request(msg) => msg,
            _ => "Failed to get Data".to_string(),
        })
}

#[component]
fn App() -> impl IntoView {
    let (symbol, set_symbol) = signal("ETHUSDT");
    let (interval, set_interval) = signal("240");

    let chart_resource = LocalResource::new(move || get_chart(*symbol.read(), *interval.read()));

    let async_result = move || {
        chart_resource
            .get()
            .as_deref()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "Loading...".into())
    };

    view! {
        <div class="app">
            <h1> { "Котировки криптовалют" }</h1>
            <div class="symbols-wrapper">
                <button on:click = move |_| *set_symbol.write() = "BTCUSDT" >BTC/USDT</button>
                <button on:click = move |_| *set_symbol.write() = "ETHUSDT" >ETH/USDT</button>
                <button on:click = move |_| *set_symbol.write() = "SOLUSDT" >SOL/USDT</button>
                <button on:click = move |_| *set_symbol.write() = "TRUMPUSDT" >TRUMP/USDT</button>
            </div>
            <div class="symbols-wrapper">
                <button on:click = move |_| *set_interval.write() = "1" >1m</button>
                <button on:click = move |_| *set_interval.write() = "5" >5m</button>
                <button on:click = move |_| *set_interval.write() = "15" >15m</button>
                <button on:click = move |_| *set_interval.write() = "30" >30m</button>
                <button on:click = move |_| *set_interval.write() = "60" >1h</button>
                <button on:click = move |_| *set_interval.write() = "240" >4h</button>
                <button on:click = move |_| *set_interval.write() = "D" >Day</button>
                <button on:click = move |_| *set_interval.write() = "W" >Week</button>
                <button on:click = move |_| *set_interval.write() = "M" >Month</button>
            </div>
            <div inner_html = async_result />
        </div>
    }
}
