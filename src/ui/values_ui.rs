use super::consts;
use crate::lang;
use nwd::NwgPartial;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{Dimension, FlexDirection};

#[derive(Default, NwgPartial)]
pub struct ValuesUi {
    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: &lang::tr("ui-values-label"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Percent(1.0), height: consts::INPUT_HEIGHT })]
    label: nwg::Label,

    // #[nwg_resource(family: "Courier New")]
    // font_fixed: nwg::Font,
    #[nwg_control(/*font: Some(&data.font_fixed)*/)]
    #[nwg_layout_item(layout: layout, flex_grow: 1.0, size: Size { width: Dimension::Percent(1.0), height: Dimension::Auto })]
    input: nwg::TextBox,
}

impl ValuesUi {
    pub fn get_values_text(&self) -> String {
        self.input.text()
    }

    pub fn reset_language(&self) {
        self.label.set_text(&lang::tr("ui-values-label"));
    }
}
