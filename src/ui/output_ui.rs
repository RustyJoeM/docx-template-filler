use super::consts;
use crate::lang::tr;
use nwd::NwgPartial;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{AlignContent, AlignItems, Dimension, FlexDirection, FlexWrap};

#[derive(Default, NwgPartial)]
pub struct OutputUi {
    #[nwg_layout(flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_items: AlignItems::Center, align_content: AlignContent::FlexStart)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: &tr("ui-output-label"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Percent(1.0), height: consts::INPUT_HEIGHT })]
    label: nwg::Label,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, flex_grow: 1.0, size: Size { width: Dimension::Auto, height: consts::INPUT_HEIGHT })]
    input: nwg::TextInput,

    #[nwg_control(text: &tr("ui-output-button"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Points(180.0), height: consts::BUTTON_HEIGHT })]
    pub button: nwg::Button,
}

impl OutputUi {
    pub fn output_pattern(&self) -> String {
        self.input.text()
    }

    pub fn set_output_pattern(&self, pattern: &str) {
        self.input.set_text(pattern);
    }

    pub fn reset_language(&self) {
        self.label.set_text(&tr("ui-output-label"));
        self.button.set_text(&tr("ui-output-button"));
    }
}
