use leptos::*;

use crate::components::{
    icons::Icon,
    settings::{ApplicationSettings, DatabaseSettings, SecuritySettings, SystemSettings},
};

#[derive(Clone, Copy, PartialEq)]
enum SettingsSection {
    Security,
    Database,
    Application,
    System,
}

#[component]
pub fn SettingsPanel(#[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let (active_section, set_active_section) = create_signal(SettingsSection::Security);

    let exit_icon = create_memo(move |_| "x-mark");
    let shield_icon = create_memo(move |_| "shield-check");
    let database_icon = create_memo(move |_| "circle-stack");
    let application_icon = create_memo(move |_| "cog");
    let system_icon = create_memo(move |_| "computer-desktop");

    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50">
            <div class="absolute inset-0 bg-background-card shadow-lg flex flex-col">
                <div class="flex justify-between items-center p-6 border-b border-gray-600">
                    <h2 class="text-xl font-bold bg-gradient-primary bg-clip-text text-transparent">
                        "Einstellungen"
                    </h2>
                    <button
                        class="text-gray-400 hover:text-white transition-colors"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=exit_icon.into() class="w-5 h-5" />
                    </button>
                </div>

                <div class="flex flex-1 overflow-hidden">
                    <div class="w-64 border-r border-gray-600 overflow-y-auto">
                        <nav class="p-4 space-y-2">
                            <button
                                class=move || format!("group w-full flex items-center text-left px-4 py-2 rounded transition-colors {}",
                                    if active_section.get() == SettingsSection::Security {
                                        "bg-background text-primary-100"
                                    } else {
                                        "text-gray-400 hover:text-white hover:bg-background"
                                    }
                                )
                                on:click=move |_| set_active_section.set(SettingsSection::Security)
                            >
                                <Icon icon=shield_icon.into() class="w-5 h-5" />
                                <span class="ml-2">"Sicherheit"</span>
                            </button>

                            <button
                                class=move || format!("group w-full flex items-center text-left px-4 py-2 rounded transition-colors {}",
                                    if active_section.get() == SettingsSection::Database {
                                        "bg-background text-primary-100"
                                    } else {
                                        "text-gray-400 hover:text-white hover:bg-background"
                                    }
                                )
                                on:click=move |_| set_active_section.set(SettingsSection::Database)
                            >
                                <Icon icon=database_icon.into() class="w-5 h-5" />
                                <span class="ml-2">"Datenbank"</span>
                            </button>

                            <button
                                class=move || format!("group w-full flex items-center text-left px-4 py-2 rounded transition-colors {}",
                                    if active_section.get() == SettingsSection::Application {
                                        "bg-background text-primary-100"
                                    } else {
                                        "text-gray-400 hover:text-white hover:bg-background"
                                    }
                                )
                                on:click=move |_| set_active_section.set(SettingsSection::Application)
                            >
                                <Icon icon=application_icon.into() class="w-5 h-5" />
                                <span class="ml-2">"Anwendung"</span>
                            </button>

                            <button
                                class=move || format!("group w-full flex items-center text-left px-4 py-2 rounded transition-colors {}",
                                    if active_section.get() == SettingsSection::System {
                                        "bg-background text-primary-100"
                                    } else {
                                        "text-gray-400 hover:text-white hover:bg-background"
                                    }
                                )
                                on:click=move |_| set_active_section.set(SettingsSection::System)
                            >
                                <Icon icon=system_icon.into() class="w-5 h-5" />
                                <span class="ml-2">"System"</span>
                            </button>
                        </nav>
                    </div>

                    <div class="flex-1 overflow-y-auto p-6">
                        {move || match active_section.get() {
                            SettingsSection::Security => view! {
                                <SecuritySettings />
                            }.into_view(),
                            SettingsSection::Database => view! {
                                <DatabaseSettings />
                            }.into_view(),
                            SettingsSection::Application => view! {
                                <ApplicationSettings />
                            }.into_view(),
                            SettingsSection::System => view! {
                                <SystemSettings />
                            }.into_view(),
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
