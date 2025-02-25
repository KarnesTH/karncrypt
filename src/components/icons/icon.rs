use leptos::*;

#[component]
pub fn Icon(icon: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    let classes = format!("{}", class);

    let content = match icon {
        "eye" => include_str!("../../assets/icons/eye.svg"),
        "eye-slash" => include_str!("../../assets/icons/eye-slash.svg"),
        "arrow-path" => include_str!("../../assets/icons/arrow-path.svg"),
        "clipboard" => include_str!("../../assets/icons/clipboard.svg"),
        _ => "",
    };

    let new_icon = content.replace("class=\"size-6\"", &format!("class=\"{}\"", classes));

    view! {
        <div inner_html={new_icon} />
    }
}
