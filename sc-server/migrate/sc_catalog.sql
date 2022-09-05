DROP TABLE IF EXISTS "public"."sc_catalog";
CREATE TABLE "public"."sc_catalog" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL COLLATE "pg_catalog"."default",
  "num" INTEGER NOT NULL,
  "create_time" TIMESTAMP NOT NULL DEFAULT TIMEZONE('UTC-8'::TEXT, NOW()::TIMESTAMP(0) WITHOUT TIME ZONE)
)
;
