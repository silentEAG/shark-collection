use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::str;

use crate::model::request::NewItem;
use crate::types::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i32>,
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub catalog: String,
}

impl From<NewItem> for Item {
    fn from(frm: NewItem) -> Self {
        Self {
            id: None,
            url: frm.url,
            title: frm.title,
            tags: frm.tags,
            catalog: frm.catalog,
        }
    }
}

impl Item {
    pub async fn find_id_by_url(&self, pool: &Pool<Postgres>) -> i32 {
        let res = sqlx::query!(
            r#"
                SELECT id FROM sc_item WHERE url = $1 LIMIT 1
            "#,
            self.url
        )
        .fetch_one(pool)
        .await;
        match res {
            Ok(row) => row.id,
            Err(_) => 0,
        }
    }

    pub async fn update_tag_id_list(
        &self,
        tag_id_list: &[i32],
        pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE sc_item
                SET tags_id = $1
                WHERE url = $2
            "#,
            tag_id_list,
            self.url
        )
        .execute(&mut *pool)
        .await?;
        Ok(())
    }

    /// Insert to `sc_item`
    pub async fn insert(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<i32> {
        // Insert to item table, `tags_id` is not update yet
        let item = sqlx::query!(
            r#"
                INSERT INTO sc_item
                (url, title, tags, catalog, tags_num)
                VALUES ($1, $2, $3, $4, $5) RETURNING id
            "#,
            self.url,
            self.title,
            &self.tags,
            self.catalog,
            self.tags.len() as i32
        )
        .fetch_one(&mut *pool)
        .await?;
        Ok(item.id)
    }
}
