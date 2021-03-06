use super::indent_square;

const LENGTH_UNIT: u16 = 30;

pub const SIDEBAR_LENGTH: u16 = LENGTH_UNIT * 6;
pub const FILTER_METHOD_BUTTON_LENGTH: u16 = LENGTH_UNIT * 3;
pub const MASS_CHECK_BUTTON_LENGTH: u16 = LENGTH_UNIT * 2;
pub const TAG_PREFIX_LENGTH: u16 = indent_square::SIZE;
pub const TAG_CONTENT_LENGTH: u16 = SIDEBAR_LENGTH - TAG_PREFIX_LENGTH;

#[test]
fn test_sizes() {
    assert_eq!(
        FILTER_METHOD_BUTTON_LENGTH,
        SIDEBAR_LENGTH / 2,
        "FILTER_METHOD_BUTTON_LENGTH",
    );
    assert_eq!(
        MASS_CHECK_BUTTON_LENGTH,
        SIDEBAR_LENGTH / 3,
        "MASS_CHECK_BUTTON_LENGTH",
    );
    assert_eq!(
        TAG_PREFIX_LENGTH + TAG_CONTENT_LENGTH,
        SIDEBAR_LENGTH,
        "TAG_PREFIX_LENGTH + TAG_CONTENT_LENGTH",
    );
}
