#[derive(Clone, Copy)]
pub enum UIType {
    Button,
    Text,
    Slider,
    Input,
    Checkbox,
    Dropdown,
    Radio,
    List,
    Table,
}

pub const UIMAP: &[(&str,UIType)] = &[
    ("button",UIType::Button),
    ("text",UIType::Text),
    ("slider",UIType::Slider),
    ("input",UIType::Input),
    ("checkbox",UIType::Checkbox),
    ("dropdown",UIType::Dropdown),
    ("radio",UIType::Radio),
    ("list",UIType::List),
    ("table",UIType::Table),
];

impl UIType {
    pub fn from_str(s: &str) -> Option<UIType> {
        for (name, ui_type) in UIMAP {
            if s == *name {
                return Some(*ui_type);
            }
        }
        None
    }
}