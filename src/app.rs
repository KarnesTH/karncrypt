use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::components::icons::Icon;
use crate::components::password_manager::PasswordManager;

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
    let (_show_info, _set_show_info) = create_signal(false);
    let (_show_settings, set_show_settings) = create_signal(false);
    let (info_dropdown, set_info_dropdown) = create_signal(false);
    let info_dropdown_ref = create_node_ref::<html::Div>();

    let info_icon = create_memo(move |_| "information-circle");
    let settings_icon = create_memo(move |_| "cog-6-tooth");
    let logout_icon = create_memo(move |_| "arrow-left-start-on-rectangle");

    window_event_listener(ev::click, move |ev| {
        let target = ev.target();
        let dropdown_el = info_dropdown_ref.get();

        if let (Some(target), Some(dropdown)) = (target, dropdown_el) {
            let target_node: &Node = target.unchecked_ref();
            if !dropdown.contains(Some(target_node)) {
                set_info_dropdown.set(false);
            }
        }
    });

    spawn_local(async move {
        let response = invoke("check_is_initialized", JsValue::NULL).await;
        if let Ok(initialized) = serde_wasm_bindgen::from_value::<bool>(response) {
            set_is_initialized.set(initialized);
        }
    });

    let on_auth_success = move |_| {
        set_is_authenticated.set(true);
    };

    let on_logout = move |_| {
        spawn_local(async move {
            let response = invoke("logout", JsValue::NULL).await;
            if serde_wasm_bindgen::from_value::<()>(response).is_ok() {
                set_is_authenticated.set(false);
            }
        });
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
                                                "Passwörter"
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
                                    <div class="flex items-center space-x-6">
                                        <div class="relative">
                                            <button
                                                class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    set_info_dropdown.update(|state| *state = !*state)
                                                }
                                            >
                                                <Icon
                                                    icon=info_icon.into()
                                                    class="w-5 h-5"
                                                />
                                                <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                                                    "Info"
                                                </span>
                                            </button>

                                            {move || {
                                                if info_dropdown.get() {
                                                view! {
                                                    <div
                                                        _ref=info_dropdown_ref
                                                        class="absolute right-0 mt-2 w-48 py-2 bg-background-card rounded-lg shadow-xl"
                                                        on:click=move |ev| {
                                                            ev.stop_propagation();
                                                        }
                                                    >
                                                        <button
                                                            class="w-full text-left px-4 py-2 text-white hover:bg-background"
                                                            on:click=move |_| {
                                                                // TODO: Implement about modal
                                                                set_info_dropdown.set(false);
                                                            }
                                                        >
                                                            "About"
                                                        </button>
                                                        <button
                                                            class="w-full text-left px-4 py-2 text-white hover:bg-background"
                                                            on:click=move |_| {
                                                                // TODO: Implement guide modal
                                                                set_info_dropdown.set(false);
                                                            }
                                                        >
                                                            "Guide"
                                                        </button>
                                                    </div>
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                     <div />
                                                    }.into_view()
                                                }
                                            }}
                                        </div>
                                        <button
                                            class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                                            on:click=move |_| set_show_settings.set(true)
                                        >
                                            <Icon
                                                icon=settings_icon.into()
                                                class="w-5 h-5"
                                            />
                                            <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                                                "Einstellungen"
                                            </span>
                                        </button>
                                        <button
                                            class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                                            on:click=on_logout
                                        >
                                            <Icon
                                                icon=logout_icon.into()
                                                class="w-5 h-5"
                                            />
                                            <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                                                "Abmelden"
                                            </span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </nav>
                        <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                            {move || match current_tab.get() {
                                DashboardTab::Passwords => view! {
                                    <div class="bg-background-card shadow-lg rounded-lg p-6">
                                        <PasswordManager />
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
