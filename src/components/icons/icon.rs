use leptos::*;

#[component]
pub fn Icon(icon: Signal<&'static str>, #[prop(optional)] class: &'static str) -> impl IntoView {
    let content = move || match icon.get() {
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
        "cog-6-tooth" => include_str!("../../assets/icons/cog-6-tooth.svg"),
        "information-circle" => include_str!("../../assets/icons/information-circle.svg"),
        "folder-open" => include_str!("../../assets/icons/folder-open.svg"),
        "archive-box" => include_str!("../../assets/icons/archive-box.svg"),
        "archive-box-arrow-down" => include_str!("../../assets/icons/archive-box-arrow-down.svg"),
        "chevron-down" => include_str!("../../assets/icons/chevron-down.svg"),
        "lock-closed" => include_str!("../../assets/icons/lock-closed.svg"),
        "paint-brush" => include_str!("../../assets/icons/paint-brush.svg"),
        "language" => include_str!("../../assets/icons/language.svg"),
        "rocket-launch" => include_str!("../../assets/icons/rocket-launch.svg"),
        "clock" => include_str!("../../assets/icons/clock.svg"),
        "shield-check" => include_str!("../../assets/icons/shield-check.svg"),
        "circle-stack" => include_str!("../../assets/icons/circle-stack.svg"),
        "cog" => include_str!("../../assets/icons/cog.svg"),
        "computer-desktop" => include_str!("../../assets/icons/computer-desktop.svg"),
        "arrow-down-tray" => include_str!("../../assets/icons/arrow-down-tray.svg"),
        "arrow-up-tray" => include_str!("../../assets/icons/arrow-up-tray.svg"),
        _ => "",
    };

    let new_icon = move || content().replace("class=\"size-6\"", &format!("class=\"{}\"", class));

    view! {
        <div inner_html={new_icon} />
    }
}
