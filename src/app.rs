use leptos::*;
use wasm_bindgen::prelude::*;

use super::components::auth::{Login, Register};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Login,
    Register,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_view, set_current_view) = create_signal(AppView::Login);
    let (is_authenticated, set_is_authenticated) = create_signal(false);

    let on_auth_success = move |_user_id: i32| {
        set_is_authenticated.set(true);
    };

    view! {
        <div class="min-h-screen bg-gray-100 flex items-center justify-center">
        {move || if is_authenticated.get() {
            view! {
                <h1>"Hello App!"</h1>
            }.into_view()
        } else {
            match current_view.get() {
                AppView::Login => view! {
                    <Login
                        on_success=on_auth_success
                        on_register=move |_| set_current_view.set(AppView::Register)
                    />
                },
                AppView::Register => view! {
                    <Register
                        on_success=move |_| set_current_view.set(AppView::Login)
                        on_back=move |_| set_current_view.set(AppView::Login)
                    />
                }
            }.into_view()
        }}
        </div>
    }
}
