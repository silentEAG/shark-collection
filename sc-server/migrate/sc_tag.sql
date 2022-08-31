DROP TABLE IF EXISTS "public"."sc_tag";
CREATE TABLE "public"."sc_tag" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR(255) COLLATE "pg_catalog"."default",
  "num" INTEGER NOT NULL,
  UNIQUE(name)
)
;