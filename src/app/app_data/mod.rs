use std::sync::{Mutex, Arc};

use super::services::google::service::GoogleService;


#[derive(Clone)]
pub struct AppData {
  pub google_service: Arc<Mutex<GoogleService>>,
  pub redis_connection: Arc<Mutex<redis::Connection>>,
}