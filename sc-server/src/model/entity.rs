use serde::{Deserialize, Serialize};
use std::str;

use super::request::ItemJson;
use crate::common::error;
use crate::types::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScItem {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub catalog: String,
}

impl From<ItemJson> for ScItem {
    fn from(frm: ItemJson) -> Self {
        Self {
            url: frm.url,
            title: frm.title,
            tags: frm.tags,
            catalog: frm.catalog,
        }
    }
}

impl ScItem {
    pub async fn find_id_by_url(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> i32 {
        let res = sqlx::query!(
            r#"
                SELECT id FROM sc_item WHERE url = $1 LIMIT 1
            "#,
            self.url
        )
        .fetch_one(&mut *pool)
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
    pub async fn insert(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<()> {
        // Judge if item exists
        if self.find_id_by_url(pool).await != 0 {
            return error("Item already exists.");
        }

        // Update catalog
        ScCatalog::new(self.catalog.clone()).update(pool).await?;

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

        // 1. Upsert tag in table
        // 2. Collect tag's id for updating item table's `tags_id`
        let mut tag_id_vec = Vec::with_capacity(self.tags.len());
        for tag in &self.tags {
            // Upsert tag table
            let tag_id = ScTag::new(tag.clone()).upsert(pool).await?;
            tag_id_vec.push(tag_id as i32);
            sqlx::query!(
                r#"
                    INSERT INTO sc_tag_map
                    (tag_id, item_id)
                    VALUES ($1, $2)
                "#,
                tag_id as i32,
                item.id as i32
            )
            .execute(&mut *pool)
            .await?;
        }

        self.update_tag_id_list(&tag_id_vec, pool).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScCatalog {
    pub name: String,
    pub num: Option<isize>,
}

impl From<ScItem> for ScCatalog {
    fn from(item: ScItem) -> Self {
        Self {
            name: item.catalog,
            num: None,
        }
    }
}

impl ScCatalog {
    pub fn new(name: String) -> Self {
        Self { name, num: None }
    }

    /// If it not exists, return error
    pub async fn update(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<()> {
        let res = sqlx::query!(
            r#"
                SELECT id FROM sc_catalog WHERE name = $1 LIMIT 1
            "#,
            self.name
        )
        .fetch_one(&mut *pool)
        .await;

        match res {
            Ok(row) => {
                sqlx::query!(
                    r#"
                        UPDATE sc_catalog
                        SET num = num + 1
                        WHERE id = $1
                    "#,
                    row.id
                )
                .execute(pool)
                .await?;
                Ok(())
            }
            Err(_) => error("Catalog is not existed."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScTag {
    pub name: String,
    pub num: Option<isize>,
}

impl ScTag {
    pub fn new(name: String) -> Self {
        Self { name, num: None }
    }

    /// Upsert tag table and return `id`
    pub async fn upsert(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<isize> {
        let res = sqlx::query!(
            r#"
                INSERT INTO sc_tag(name, num)
                VALUES($1, 0)
                ON conflict(name)
                DO UPDATE SET num = sc_tag.num + 1 RETURNING id
            "#,
            self.name
        )
        .fetch_one(&mut *pool)
        .await?;
        Ok(res.id as isize)
    }
}
