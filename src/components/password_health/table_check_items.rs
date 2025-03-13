use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, PartialEq, Deserialize)]
pub struct TableCheckItemArgs {
    pub service: String,
    pub username: String,
    pub score: u8,
    pub strength: String,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
    pub last_modified: String,
}

impl TableCheckItemArgs {
    fn format_date(&self) -> String {
        let date = web_sys::js_sys::Date::new(&JsValue::from_str(&self.last_modified));

        format!(
            "{:02}.{:02}.{} {:02}:{:02} Uhr",
            date.get_date(),
            date.get_month() + 1,
            date.get_full_year(),
            date.get_hours(),
            date.get_minutes()
        )
    }

    fn format_relative_date(&self) -> String {
        let date = web_sys::js_sys::Date::new(&JsValue::from_str(&self.last_modified));
        let now = web_sys::js_sys::Date::new_0();
        let diff = now.get_time() - date.get_time();

        let days = (diff / (1000.0 * 60.0 * 60.0 * 24.0)).floor();
        let hours = (diff / (1000.0 * 60.0 * 60.0)).floor();
        let minutes = (diff / (1000.0 * 60.0)).floor();

        if days > 30.0 {
            format!("vor {} Monaten", (days / 30.0).floor())
        } else if days > 0.0 {
            format!("vor {} Tagen", days)
        } else if hours > 0.0 {
            format!("vor {} Stunden", hours)
        } else if minutes > 0.0 {
            format!("vor {} Minuten", minutes)
        } else {
            "gerade eben".to_string()
        }
    }
}

#[component]
pub fn TableCheckItems(items: TableCheckItemArgs) -> impl IntoView {
    let item = create_memo(move |_| items.clone());

    let strength_color = move |strength: &str| match strength {
        "VeryWeak" => "bg-red-500",
        "Weak" => "bg-orange-500",
        "Moderate" => "bg-yellow-500",
        "Strong" => "bg-green-500",
        "VeryStrong" => "bg-primary-100",
        _ => "bg-gray-500",
    };

    view! {
        <tr class="hover:bg-background">
            <td class="p-4">
                <div class="flex flex-col">
                    <span>{move || item.get().service}</span>
                    <span class="text-sm text-gray-400">{move || item.get().username}</span>
                </div>
            </td>
            <td class="p-4">
                <div class="flex items-center">
                    <span class="text-primary-100 font-medium">{move || item.get().score}</span>
                    <div class="w-24 h-2 bg-gray-700 rounded-full ml-2">
                        <div
                            class="h-2 bg-primary-100 rounded-full"
                            style={move || format!("width: {}%", item.get().score)}
                        />
                    </div>
                </div>
            </td>
            <td class="p-4">
                <div class="flex flex-col gap-2">
                    <div class="flex items-center">
                        <div
                            class={move || format!("w-2 h-2 rounded-full {} mr-2",
                                strength_color(&item.get().strength))}
                        />
                        <span>{move || item.get().strength}</span>
                    </div>
                    <div class="text-sm text-gray-400">
                        {move || item.get().issues.iter().map(|issue| {
                            view! {
                                <span class="inline-block px-2 py-1 rounded-full bg-background-light mr-1 mb-1">
                                    {issue}
                                </span>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </td>
            <td class="p-4 text-gray-400">
                <ul class="list-disc list-inside space-y-1">
                    {move || item.get().suggestions.iter().map(|suggestion| {
                        view! {
                            <li>{suggestion}</li>
                        }
                    }).collect_view()}
                </ul>
            </td>
                <td class="p-4 text-sm text-gray-400">
                    <span title={move || item.get().format_date()}>
                        {move || item.get().format_relative_date()}
                    </span>
                </td>
        </tr>
    }
}
