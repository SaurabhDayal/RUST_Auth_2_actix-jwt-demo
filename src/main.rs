use std::io::Result;
use actix_web::{ web, App, HttpServer };

mod scopes;
mod extractors;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
	App::new()
	    .app_data(web::Data::<String>::new("secret".to_owned()))
	    .service(user_scope())
    }).bind(("127.0.0.1", 8080))?
	.run()
	.await
}

// // // the actix_web main macro will turn the main function to something like this
// fn main() -> Result<()> {
//     // System is a manager for a per-thread distributed async runtime.
//     // new returns a SystemRunner, which keeps a System's event loop alive until stop message is received
//     // block_on runs the provided future, blocking the current thread until the future completes
//     rt::System::new().block_on(
// 	HttpServer::new(|| {
// 	    App::new()
// 		.service(user_scope())
// 	}).bind(("127.0.0.1", 8080))?
// 	    .run()
//     )
// }
