use crate::types::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub num: Option<isize>,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name, num: None }
    }

    pub fn new_with_num(name: String, num: isize) -> Self {
        Self {name, num: Some(num)}
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
}
