mod transaction;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use tinytemplate::TinyTemplate;
use serde::{Serialize, Deserialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use transaction::Image;

const RPC_URL: &str = "https://api.devnet.solana.com";

static INDEX_HTML: &'static str = include_str!("../content/index.html");
static CSS: &'static str = include_str!("../content/output.css");
static INDEX_JS: &'static str = include_str!("../content/index.js");
static FIND_IMAGE_HTML: &'static str = include_str!("../content/find-image.html");
static ADD_IMAGE_HTML: &'static str = include_str!("../content/add-image.html");
static WALLET_JS: &'static str = include_str!("../content/wallet.js");

#[derive(Deserialize)]
struct FindImageInfo {
    address: String,
    image_title: String
}

#[derive(Serialize)]
struct ImageInfo {
    image_title: String,
    image_url: String
}

#[derive(Deserialize, Debug)]
struct DataAndPDAInfo {
    signer: String,
    image: Image
}

#[derive(Serialize)]
struct DataAndPDAResponse {
    data: Vec<u8>,
    pda: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(style)
            .service(script)
            .service(find_image)
            .service(add_image)
            .service(walletjs)
            .service(get_data_and_pda)
    })
    .bind(("0.0.0.0", 8080))?
    .run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .body(INDEX_HTML)
}

#[get("/style.css")]
async fn style() -> impl Responder {
    HttpResponse::Ok()
        .body(CSS)
}

#[get("/index.js")]
async fn script() -> impl Responder {
    HttpResponse::Ok()
        .body(INDEX_JS)
}

#[get("/wallet.js")]
async fn walletjs() -> impl Responder {
    HttpResponse::Ok()
        .body(WALLET_JS)
}

#[get("/add-image")]
async fn add_image() -> impl Responder {
    HttpResponse::Ok()
        .body(ADD_IMAGE_HTML)
}

#[get("/find-image")]
async fn find_image(query: web::Query<FindImageInfo>) -> impl Responder {
    let rpc_client = RpcClient::new(String::from(RPC_URL));
    let pubkey = Pubkey::try_from(query.address.as_str()).unwrap();
    let program_id = Pubkey::try_from(transaction::PROGRAM_ID).unwrap();

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[pubkey.as_ref(), query.image_title.as_bytes()],
        &program_id
    );

    let image: Image = bincode::deserialize(
        &rpc_client.get_account_data(&pda).await.unwrap()
    ).unwrap();

    let image_info = ImageInfo {
        image_title: query.image_title.clone(),
        image_url: image.url
    };

    let mut template = TinyTemplate::new();
    template.add_template("find-image", FIND_IMAGE_HTML).unwrap();

    let response_body = template.render("find-image", &image_info).unwrap();

    HttpResponse::Ok()
        .body(response_body)
}

#[post("/get-data-and-pda")]
async fn get_data_and_pda(info: web::Json<DataAndPDAInfo>) -> impl Responder {
    let signer = Pubkey::try_from(info.signer.as_str()).unwrap();

    let data = transaction::get_data(&info.image);
    let pda = transaction::get_pda(&signer, &info.image);

    let response_body = DataAndPDAResponse {
        data,
        pda
    };

    HttpResponse::Ok()
        .json(response_body)
}
