-- This file have to be run as super user on database creation
-- Runing this file without being a super user will only work in the case the extensions exist

CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

CREATE TABLE IF NOT EXISTS "addresses" (
    "id" SERIAL primary key,
    "gov_code" VARCHAR(5) NOT NULL,
    "zip_code_old" VARCHAR(5) NOT NULL,
    "zip_code" VARCHAR(7) NOT NULL,
    "prefecture_kana" VARCHAR(50) NOT NULL,
    "muncipalities_kana" VARCHAR(50) NOT NULL,
    "town_kana" VARCHAR(100) NOT NULL,
    "prefecture_kanji" VARCHAR(50) NOT NULL,
    "muncipalities_kanji" VARCHAR(50) NOT NULL,
    "town_kanji" VARCHAR(50) NOT NULL,
    "two_or_more_zip" BOOLEAN NOT NULL DEFAULT FALSE,
    "address_numbered" BOOLEAN NOT NULL DEFAULT FALSE,
    "with_chome" BOOLEAN NOT NULL DEFAULT FALSE,
    "two_or_more_area" BOOLEAN NOT NULL DEFAULT FALSE,
    "updated" SMALLINT NOT NULL DEFAULT 0,
    "change_reason" SMALLINT NOT NULL DEFAULT 0
);

CREATE INDEX "zip_code_idx" ON "addresses" ("zip_code");
CREATE INDEX "prefecture_kanji_idx" ON "addresses" ("prefecture_kanji");
-- More indexes could be added if needed.
