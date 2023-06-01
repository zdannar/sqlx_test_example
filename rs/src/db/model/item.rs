use sqlx::types::{time::PrimitiveDateTime, Uuid};

#[derive(Debug, sqlx::FromRow)]
pub struct Item {
    id: Uuid,
    cart_id: Uuid,
    item_type: String,
    created_at: Option<PrimitiveDateTime>,
    updated_at: Option<PrimitiveDateTime>,
}

impl Item {
    pub fn new(cart_id: Uuid, item_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            cart_id,
            item_type,
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Item, sqlx::Error> {
        sqlx::query_as("INSERT INTO item (id, cart_id, item_type) VALUES ($1, $2, $3) RETURNING *")
            .bind(self.id)
            .bind(self.cart_id)
            .bind(&self.item_type)
            .fetch_one(tx)
            .await
    }
}
#[cfg(test)]
mod test {}
