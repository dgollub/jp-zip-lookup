-- This file have to be run as super user on database creation
-- Runing this file without being a super user will only work in the case the extensions exist

CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

CREATE TABLE IF NOT EXISTS "addresses" (
    "id" SERIAL primary key,
    "gov_code" VARCHAR(5) NOT NULL,                    --  1. CSV column
    "zip_code_old" VARCHAR(5) NOT NULL,                --  2. CSV column
    "zip_code" VARCHAR(7) NOT NULL,                    --  3. CSV column
    "prefecture_kana" VARCHAR(50) NOT NULL,            --  4. CSV column
    "muncipalities_kana" VARCHAR(50) NOT NULL,         --  5. CSV column
    "town_kana" VARCHAR(100) NOT NULL,                 --  6. CSV column
    "prefecture_kanji" VARCHAR(50) NOT NULL,           --  7. CSV column
    "muncipalities_kanji" VARCHAR(50) NOT NULL,        --  8. CSV column
    "town_kanji" VARCHAR(50) NOT NULL,                 --  9. CSV column
    "two_or_more_zip" BOOLEAN NOT NULL DEFAULT FALSE,  -- 10. CSV column
    "address_numbered" BOOLEAN NOT NULL DEFAULT FALSE, -- 11. CSV column
    "with_chome" BOOLEAN NOT NULL DEFAULT FALSE,       -- 12. CSV column
    "two_or_more_area" BOOLEAN NOT NULL DEFAULT FALSE, -- 13. CSV column
    "updated" SMALLINT NOT NULL DEFAULT 0,             -- 14. CSV column
    "change_reason" SMALLINT NOT NULL DEFAULT 0        -- 15. CSV column
);

CREATE INDEX "zip_code_idx" ON "addresses" ("zip_code");
CREATE INDEX "prefecture_kanji_idx" ON "addresses" ("prefecture_kanji");
-- More indexes could be added if needed.
