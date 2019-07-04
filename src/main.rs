use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;
use structopt::StructOpt;

#[derive(Serialize)]
struct Credentials {
    #[serde(rename = "authType")]
    auth_type: String,
    source: String,
    key: String,
}

#[derive(Serialize)]
struct ProvisionResult {
    #[serde(rename = "deviceId")]
    device_id: String,
    #[serde(rename = "hubName")]
    hub: String,
    credentials: Credentials,
}

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "edge-external-provisioner")]
struct Opt {
    #[structopt(short = "h", long = "hub-name")]
    hub: String,

    #[structopt(short = "d", long = "device-id")]
    device_id: String,

    #[structopt(short = "k", long = "key")]
    key: String,
}

fn provision(data: web::Data<Opt>, req: HttpRequest) -> impl Responder {
    println!(
        "Handling provision request from {:?}",
        req.connection_info()
    );

    let response = ProvisionResult {
        device_id: data.device_id.clone(),
        hub: data.hub.clone(),
        credentials: Credentials {
            auth_type: "symmetric-key".to_owned(),
            source: "payload".to_owned(),
            key: data.key.clone(),
        },
    };

    web::Json(response)
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(opt.clone()))
            .service(web::resource("/device/provisioninginformation").to(provision))
    })
    .bind("127.0.0.1:9999")?
    .run()
}
