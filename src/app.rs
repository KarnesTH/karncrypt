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
    Dashboard,
}

#[derive(Clone, Copy, PartialEq)]
enum DashboardTab {
    Passwords,
    Generator,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_view, set_current_view) = create_signal(AppView::Login);
    let (is_authenticated, set_is_authenticated) = create_signal(false);
    let (current_tab, set_current_tab) = create_signal(DashboardTab::Passwords);
    let (_user_id, set_user_id) = create_signal(None::<i32>);

    let on_auth_success = move |id: i32| {
        set_user_id.set(Some(id));
        set_is_authenticated.set(true);
        set_current_view.set(AppView::Dashboard);
    };

    let on_logout = move |_| {
        set_is_authenticated.set(false);
        set_user_id.set(None);
        set_current_view.set(AppView::Login);
    };

    view! {
        <div class="min-h-screen bg-gray-100">
        {move || if is_authenticated.get() {
            view! {
                <nav class="bg-white shadow-lg">
                    <div class="max-w-7xl mx-auto px-4">
                        <div class="flex justify-between h-16">
                            <div class="flex">
                                <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                                    <button
                                        class=move || format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
                                            if current_tab.get() == DashboardTab::Passwords {
                                                "border-indigo-500 text-gray-900"
                                            } else {
                                                "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
                                            }
                                        )
                                        on:click=move |_| set_current_tab.set(DashboardTab::Passwords)
                                    >
                                        "Passwords"
                                    </button>
                                    <button
                                        class=move || format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
                                            if current_tab.get() == DashboardTab::Generator {
                                                "border-indigo-500 text-gray-900"
                                            } else {
                                                "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
                                            }
                                        )
                                        on:click=move |_| set_current_tab.set(DashboardTab::Generator)
                                    >
                                        "Generator"
                                    </button>
                                </div>
                            </div>
                            <div class="flex items-center">
                                <button
                                    class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                                    on:click=on_logout
                                >
                                    "Logout"
                                </button>
                            </div>
                        </div>
                    </div>
                </nav>
                <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    {move || match current_tab.get() {
                        DashboardTab::Passwords => view! {
                            <div class="bg-white shadow-sm rounded-lg p-6">
                                <h2 class="text-lg font-semibold mb-4">"Your Passwords"</h2>
                                // Hier kommt die Passwort-Tabelle
                            </div>
                        },
                        DashboardTab::Generator => view! {
                            <div class="bg-white shadow-sm rounded-lg p-6">
                                <h2 class="text-lg font-semibold mb-4">"Password Generator"</h2>
                                // Hier kommt der Password Generator
                            </div>
                        }
                    }}
                </main>
            }.into_view()
        } else {
            match current_view.get() {
                AppView::Login => view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Login
                            on_success=on_auth_success
                            on_register=move |_| set_current_view.set(AppView::Register)
                        />
                    </div>
                },
                AppView::Register => view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Register
                            on_success=move |_| set_current_view.set(AppView::Login)
                            on_back=move |_| set_current_view.set(AppView::Login)
                        />
                    </div>
                },
                _ => view! { <div>"Unexpected state"</div> }
            }.into_view()
        }}
        </div>
    }
}
