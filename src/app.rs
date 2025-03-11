use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::components::icons::Icon;
use crate::components::password_health::Dashboard;
use crate::components::password_manager::PasswordManager;
use crate::components::settings::SettingsPanel;

use super::components::auth::Login;
use super::components::info::{About, Guide, InfoModal, License};
use super::components::password_generator::PasswordGenerator;
use super::components::setup::Init;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq)]
enum DashboardTab {
    Dashboard,
    Passwords,
    Generator,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_initialized, set_is_initialized) = create_signal(false);
    let (is_authenticated, set_is_authenticated) = create_signal(false);
    let (current_tab, set_current_tab) = create_signal(DashboardTab::Dashboard);
    let (show_about, set_show_about) = create_signal(false);
    let (show_guide, set_show_guide) = create_signal(false);
    let (show_license, set_show_license) = create_signal(false);
    let (show_settings, set_show_settings) = create_signal(false);
    let (info_dropdown, set_info_dropdown) = create_signal(false);
    let info_dropdown_ref = create_node_ref::<html::Div>();

    let info_icon = create_memo(move |_| "information-circle");
    let settings_icon = create_memo(move |_| "cog-6-tooth");
    let logout_icon = create_memo(move |_| "arrow-left-start-on-rectangle");
    let about_icon = create_memo(move |_| "information-circle");
    let guide_icon = create_memo(move |_| "book-open");
    let license_icon = create_memo(move |_| "document-text");
    let passwords_icon = create_memo(move |_| "key");
    let generator_icon = create_memo(move |_| "sparkles");
    let dashboard_icon = create_memo(move |_| "chart-pie");

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

    create_effect(move |_| {
        if is_authenticated.get() {
            let check_interval = 30_000;

            let closure = Closure::wrap(Box::new(move || {
                spawn_local(async move {
                    let response = invoke("check_users_session", JsValue::NULL).await;
                    if let Ok(is_valid) = serde_wasm_bindgen::from_value::<bool>(response) {
                        if !is_valid {
                            let window = web_sys::window().unwrap();
                            window.location().reload().unwrap();
                        }
                    }
                });
            }) as Box<dyn FnMut()>);

            let window = web_sys::window().unwrap();
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    check_interval,
                )
                .unwrap();

            closure.forget();
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
                                                    if current_tab.get() == DashboardTab::Dashboard {
                                                        "border-primary-100 text-white"
                                                    } else {
                                                        "border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600"
                                                    }
                                                )
                                                on:click=move |_| set_current_tab.set(DashboardTab::Dashboard)
                                            >
                                                <Icon icon=dashboard_icon.into() class="w-5 h-5 mr-2" />
                                                "Übersicht"
                                            </button>

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
                                                <Icon icon=passwords_icon.into() class="w-5 h-5 mr-2" />
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
                                                <Icon icon=generator_icon.into() class="w-5 h-5 mr-2" />
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
                                                        class="absolute right-0 mt-2 w-48 py-2 bg-background-card rounded-lg shadow-xl z-50"
                                                        on:click=move |ev| {
                                                            ev.stop_propagation();
                                                        }
                                                    >
                                                        <button
                                                            class="w-full flex items-center text-left px-4 py-2 text-gray-400 hover:text-white hover:bg-background transition-colors"
                                                            on:click=move |_| {
                                                                set_show_about.set(true);
                                                                set_info_dropdown.set(false);
                                                            }
                                                        >
                                                            <Icon icon=about_icon.into() class="w-5 h-5" />
                                                            <span class="ml-2">"About"</span>
                                                        </button>
                                                        <button
                                                            class="w-full flex items-center text-left px-4 py-2 text-gray-400 hover:text-white hover:bg-background transition-colors"
                                                            on:click=move |_| {
                                                                set_show_guide.set(true);
                                                                set_info_dropdown.set(false);
                                                            }
                                                        >
                                                            <Icon icon=guide_icon.into() class="w-5 h-5" />
                                                            <span class="ml-2">"Guide"</span>
                                                        </button>
                                                        <button
                                                            class="w-full flex items-center text-left px-4 py-2 text-gray-400 hover:text-white hover:bg-background transition-colors"
                                                            on:click=move |_| {
                                                                set_show_license.set(true);
                                                                set_info_dropdown.set(false);
                                                            }
                                                        >
                                                            <Icon icon=license_icon.into() class="w-5 h-5" />
                                                            <span class="ml-2">"Lizenz"</span>
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
                                DashboardTab::Dashboard => view! {
                                    <div class="bg-background-card shadow-lg rounded-lg p-6 h-[calc(100vh-150px)]">
                                        <Dashboard />
                                    </div>
                                }.into_view(),
                                DashboardTab::Passwords => view! {
                                    <div class="bg-background-card shadow-lg rounded-lg p-6 h-[calc(100vh-150px)]">
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

            {move || {
                if show_about.get() {
                    view! {
                        <InfoModal
                            title="About".to_string()
                            icon="information-circle"
                            on_close=move |_| set_show_about.set(false)
                        >
                            <About />
                        </InfoModal>
                    }.into_view()
                } else if show_guide.get() {
                    view! {
                        <InfoModal
                            title="Passwort Guide".to_string()
                            icon="book-open"
                            on_close=move |_| set_show_guide.set(false)
                        >
                            <Guide/>
                        </InfoModal>
                    }.into_view()
                } else if show_license.get() {
                    view! {
                        <InfoModal
                            title="Lizenz".to_string()
                            icon="document-text"
                            on_close=move |_| set_show_license.set(false)
                        >
                            <License />
                        </InfoModal>
                    }.into_view()
                } else {
                    view! {
                        <div />
                    }.into_view()
                }
            }}

            {move || {
                if show_settings.get() {
                    view! {
                        <SettingsPanel on_close=move |_| set_show_settings.set(false) />
                    }.into_view()
                } else {
                    view! {
                        <div />
                    }.into_view()
                }
            }}
        </div>
    }
}
