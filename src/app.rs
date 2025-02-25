use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::icons::Icon;

use super::components::auth::Login;
use super::components::password_generator::PasswordGenerator;
use super::components::setup::Init;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq)]
enum DashboardTab {
    Passwords,
    Generator,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_initialized, set_is_initialized) = create_signal(false);
    let (is_authenticated, set_is_authenticated) = create_signal(false);
    let (current_tab, set_current_tab) = create_signal(DashboardTab::Passwords);
    let (_user_id, set_user_id) = create_signal(None::<i32>);

    spawn_local(async move {
        let response = invoke("check_is_initialized", JsValue::NULL).await;
        if let Ok(initialized) = serde_wasm_bindgen::from_value::<bool>(response) {
            set_is_initialized.set(initialized);
        }
    });

    let on_auth_success = move |id: i32| {
        set_user_id.set(Some(id));
        set_is_authenticated.set(true);
    };

    let on_logout = move |_| {
        set_is_authenticated.set(false);
        set_user_id.set(None);
    };

    view! {
        <div class="min-h-screen bg-background">
            {move || {
                if !is_initialized.get() {
                    view! {
                        <Init
                            on_complete=move |_| {
                                set_is_initialized.set(true);
                            }
                        />
                    }.into_view()
                } else if is_authenticated.get() {
                    view! {
                        <nav class="bg-background-card shadow-lg">
                            <div class="max-w-7xl mx-auto px-4">
                                <div class="flex justify-between h-16">
                                    <div class="flex">
                                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                                            <button
                                                class=move || format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
                                                    if current_tab.get() == DashboardTab::Passwords {
                                                        "border-primary-100 text-white"
                                                    } else {
                                                        "border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600"
                                                    }
                                                )
                                                on:click=move |_| set_current_tab.set(DashboardTab::Passwords)
                                            >
                                                "Passwords"
                                            </button>
                                            <button
                                                class=move || format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
                                                    if current_tab.get() == DashboardTab::Generator {
                                                        "border-primary-100 text-white"
                                                    } else {
                                                        "border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600"
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
                                            class="bg-gradient-primary text-white font-bold py-2 px-4 rounded hover:opacity-90 transition-opacity flex items-center"
                                            on:click=on_logout
                                        >
                                            <Icon
                                                icon="arrow-left-start-on-rectangle"
                                                class="w-5 h-5 mr-2"
                                            />
                                            "Logout"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </nav>
                        <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                            {move || match current_tab.get() {
                                DashboardTab::Passwords => view! {
                                    <div class="bg-background-card shadow-lg rounded-lg p-6">
                                        <h2 class="text-lg font-semibold mb-4 text-white">"Your Passwords"</h2>
                                        // Hier kommt später die Passwort-Tabelle
                                    </div>
                                }.into_view(),
                                DashboardTab::Generator => view! {
                                    <div class="bg-background-card shadow-lg rounded-lg p-6">
                                        <PasswordGenerator />
                                    </div>
                                }.into_view()
                            }}
                        </main>
                    }.into_view()
                } else {
                    view! {
                        <div class="min-h-screen flex items-center justify-center">
                            <Login on_success=on_auth_success />
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
