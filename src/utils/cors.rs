use std::io::Cursor;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Method, Status},
    Response,
};

pub struct CorsFairing;

// impl<'r> Responder<'r, 'static> for CorsFairing {
//     fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
//         Response::build()
//             .raw_header("Access-Control-Allow-Origin", "*")
//             .raw_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS, PUT")
//             .raw_header("Access-Control-Allow-Headers", "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range")
//             .raw_header("Access-Control-Max-Age", "1728000")
//             .header(ContentType::Plain)
//             .status(Status::NoContent).ok()
//     }
// }

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Cors Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut Response<'r>) {
        // Don't change a successful user's response, ever.
        if res.status() != Status::NotFound {
            return;
        }

        if req.method() == Method::Options {
            res.set_status(Status::NoContent);
            res.set_header(ContentType::Plain);
            res.set_raw_header("Access-Control-Allow-Origin", "*");
            res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS, PUT");
            res.set_raw_header(
            "Access-Control-Allow-Headers",
            "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization",
        );
            res.set_raw_header("Access-Control-Max-Age", "1728000");
            res.set_sized_body(0, Cursor::new(""))
        }
    }
}
