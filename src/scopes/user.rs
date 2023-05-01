use actix_web::{ HttpResponse, Scope, web };
use jsonwebtoken::{
    encode,
    decode,
    Algorithm,
    Validation,
    Header,
    EncodingKey,
    DecodingKey,
    TokenData,
    errors::Error as JwtError,
};
use serde::{ Serialize, Deserialize };
use chrono::{ Utc, Duration };
use crate::extractors::authentication_token::{ Claims, AuthenticationToken };

pub fn user_scope() -> Scope {
    web::scope("/user")
	.route("/encode/{id}", web::get().to(encode_token))
	.route("/decode", web::post().to(decode_token))
	.route("/protected", web::get().to(protected_route))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret(secret.as_str().as_ref()),
    ).unwrap();
    HttpResponse::Ok().json(EncodeResponse {
	message: "Successfully created account.".to_owned(),
	token,
    })
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String
}

// Example for when the token is send through the body
async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
    let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
	&body.token,
	&DecodingKey::from_secret(secret.as_str().as_ref()),
	&Validation::new(Algorithm::HS256),
    );

    match token_result {
	Ok(token) => HttpResponse::Ok().json(DecodeResponse {
	    message: "Successfully logged in.".to_owned(),
	    id: token.claims.id,
	}),
	Err(e) => HttpResponse::Unauthorized().json(Response { message: e.to_string() }),
    }
}

// Example on how it probably should be handled
async fn protected_route(auth_token: AuthenticationToken) -> HttpResponse {
    println!("{:#?}", auth_token);
    HttpResponse::Ok().json(Response { message: "Authorized".to_owned() })
}
