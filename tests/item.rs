use sqlx::{Acquire, PgPool};
use sqlx_play::db::model::prelude::*;

#[sqlx::test(fixtures("item_types"))]
async fn insert_item_test(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    let mut tx = conn.begin().await?;

    let c = Cart::default().insert(&mut tx).await?;
    println!("Cart: {c:?}");

    let i = Item::new(c.id, "beer".to_string()).insert(&mut tx).await?;
    println!("Item: {i:?}");
    tx.commit().await?;

    Ok(())
}
