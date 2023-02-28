use black76::{Greeks, ImpliedVolatility, Pricing};

const INPUTS_CALL_OTM: black76::Inputs = black76::Inputs {
    option_type: black76::OptionType::Call,
    f: 100.0,
    k: 110.0,
    p: None,
    r: 0.05,
    t: 20.0 / 365.25,
    sigma: Some(0.2),
};
const INPUTS_CALL_ITM: black76::Inputs = black76::Inputs {
    option_type: black76::OptionType::Call,
    f: 100.0,
    k: 90.0,
    p: None,
    r: 0.05,
    t: 20.0 / 365.25,
    sigma: Some(0.2),
};
const INPUTS_PUT_OTM: black76::Inputs = black76::Inputs {
    option_type: black76::OptionType::Put,
    f: 100.0,
    k: 90.0,
    p: None,
    r: 0.05,
    t: 20.0 / 365.25,
    sigma: Some(0.2),
};
const INPUTS_PUT_ITM: black76::Inputs = black76::Inputs {
    option_type: black76::OptionType::Put,
    f: 100.0,
    k: 110.0,
    p: None,
    r: 0.05,
    t: 20.0 / 365.25,
    sigma: Some(0.2),
};

#[test]
fn price_call_otm() {
    assert!((INPUTS_CALL_OTM.calc_price().unwrap() - 0.0376).abs() < 0.001);
}
#[test]
fn price_call_itm() {
    assert!((INPUTS_CALL_ITM.calc_price().unwrap() - 9.9913).abs() < 0.001);
}

#[test]
fn price_put_otm() {
    assert!((INPUTS_PUT_OTM.calc_price().unwrap() - 0.01867).abs() < 0.001);
}
#[test]
fn price_put_itm() {
    assert!((INPUTS_PUT_ITM.calc_price().unwrap() - 10.0103).abs() < 0.001);
}
