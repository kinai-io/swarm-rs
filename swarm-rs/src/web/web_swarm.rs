use rocket::{
    catch, catchers, fairing::AdHoc, http::Status, post, routes, serde::json::Json, Build, Rocket,
    State,
};

use crate::{
    agent::{Action, Output}, logger::Logger, prelude::Swarm
};

use super::{
    auth::{AuthAgent, AuthHeaders}, request_headers::RequestHeaders, spa_services::{self, SPA}
};

pub struct WebSwarm {}

impl WebSwarm {
    pub fn serve(swarm: Swarm) -> Rocket<Build> {
        let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));
        let spa_settings = SPA::default();
        let logger = Logger::new("logs", "webswarm");
        rocket::custom(figment)
            .manage(spa_settings)
            .manage(swarm)
            .manage(logger)
            .mount(
                "/",
                routes![spa_services::app_index, spa_services::app_resources],
            )
            .register("/", catchers![forbidded_catcher])
            .mount("/api", routes![execute_action])
            .attach(AdHoc::on_shutdown("Shutdown Printer", |_| {
                Box::pin(async move {
                    println!("...shutdown has commenced!");
                    // TODO : https://rocket.rs/guide/v0.5/fairings/#callbacks
                })
            }))
    }
}

#[post("/action", data = "<action>")]
pub async fn execute_action(
    auth_headers: AuthHeaders,
    action: Json<Action>,
    agents_swarm: &State<Swarm>,
    headers: RequestHeaders,
    logger: &State<Logger>,
) -> Result<Json<Output>, Status> {
    logger.info("ACTION", &headers);

    let accessible = if let Some(auth_agent) = agents_swarm.get_agent::<AuthAgent>("Auth") {
        let action_id = action.get_id();
        auth_agent.is_accessible(&auth_headers.token, action_id)
    }else {
        true
    };
    if accessible {
        let output = agents_swarm.execute_action(&action).await;
        Ok(Json(output))
    }else {
        Err(Status::Forbidden)
    }
    
}

#[catch(403)]
pub fn forbidded_catcher(req: &rocket::Request) -> String {
    format!("Forbidden access: {}", req.uri())
}
