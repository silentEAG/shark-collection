DROP TABLE IF EXISTS "public"."sc_item";
CREATE TABLE "public"."sc_item" (
  "id" SERIAL PRIMARY KEY,
  "url" VARCHAR(255) COLLATE "pg_catalog"."default",
  "title" VARCHAR(255) COLLATE "pg_catalog"."default",
  "tags" VARCHAR(255)[] COLLATE "pg_catalog"."default",
  "tags_num" INTEGER DEFAULT 0,
  "tags_id" INTEGER[],
  "catalog" VARCHAR(255) COLLATE "pg_catalog"."default"
)
;