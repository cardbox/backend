use crate::Service;
use cardbox_core::contracts::Repository;
use std::sync::Arc;

pub fn mock_app<R: Repository + 'static>(db: R) -> crate::App {
    let db: Arc<dyn Repository> = Arc::new(db);
    let db: Service<dyn Repository> = Service::from(db);

    crate::App::builder().with_service(db).build()
}
