use ds1307::Error;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(can_read_wd, get_weekday, 7, trans_read!(DOW, [7]));

set_invalid_test!(too_small, set_weekday, 0);
set_invalid_test!(too_big, set_weekday, 8);

set_test!(can_set_wd, set_weekday, 7, trans_write!(DOW, [0b0000_0111]));
