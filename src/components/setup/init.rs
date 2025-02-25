use crate::{app::invoke, components::auth::Register};
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct CompleteSetupArgs<'a> {
    custom_path: Option<&'a str>,
}

#[derive(Clone, Copy, PartialEq)]
enum InitStep {
    Welcome,
    Database,
    MasterPassword,
}

#[component]
pub fn Init(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(InitStep::Welcome);
    let (db_path, set_db_path) = create_signal(String::new());

    view! {
        <div class="min-h-screen flex items-center justify-center bg-background">
            <div class="w-full max-w-md">
                <div class="mb-8">
                    <div class="flex justify-between">
                        <div class="text-center">
                            <div class=move || format!("rounded-full h-8 w-8 flex items-center justify-center border-2 {}",
                                match current_step.get() {
                                    InitStep::Welcome => "border-primary-100 text-primary-100",
                                    _ => "border-gray-400 text-gray-400"
                                }
                            )>
                                "1"
                            </div>
                            <div class="text-xs mt-1 text-white">"Willkommen"</div>
                        </div>
                        <div class="text-center">
                            <div class=move || format!("rounded-full h-8 w-8 flex items-center justify-center border-2 {}",
                                match current_step.get() {
                                    InitStep::Database => "border-primary-100 text-primary-100",
                                    _ => "border-gray-400 text-gray-400"
                                }
                            )>
                                "2"
                            </div>
                            <div class="text-xs mt-1 text-white">"Datenbank"</div>
                        </div>
                        <div class="text-center">
                            <div class=move || format!("rounded-full h-8 w-8 flex items-center justify-center border-2 {}",
                                match current_step.get() {
                                    InitStep::MasterPassword => "border-primary-100 text-primary-100",
                                    _ => "border-gray-400 text-gray-400"
                                }
                            )>
                                "3"
                            </div>
                            <div class="text-xs mt-1 text-white">"Master-Passwort"</div>
                        </div>
                    </div>
                </div>
                {move || match current_step.get() {
                    InitStep::Welcome => view! {
                        <div class="bg-background-card rounded-lg p-6 shadow-lg">
                            <h2 class="text-xl font-bold mb-4 text-white">"Willkommen beim Password Manager"</h2>
                            <p class="text-gray-300 mb-6">
                                "In den nächsten Schritten richten wir Ihren Password Manager ein:"
                            </p>
                            <ul class="list-disc list-inside text-gray-300 mb-6 space-y-2">
                                <li>"Einrichtung der verschlüsselten Datenbank"</li>
                                <li>"Erstellung eines sicheren Master-Passworts"</li>
                                <li class="text-primary-100 font-semibold">
                                    "WICHTIG: Das Master-Passwort kann nicht wiederhergestellt werden!"
                                </li>
                            </ul>
                            <button
                                class="w-full bg-gradient-primary text-white font-bold py-2 px-4 rounded transition-all hover:opacity-90"
                                on:click=move |_| set_current_step.set(InitStep::Database)
                            >
                                "Setup starten"
                            </button>
                        </div>
                    }.into_view(),
                    InitStep::Database => view! {
                        <div class="bg-background-card rounded-lg p-6 shadow-lg">
                            <h2 class="text-xl font-bold mb-4 text-white">"Datenbank Setup"</h2>
                            <p class="text-gray-300 mb-4">
                                "Standardmäßig wird die Datenbank im Konfigurationsverzeichnis gespeichert. "
                                "Optional können Sie einen benutzerdefinierten Pfad angeben."
                            </p>
                            <div class="mb-4">
                                <input
                                    type="text"
                                    placeholder="Optionaler Datenbankpfad"
                                    class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none"
                                    on:input=move |ev| set_db_path.set(event_target_value(&ev))
                                    prop:value=db_path
                                />
                            </div>
                            <button
                                class="w-full bg-gradient-primary text-white font-bold py-2 px-4 rounded transition-all hover:opacity-90"
                                on:click=move |_| set_current_step.set(InitStep::MasterPassword)
                            >
                                "Weiter"
                            </button>
                        </div>
                    }.into_view(),
                    InitStep::MasterPassword => view! {
                        <div class="bg-background-card rounded-lg p-6 shadow-lg">
                            <h2 class="text-xl font-bold mb-4 text-white">"Master-Passwort erstellen"</h2>
                            <Register
                                on_success=move |_| {
                                    spawn_local(async move {
                                        let path = db_path.get();
                                        let args = serde_wasm_bindgen::to_value(&CompleteSetupArgs {
                                            custom_path: if path.is_empty() { None } else { Some(&path) }
                                        }).unwrap();

                                        let response = invoke("complete_setup", args).await;
                                        if serde_wasm_bindgen::from_value::<()>(response).is_ok() {
                                            on_complete.call(());
                                        }
                                    });
                                }
                            />
                        </div>
                    }.into_view(),
                }}
            </div>
        </div>
    }
}
