use nwg::stretch::geometry::{Rect, Size};
use nwg::stretch::style::Dimension;

/// primary sizing constant
const LINE_HEIGHT: f32 = 20.0;

/// Common height for text input across the app UI.
pub const INPUT_HEIGHT: Dimension = Dimension::Points(LINE_HEIGHT);

/// Common height for buttons across the app UI.
pub const BUTTON_HEIGHT: Dimension = Dimension::Points(1.5 * LINE_HEIGHT);

/// Common size to unify controls that are "one line of UI elements" tall.
pub const MIN_ONELINER_SIZE: Size<Dimension> = Size {
    width: Dimension::Percent(1.0),
    height: Dimension::Points(2.0 * LINE_HEIGHT),
};

/// Common size to unify controls that are "two lines of UI elements" tall.
pub const MIN_TWOLINER_SIZE: Size<Dimension> = Size {
    width: Dimension::Percent(1.0),
    height: Dimension::Points(4.0 * LINE_HEIGHT),
};

/// Padding for the main application window
pub const WINDOW_PAD: Rect<Dimension> = Rect {
    start: W_PT,
    end: W_PT,
    top: W_PT,
    bottom: W_PT,
};

const W_PT: Dimension = Dimension::Points(0.6 * LINE_HEIGHT);
