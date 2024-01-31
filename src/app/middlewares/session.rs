use std::{future::Future, pin::Pin, rc::Rc};

use actix_utils::future::{ready, Ready};
use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage,
};

use crate::{
    app::services::{session::service::SessionService, traits::session_storage::SessionStorage},
    config::session_config::SessionConfig,
};

#[derive(Clone)]
pub struct SessionMiddleware<Storage: SessionStorage> {
    storage: Rc<Storage>,
    configuration: Rc<SessionConfig>,
}

impl<Storage: SessionStorage> SessionMiddleware<Storage> {
    pub fn new(storage: Storage, configuration: SessionConfig) -> Self {
        Self {
            storage: Rc::new(storage),
            configuration: Rc::new(configuration),
        }
    }
}

impl<S, B, Storage> Transform<S, ServiceRequest> for SessionMiddleware<Storage>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    Storage: SessionStorage + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = InnerSessionMiddleware<S, Storage>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerSessionMiddleware {
            service: Rc::new(service),
            configuration: Rc::clone(&self.configuration),
            storage: Rc::clone(&self.storage),
        }))
    }
}

pub struct InnerSessionMiddleware<S, Storage: SessionStorage + 'static> {
    service: Rc<S>,
    configuration: Rc<SessionConfig>,
    storage: Rc<Storage>,
}

impl<S, B, Storage> Service<ServiceRequest> for InnerSessionMiddleware<S, Storage>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    Storage: SessionStorage + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let storage = Rc::clone(&self.storage);
        let configuration = Rc::clone(&self.configuration);

        Box::pin(async move {
            // TODO: update session middleware logic to keep anonymous user sessions on every endpoint
            if let Some(session_key) =
                SessionService::get_cookie_session_id(&configuration.cookie_config, &req)
            {
                log::debug!("Session Key: {}", session_key);
                if let Some(session) = match storage.as_ref().load(&session_key) {
                    Ok(try_session) => try_session,
                    Err(err) => {
                        log::error!(
                            "Unable to get user session from cache storage, session key: {}, error: {:?}",
                            &session_key,
                            err
                        );
                        None
                    }
                } {
                    req.extensions_mut().insert(session);
                };
            };

            let res = service.call(req).await?;

            Ok(res)
        })
    }
}
