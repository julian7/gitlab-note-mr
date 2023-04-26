use crate::{Settings, settings::Target};
use actix_web::{web, get, post, Responder, HttpResponse, HttpResponseBuilder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use log::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(welcome);
    cfg.service(health);
    cfg.service(note);
}

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("<h1>Welcome</h1>\n")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK\n")
}

#[derive(Deserialize, Serialize)]
pub struct NoteReq {
    pub body: String
}

#[derive(Deserialize)]
pub struct NoteReqPath {
    pub projectid: u32,
    pub mrid: u32,
}

#[post("/note/{projectid}/{mrid}")]
async fn note<'a>(settings: web::Data<Settings>, auth: BasicAuth, path: web::Path<NoteReqPath>, data: web::Json<NoteReq>) -> impl Responder {
    let target = match find_target(&settings.targets, &auth) {
        None => {
            info!("unauthorized access by user {} on project {}/MR {}", auth.user_id(), path.projectid, path.mrid);
            return HttpResponse::Unauthorized();
        },
        Some(target) => target,
    };

    debug!("writing note as user {}, to project {} / MR {}", auth.user_id(), path.projectid, path.mrid);
    let client = Client::new();
    let res = client.post(format!("{}/api/v4/projects/{}/merge_requests/{}/notes", settings.gitlab_url, path.projectid, path.mrid))
        .header("PRIVATE-TOKEN", &target.token)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await;

    match res {
        Ok(res) => res,
        Err(e) => {
            if let Some(url) = e.url() {
                debug!("note API error from {}: {}", url, e);
            } else {
                debug!("note backend error: {}", e);
            }
            return HttpResponseBuilder::new(e.status().unwrap());
        },
    };

    HttpResponse::Created()
}

fn find_target<'a>(targets: &'a Vec<Target>, auth: &'a BasicAuth) -> Option<&'a Target> {
    let user = auth.user_id();
    let pass = auth.password();

    targets.iter().find(|&t| {
        if t.user != user {
            return false;
        }
        match pass {
            Some(data) => t.pass == data,
            None => true,
        }
    })
}
