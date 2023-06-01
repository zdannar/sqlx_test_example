use sqlx::types::{time::PrimitiveDateTime, Uuid};

#[derive(Debug, sqlx::FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

impl Default for Cart {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: None,
            updated_at: None,
        }
    }
}

impl Cart {
    pub async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Cart, sqlx::Error> {
        sqlx::query_as("INSERT INTO cart (id) VALUES ($1) RETURNING *")
            .bind(self.id)
            .fetch_one(tx)
            .await
    }
}

#[cfg(test)]
mod test {}
