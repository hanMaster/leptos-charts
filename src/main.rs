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
            <h1>{"Котировки криптовалют"}</h1>
            <div class="symbols-wrapper">
                <button
                    on:click=move |_| *set_symbol.write() = "BTCUSDT"
                    class:selected=move || *symbol.read() == "BTCUSDT"
                >
                    BTC/USDT
                </button>
                <button
                    on:click=move |_| *set_symbol.write() = "ETHUSDT"
                    class:selected=move || *symbol.read() == "ETHUSDT"
                >
                    ETH/USDT
                </button>
                <button
                    on:click=move |_| *set_symbol.write() = "SOLUSDT"
                    class:selected=move || *symbol.read() == "SOLUSDT"
                >
                    SOL/USDT
                </button>
                <button
                    on:click=move |_| *set_symbol.write() = "TRUMPUSDT"
                    class:selected=move || *symbol.read() == "TRUMPUSDT"
                >
                    TRUMP/USDT
                </button>
            </div>
            <div class="symbols-wrapper">
                <button
                    on:click=move |_| *set_interval.write() = "1"
                    class:selected=move || *interval.read() == "1"
                >
                    1m
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "5"
                    class:selected=move || *interval.read() == "5"
                >
                    5m
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "15"
                    class:selected=move || *interval.read() == "15"
                >
                    15m
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "30"
                    class:selected=move || *interval.read() == "30"
                >
                    30m
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "60"
                    class:selected=move || *interval.read() == "60"
                >
                    1h
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "240"
                    class:selected=move || *interval.read() == "240"
                >
                    4h
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "D"
                    class:selected=move || *interval.read() == "D"
                >
                    Day
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "W"
                    class:selected=move || *interval.read() == "W"
                >
                    Week
                </button>
                <button
                    on:click=move |_| *set_interval.write() = "M"
                    class:selected=move || *interval.read() == "M"
                >
                    Month
                </button>
            </div>
            <div class="chart" inner_html=async_result />
        </div>
    }
}
