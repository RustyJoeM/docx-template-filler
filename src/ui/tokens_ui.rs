use super::consts;
use crate::docx_filler::TokenPack;
use crate::lang::tr;
use nwd::NwgPartial;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{AlignContent, AlignItems, Dimension, FlexDirection, FlexWrap, Style};
use nwg::HTextAlign;
use std::cell::RefCell;

#[derive(Default, NwgPartial)]
pub struct TokensUi {
    #[nwg_layout(flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_items: AlignItems::FlexStart, align_content: AlignContent::Center)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: &tr("ui-tokens-label"))]
    #[nwg_layout_item(layout: layout, size: Size { width: Dimension::Percent(1.0), height: consts::INPUT_HEIGHT })]
    label: nwg::Label,

    dropdowns: RefCell<Vec<nwg::ComboBox<String>>>,
    separators: RefCell<Vec<nwg::Label>>,
}

impl TokensUi {
    pub fn get_selected_tokens(&self) -> TokenPack {
        let dds = self.dropdowns.borrow();
        let tokens = dropdowns_to_tokens(&dds);
        tokens
    }

    pub fn set_tokens_of_frame(&self, frame: &nwg::Frame, tokens: &TokenPack, sep_str: &str) {
        clear_objects_from_layout(&self.dropdowns, &self.layout);
        clear_objects_from_layout(&self.separators, &self.layout);

        for i in 0..tokens.len() {
            if i > 0 {
                self.add_new_separator(frame, sep_str);
            }
            self.add_new_token(frame, tokens, i);
        }
    }

    fn add_new_separator(&self, frame: &nwg::Frame, sep_str: &str) {
        let mut new_sep: nwg::Label = nwg::Label::default();
        nwg::Label::builder()
            .parent(frame)
            .text(sep_str)
            .h_align(HTextAlign::Center)
            .build(&mut new_sep)
            .expect(&tr("ui-tokens-failed-sep-create"));
        let style = Style {
            size: Size {
                width: Dimension::Points(20.0),
                height: consts::INPUT_HEIGHT,
            },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        };
        self.layout
            .add_child(&new_sep, style)
            .expect(&tr("ui-tokens-failed-sep-add"));
        self.separators.borrow_mut().push(new_sep);
    }

    // init & bind new token dropdown to running window
    fn add_new_token(&self, frame: &nwg::Frame, values: &TokenPack, selected_index: usize) {
        let new_coll = values.to_owned();
        let mut new_dd: nwg::ComboBox<String> = nwg::ComboBox::<String>::default();

        nwg::ComboBox::builder()
            .collection(new_coll)
            .selected_index(Some(selected_index))
            .parent(frame)
            .build(&mut new_dd)
            .expect(&tr("ui-tokens-failed-tok-create"));

        let style = Style {
            size: Size {
                width: Dimension::Auto,
                height: consts::INPUT_HEIGHT,
            },
            flex_grow: 1.0,
            // justify_content: JustifyContent::Center,
            ..Default::default()
        };
        self.layout
            .add_child(&new_dd, style)
            .expect(&tr("ui-tokens-failed-tok-add"));
        self.dropdowns.borrow_mut().push(new_dd);
    }

    pub fn reset_language(&self) {
        self.label.set_text(&tr("ui-tokens-label"));
    }
}

fn clear_objects_from_layout<W>(keeper: &RefCell<Vec<W>>, layout: &nwg::FlexboxLayout)
where
    for<'local> &'local W: Into<nwg::ControlHandle>,
{
    for d in keeper.borrow().iter() {
        if layout.has_child(d) {
            layout.remove_child(d);
        }
    }
    keeper.replace(Default::default());
}

fn dropdowns_to_tokens(dropdowns: &Vec<nwg::ComboBox<String>>) -> TokenPack {
    dropdowns
        .iter()
        .map(|dd| dd.selection_string())
        .flatten()
        .collect()
}
