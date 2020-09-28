#![feature(decl_macro)]

#[macro_use]
extern crate rocket_include_static_resources;

extern crate rocket_raw_response;

#[macro_use]
extern crate rocket;

extern crate rocket_multipart_form_data_async;

use rocket::http::ContentType;
use rocket::Data;

use rocket_include_static_resources::StaticResponse;

use rocket_multipart_form_data_async::mime;
use rocket_multipart_form_data_async::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

use rocket_raw_response::RawResponse;

#[get("/")]
async fn index() -> StaticResponse {
    static_response!("html-image-uploader")
}

#[post("/upload", data = "<data>")]
async fn upload(content_type: &ContentType, data: Data) -> Result<RawResponse, &'static str> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let parse_result = MultipartFormData::parse(content_type, data, options).await;

    let mut multipart_form_data = match parse_result {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.");
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("The file is not an image.");
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            let content_type = raw.content_type;
            let file_name = raw.file_name.unwrap_or("Image".to_string());
            let data = raw.raw;

            Ok(RawResponse::from_vec(data, Some(file_name), content_type))
        }
        None => Err("Please input a file."),
    }
}
#[rocket::main]
async fn main() {
    let res = rocket::ignite()
        // .attach(StaticResponse::fairing(|resources| {
        //     static_resources_initialize!(
        //         resources,
        //         "html-image-uploader",
        //         "examples/front-end/html/image-uploader.html",
        //     );
        // }))
        .mount("/", routes![index])
        .mount("/", routes![upload])
        .launch().await;
}
