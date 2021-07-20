#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate async_trait;

mod accesso_authorize;
mod cards;
mod configure;
mod cookie;
mod health;
mod session;

pub use configure::{configure, install_logger, not_found};
pub use cookie::SessionCookieConfig;
pub(crate) use health::health_service;

use hashbrown::HashMap;
use std::any::{Any, TypeId};
use std::hash::{BuildHasherDefault, Hasher};

use std::ops::Deref;
use std::sync::Arc;

type ServiceMap = HashMap<TypeId, Box<dyn Any + Send + Sync>, BuildHasherDefault<TypeIdHasher>>;

#[derive(Debug, Clone, Default)]
struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }
}

#[derive(Debug)]
pub struct Service<T: ?Sized>(Arc<T>);

impl<T> Service<T> {
    pub fn new(state: T) -> Service<T> {
        Service(Arc::new(state))
    }

    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Service<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Service<T> {
    fn clone(&self) -> Service<T> {
        Service(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for Service<T> {
    fn from(arc: Arc<T>) -> Self {
        Service(arc)
    }
}

#[derive(Debug, Default)]
pub struct AppBuilder {
    services: ServiceMap,
}

#[derive(Debug, Default)]
pub struct App {
    services: ServiceMap,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            services: ServiceMap::with_hasher(BuildHasherDefault::default()),
        }
    }

    pub fn with_service<T: 'static + Send + Sync>(mut self, service: T) -> Self {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
        self
    }

    pub fn build(self) -> App {
        App {
            services: self.services,
        }
    }
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn get<T: 'static>(&self) -> Result<&T, eyre::Report> {
        let service_name = std::any::type_name::<T>();
        let service = self
            .services
            .get(&TypeId::of::<T>())
            .and_then(|rc| (&*rc).downcast_ref());

        service.ok_or_else(|| eyre::eyre!("Could not get service with name: {}", service_name))
    }
}
