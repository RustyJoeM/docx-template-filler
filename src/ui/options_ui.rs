use super::consts;
use crate::lang;
use nwd::NwgPartial;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{AlignItems, Dimension, FlexDirection};

#[derive(Default, NwgPartial)]
pub struct OptionsUi {
    #[nwg_layout(flex_direction: FlexDirection::Row, align_items: AlignItems::Center)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "\u{2328}:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Points(30.0), height: consts::INPUT_HEIGHT })]
    lang_label: nwg::Label,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Points(140.0), height: consts::BUTTON_HEIGHT })]
    fixer_frame: nwg::Frame,
    #[nwg_layout(parent: fixer_frame, flex_direction: FlexDirection::Row, align_items: AlignItems::Center)]
    fixer_layout: nwg::FlexboxLayout,
    #[nwg_control(parent: fixer_frame, collection: supported_langs(), selected_index: Some(0))]
    #[nwg_layout_item(layout: fixer_layout, size: Size { width: Dimension::Points(100.0), height: consts::BUTTON_HEIGHT })]
    pub lang_dropdown: nwg::ComboBox<String>,

    #[nwg_control(text: &lang::tr("ui-options-sep-label"), h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, flex_grow: 1.0, min_size: Size { width: Dimension::Points(140.0), height: consts::INPUT_HEIGHT })]
    separator_label: nwg::Label,

    #[nwg_control(text: ";")]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Points(100.0), height: consts::INPUT_HEIGHT })]
    separator_input: nwg::TextInput,
}

impl OptionsUi {
    /// Gets value of currently configured value separator, used for splitting each line of value tex field.
    pub fn get_separator(&self) -> String {
        self.separator_input.text()
    }

    pub fn set_current_lang(&self) -> Result<(), String> {
        if let Some(new_lang_str) = self.lang_dropdown.selection_string() {
            if let Some(new_lang) = lang::SupportedLanguage::from_string(&new_lang_str) {
                lang::set_current_lang(new_lang);
                return Ok(());
            }
        }
        Err(lang::tr("lang-not-found"))
    }

    pub fn reset_language(&self) {
        self.separator_label
            .set_text(&lang::tr("ui-options-sep-label"));
    }
}

fn supported_langs() -> Vec<String> {
    lang::SupportedLanguage::collection()
        .iter()
        .map(|x| x.to_name())
        .collect()
}
