DROP TABLE IF EXISTS "public"."sc_tag";
CREATE TABLE "public"."sc_tag" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR(255) UNIQUE,
  "num" INTEGER NOT NULL
)
;