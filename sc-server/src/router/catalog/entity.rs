use crate::types::Result;
use crate::{common::err_message, router::item::entity::Item};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub id: Option<i32>,
    pub name: String,
    pub num: Option<isize>,
}

impl From<Item> for Catalog {
    fn from(item: Item) -> Self {
        Self {
            id: None,
            name: item.catalog,
            num: None,
        }
    }
}

impl Catalog {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            num: None,
        }
    }

    pub async fn is_existd(&self, pool: &Pool<Postgres>) -> Result<i32> {
        let res = sqlx::query!(
            r#"
                SELECT id FROM sc_catalog WHERE name = $1 LIMIT 1
            "#,
            self.name
        )
        .fetch_one(pool)
        .await;
        match res {
            Ok(r) => Ok(r.id),
            Err(_) => Err(err_message("Catalog is not existed.")),
        }
    }

    /// Update num
    pub async fn update(
        &self,
        pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> crate::types::Result<()> {
        // TODO: id unwrap handle
        sqlx::query!(
            r#"
                UPDATE sc_catalog
                SET num = num + 1
                WHERE id = $1
            "#,
            self.id.unwrap()
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
