DROP TABLE IF EXISTS "public"."sc_tag_map";
CREATE TABLE "public"."sc_tag_map" (
  "id" SERIAL PRIMARY KEY,
  "tag_id" INTEGER NOT NULL,
  "item_id" INTEGER NOT NULL
)
;