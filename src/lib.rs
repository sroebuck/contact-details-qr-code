use chrono::Local;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let now = Local::now();
    Model {
        name: "".into(),
        telephone: "".into(),
        date: format!("{:?}", now),
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    name: String,
    telephone: String,
    date: String,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    NameChanged(String),
    TelephoneChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(name) => model.name = name,
        Msg::TelephoneChanged(telephone) => model.telephone = telephone,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    let vcard_string = vcard(&model.name, &model.telephone, &model.date);

    let phone_number_is_valid =
        phonenumber::parse(Some(phonenumber::country::Id::GB), &model.telephone)
            .map(|n| phonenumber::is_valid(&n))
            .unwrap_or(false);
    let svg = if model.name.len() > 5 && model.name.trim().contains(" ") && phone_number_is_valid {
        let qr = QrCode::encode_text(&vcard_string, QrCodeEcc::Medium).unwrap();
        qr.to_svg_string(4)
    } else {
        String::from("")
    };

    form![
        C!["pure-form pure-form-aligned"],
        fieldset![
            div![
                C!["pure-control-group"],
                label!["Name"],
                input![
                    C!["qr-name"],
                    attrs! {
                        At::Placeholder => "Name?",
                        At::AutoFocus => AtValue::None,
                        At::Value => model.name,
                    },
                    input_ev(Ev::Input, Msg::NameChanged)
                ],
            ],
            div![
                C!["pure-control-group"],
                label!["Telephone Number"],
                input![
                    C!["qr-telephone"],
                    attrs! {
                        At::Placeholder => "Telephone?",
                        At::AutoFocus => AtValue::None,
                        At::Value => model.telephone,
                    },
                    input_ev(Ev::Input, Msg::TelephoneChanged)
                ],
            ],
        ],
        raw!(&svg),
    ]
}

fn vcard(name: &str, telephone: &str, date: &str) -> String {
    format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN;CHARSET=UTF-8:{}\nTEL;TYPE=HOME,VOICE:{}\nREV:{:?}\nEND:VCARD",
        name, telephone, date
    )
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
