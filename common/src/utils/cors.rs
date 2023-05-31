use actix_cors::Cors;

use actix_web::http::header;

pub fn dev_cors() -> Cors {
    let c = Cors::permissive();
    return c;
}
