use std::{future::Future, pin::Pin, rc::Rc};

use actix_utils::future::{ready, Ready};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpResponse,
};

use crate::{
    app::{
        common::api_path::{LOGIN, LOGOUT, REGISTER},
        handlers::common::response::USER_SHOULD_RELOGIN,
        models::session::Session,
        services::{
            common::error_as_json, session::service::SessionService,
            traits::session_storage::SessionStorage,
        },
    },
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
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let storage = Rc::clone(&self.storage);
        let configuration = Rc::clone(&self.configuration);

        Box::pin(async move {
            let session: Session;
            let (request_ref, _) = req.parts();
            if let Some(session_key) =
                SessionService::get_cookie_session_id(&configuration.cookie_config, &req)
            {
                log::debug!("Session Key: {}", session_key);
                session = match storage.as_ref().load(&session_key) {
                    Ok(try_session) => match try_session {
                        Some(s) => s,
                        None => {
                            log::debug!("No session in Cache Storage by Session Key {}. Set Anonymous Session", &session_key);
                            Session::get_anonymous_session(Some(request_ref))
                        }
                    },
                    Err(err) => {
                        log::error!(
                            "Unable to get user session from cache storage by Session Key: {}, error: {:?}",
                            &session_key,
                            err
                        );
                        // it didn't get Session from CacheStorage by Session Key
                        // set an anonymous Session
                        Session::get_anonymous_session(Some(request_ref))
                    }
                }
            } else {
                log::debug!("No Session Key on request. Set Anonymous Session");
                session = Session::get_anonymous_session(Some(request_ref));
            };

            // To do not create a new not anonymous session when request has it on LOGIN or REGISTER endpoints
            if !session.is_anonymous()
                && (req.path().contains(LOGIN) || req.path().contains(REGISTER))
            {
                let err_response =
                    HttpResponse::BadRequest().json(error_as_json(USER_SHOULD_RELOGIN));
                let (http_req, _) = req.into_parts();
                let service_err_response = ServiceResponse::new(http_req, err_response);
                return Ok(service_err_response.map_into_right_body());
            }

            req.extensions_mut().insert(session.clone());

            // Need to exclude only once the case when user /logout
            let is_restricted_endpoint_call =
                req.path().contains(LOGOUT) && !session.is_anonymous();

            let mut res = service.call(req).await?;
            //
            // Everything after the call
            //
            let m_res = res.response_mut();
            let session_cookie = m_res
                .cookies()
                .into_iter()
                .find(|cookie| cookie.name() == &configuration.cookie_config.name);
            if session_cookie.is_none() && !is_restricted_endpoint_call {
                if let Err(err) = storage
                    .as_ref()
                    .set(&session, configuration.session_ttl_sec)
                {
                    log::error!(
                        "Error to set session in Session Storage on Session Middleware: {:?}",
                        err
                    );
                }
                if let Err(err) = SessionService::set_cookie_session_id(
                    &configuration.cookie_config,
                    m_res.head_mut(),
                    session.id.to_string(),
                ) {
                    log::error!("Error to set session cookie to response: {:?}", err);
                }
            }
            Ok(res.map_into_left_body())
        })
    }
}
