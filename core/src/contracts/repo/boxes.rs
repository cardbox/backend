#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::app::{CardSaveError, CardUnsaveError};
use crate::models;
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait BoxRepo {
    async fn box_get_by_id(&self, id: Uuid) -> RepoResult<Option<models::Box>>;

    async fn box_get_user_default(&self, user_id: Uuid) -> RepoResult<models::Box>;

    async fn box_add_card(
        &self,
        box_id: Uuid,
        card_id: Uuid,
    ) -> Result<models::Card, CardSaveError>;

    async fn box_remove_card(
        &self,
        box_id: Uuid,
        card_id: Uuid,
    ) -> Result<models::Card, CardUnsaveError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl BoxRepo for crate::contracts::MockDb {
    async fn box_get_by_id(&self, id: Uuid) -> RepoResult<Option<models::Box>> {
        self.boxes.box_get_by_id(id).await
    }

    async fn box_get_user_default(&self, user_id: Uuid) -> RepoResult<models::Box> {
        self.boxes.box_get_user_default(user_id).await
    }

    async fn box_add_card(
        &self,
        box_id: Uuid,
        card_id: Uuid,
    ) -> Result<models::Card, CardSaveError> {
        self.boxes.box_add_card(box_id, card_id).await
    }

    async fn box_remove_card(
        &self,
        box_id: Uuid,
        card_id: Uuid,
    ) -> Result<models::Card, CardUnsaveError> {
        self.boxes.box_remove_card(box_id, card_id).await
    }
}
