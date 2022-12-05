# NOTE(dkg): Do not use just "postgres:14", it will fail to create/read the /docker-entrypoint-initdb.d folder!
# 			 You MUST specify "postgresql:14-bullseye" for this to work!
FROM postgres:14-bullseye

COPY ./postgres-setup.sql /docker-entrypoint-initdb.d/
