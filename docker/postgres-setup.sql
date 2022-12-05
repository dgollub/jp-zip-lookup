-- This file have to be run as super user on database creation
-- Runing this file without being a super user will only work in the case the extensions exist

CREATE DATABASE "jpzip";

CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

