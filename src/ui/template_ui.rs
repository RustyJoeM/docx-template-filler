use super::consts;
use crate::lang::tr;
use nwd::NwgPartial;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{AlignContent, AlignItems, Dimension, FlexDirection, FlexWrap};
use std::env;

#[derive(Default, NwgPartial)]
pub struct TemplateUi {
    #[nwg_layout(flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_items: AlignItems::Center, align_content: AlignContent::FlexStart)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: &tr("ui-template-label"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Percent(1.0), height: consts::INPUT_HEIGHT })]
    label: nwg::Label,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: layout, flex_grow: 1.0, size: Size { width: Dimension::Auto, height: consts::INPUT_HEIGHT })]
    input: nwg::TextInput,

    #[nwg_resource(title: &tr("ui-template-dialog"), action: nwg::FileDialogAction::Open, filters: "docx(*.docx)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: &tr("ui-template-button"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Points(180.0), height: consts::BUTTON_HEIGHT })]
    pub button: nwg::Button,
}

impl TemplateUi {
    pub fn get_browse_file<C: Into<nwg::ControlHandle>>(&self, window: C) -> Option<String> {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog
                    .set_default_folder(d)
                    .unwrap_or_else(|_| panic!("{}", tr("ui-template-default-folder-fail")));
            }
        }
        if self.dialog.run(Some(window)) {
            if let Ok(file) = self.dialog.get_selected_item() {
                let s = file.to_str()?;
                return Some(s.to_owned());
            }
        }
        None
    }

    pub fn set_current_docx(&self, file: &str) {
        self.input.set_text(file);
    }

    pub fn reset_language(&self) {
        self.label.set_text(&tr("ui-template-label"));
        self.dialog.set_title(&tr("ui-template-dialog"));
        self.button.set_text(&tr("ui-template-button"));
    }
}
