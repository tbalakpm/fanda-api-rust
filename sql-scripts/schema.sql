CREATE TABLE "roles" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "role_name" TEXT UNIQUE NOT NULL,
  "active" BOOLEAN NOT NULL DEFAULT true
);
