use crate::types::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub num: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagMap {
    pub id: i32,
    pub name: String,
    pub num: i32,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name, num: None }
    }

    /// Upsert tag table and return `id`
    pub async fn upsert(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<isize> {
        let res = sqlx::query!(
            r#"
                SELECT id FROM sc_tag WHERE name = $1
            "#,
            self.name
        )
        .fetch_one(&mut *pool)
        .await;

        let id = match res {
            Ok(r) => {
                sqlx::query!(
                    r#"
                        UPDATE sc_tag
                        SET num = num + 1
                        WHERE id = $1
                    "#,
                    r.id
                )
                .execute(&mut *pool)
                .await?;
                r.id
            }
            Err(_) => {
                let r = sqlx::query!(
                    r#"
                        INSERT INTO sc_tag(name, num)
                        VALUES($1, 1) RETURNING id
                    "#,
                    self.name
                )
                .fetch_one(&mut *pool)
                .await?;
                r.id
            }
        };
        Ok(id as isize)
    }

    pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<TagMap>> {
        let res = sqlx::query_as!(
            TagMap,
            r#"
                SELECT id, name, num FROM sc_tag ORDER BY sc_tag.num DESC, sc_tag.name ASC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
