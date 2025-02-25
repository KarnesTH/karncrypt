use leptos::*;

#[component]
pub fn Icon(icon: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    let content = match icon {
        "eye" => include_str!("../../assets/icons/eye.svg"),
        "eye-slash" => include_str!("../../assets/icons/eye-slash.svg"),
        "arrow-path" => include_str!("../../assets/icons/arrow-path.svg"),
        "clipboard" => include_str!("../../assets/icons/clipboard.svg"),
        "arrow-left-start-on-rectangle" => {
            include_str!("../../assets/icons/arrow-left-start-on-rectangle.svg")
        }
        "arrow-up-down" => include_str!("../../assets/icons/arrow-up-down.svg"),
        "at-symbol" => include_str!("../../assets/icons/at-symbol.svg"),
        "check" => include_str!("../../assets/icons/check.svg"),
        "document-text" => include_str!("../../assets/icons/document-text.svg"),
        "funnel" => include_str!("../../assets/icons/funnel.svg"),
        "identification" => include_str!("../../assets/icons/identification.svg"),
        "key" => include_str!("../../assets/icons/key.svg"),
        "link" => include_str!("../../assets/icons/link.svg"),
        "magnifying-glass" => include_str!("../../assets/icons/magnifying-glass.svg"),
        "pencil-square" => include_str!("../../assets/icons/pencil-square.svg"),
        "plus" => include_str!("../../assets/icons/plus.svg"),
        "trash" => include_str!("../../assets/icons/trash.svg"),
        "user" => include_str!("../../assets/icons/user.svg"),
        "x-mark" => include_str!("../../assets/icons/x-mark.svg"),
        _ => "",
    };

    let new_icon = content.replace("class=\"size-6\"", &format!("class=\"{}\"", class));

    view! {
        <div inner_html={new_icon} />
    }
}
