fn main() {
    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("xyz.naritanara.banking-helper"),
        None::<&str>,
        None::<&str>,
        [
            "key4",
            "puzzle-piece",
            "verified-checkmark",
            "sentiment-dissatisfied",
        ],
    );
}
