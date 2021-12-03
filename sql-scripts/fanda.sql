CREATE TABLE "roles" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "role_name" varchar UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL DEFAULT true
);

CREATE TABLE "users" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "login_name" varchar(16) UNIQUE NOT NULL,
  "email_address" varchar(100) UNIQUE NOT NULL,
  "mobile_number" varchar(10) UNIQUE NOT NULL,
  "first_name" varchar(25),
  "last_name" varchar(25),
  "password_hash" text NOT NULL,
  "password_salt" text NOT NULL,
  "role_id" int NOT NULL,
  "is_email_confirmed" boolean,
  "confirm_email_code" varchar,
  "reset_password_code" varchar,
  "login_at" timestamp,
  "is_enabled" boolean NOT NULL DEFAULT true,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp
);

CREATE TABLE "addresses" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "attention" varchar(25),
  "addr_line1" varchar(50),
  "addr_line2" varchar(50),
  "city" varchar(25),
  "addr_state" varchar(25),
  "country" varchar(25),
  "postal_code" varchar(10),
  "phone" varchar(25),
  "fax" varchar(25)
);

CREATE TABLE "contacts" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "salutation" varchar(5),
  "first_name" varchar(25),
  "last_name" varchar(25),
  "designation" varchar(25),
  "department" varchar(25),
  "email_address" varchar(100),
  "mobile_number" varchar(25),
  "work_phone" varchar(25)
);

CREATE TABLE "ledger_groups" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "group_name" varchar(25) UNIQUE NOT NULL,
  "group_desc" varchar(255),
  "group_type" int NOT NULL,
  "parent_id" int
);

CREATE TABLE "organizations" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "org_name" varchar(50) UNIQUE NOT NULL,
  "org_desc" varchar(255),
  "regd_num" varchar(25),
  "org_pan" varchar(25),
  "org_tan" varchar(25),
  "gstin" varchar(25),
  "address_id" int,
  "contact_id" int,
  "active_year_id" int,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp
);

CREATE TABLE "ledgers" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "ledger_name" varchar(25) UNIQUE NOT NULL,
  "ledger_desc" varchar(255),
  "group_id" int NOT NULL,
  "ledger_type" int NOT NULL,
  "is_system" boolean NOT NULL,
  "org_id" int UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "banks" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "ledger_id" int NOT NULL,
  "account_number" varchar(25) UNIQUE NOT NULL,
  "account_type" int,
  "ifsc_code" varchar(16) NOT NULL,
  "micr_code" varchar(16),
  "branch_code" varchar(16),
  "branch_name" varchar(25),
  "address_id" int,
  "contact_id" int,
  "is_default" boolean NOT NULL
);

CREATE TABLE "parties" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "ledger_id" int NOT NULL,
  "regd_num" varchar(25),
  "party_pan" varchar(25),
  "party_tan" varchar(25),
  "gstin" varchar(25),
  "payment_term" int NOT NULL,
  "credit_limit" numeric(12,2) NOT NULL,
  "address_id" int,
  "contact_id" int
);

CREATE TABLE "units" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "unit_name" varchar(25) UNIQUE NOT NULL,
  "unit_desc" varchar(255),
  "org_id" int UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "product_categories" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "category_name" varchar(25) UNIQUE NOT NULL,
  "category_desc" varchar(255),
  "parent_id" int,
  "org_id" int UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "products" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "code" varchar(16) UNIQUE NOT NULL,
  "product_name" varchar(25) UNIQUE NOT NULL,
  "product_desc" varchar(255),
  "product_type" int NOT NULL,
  "category_id" int NOT NULL,
  "unit_id" int NOT NULL,
  "cost_price" numeric(12,2) NOT NULL,
  "selling_price" numeric(12,2) NOT NULL,
  "tax_code" varchar(16) NOT NULL,
  "tax_preference" int NOT NULL,
  "tax_pct" numeric(5,2) NOT NULL,
  "org_id" int UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "account_years" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "year_code" varchar(16) UNIQUE NOT NULL,
  "year_begin" date UNIQUE NOT NULL,
  "year_end" date NOT NULL,
  "org_id" int UNIQUE NOT NULL,
  "is_enabled" boolean NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "ledger_balances" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "ledger_id" int UNIQUE NOT NULL,
  "year_id" int UNIQUE NOT NULL,
  "opening_balance" numeric(12,2) NOT NULL,
  "balance_sign" char(1) NOT NULL
);

CREATE TABLE "serial_numbers" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "year_id" int UNIQUE NOT NULL,
  "serial_module" int UNIQUE NOT NULL,
  "serial_prefix" varchar(5),
  "serial_format" varchar(16) NOT NULL,
  "serial_suffix" varchar(5),
  "last_serial" varchar(16),
  "last_number" int NOT NULL,
  "last_date" timestamp NOT NULL,
  "serial_reset" int NOT NULL
);

CREATE TABLE "journals" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "journal_number" varchar(16) UNIQUE NOT NULL,
  "journal_date" timestamp NOT NULL,
  "journal_type" int NOT NULL,
  "journal_sign" char(1) NOT NULL,
  "ledger_id" int NOT NULL,
  "year_id" int UNIQUE NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "journal_items" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "journal_id" int NOT NULL,
  "ledger_id" int,
  "quantity" numeric(7,3) NOT NULL,
  "amount" numeric(12,2) NOT NULL,
  "journal_desc" varchar(255),
  "ref_num" varchar(16),
  "ref_date" date
);

CREATE TABLE "consumers" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "contact_id" int,
  "address_id" int
);

CREATE TABLE "invoices" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "invoice_number" varchar(16) UNIQUE NOT NULL,
  "invoice_date" timestamp NOT NULL,
  "invoice_type" int NOT NULL,
  "stock_invoice_type" int NOT NULL,
  "gst_treatment" int NOT NULL,
  "tax_preference" int NOT NULL,
  "notes" varchar(255),
  "party_id" int NOT NULL,
  "ref_num" varchar(16),
  "ref_date" date,
  "consumer_id" int,
  "subtotal" numeric(12,2) NOT NULL,
  "total_qty" numeric(10,3) NOT NULL,
  "discount_pct" numeric(5,2) NOT NULL,
  "discount_amt" numeric(9,2) NOT NULL,
  "total_tax_amt" numeric(9,2) NOT NULL,
  "misc_add_desc" varchar(25),
  "misc_add_amt" numeric(9,2) NOT NULL,
  "net_amount" numeric(12,2) NOT NULL,
  "year_id" int UNIQUE NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp
);

CREATE TABLE "inventory" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "product_id" int NOT NULL,
  "tag_number" varchar(16) UNIQUE NOT NULL,
  "mfg_date" date,
  "expire_date" date,
  "unit_id" int NOT NULL,
  "qty_on_hand" numeric(10,3) NOT NULL
);

CREATE TABLE "invoice_items" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "invoice_id" int NOT NULL,
  "inventory_id" int,
  "item_desc" varchar(255) NOT NULL,
  "unit_id" int NOT NULL,
  "qty" numeric(9,3) NOT NULL,
  "unit_price" numeric(9,2) NOT NULL,
  "price" numeric(9,2) NOT NULL,
  "discount_pct" numeric(5,2) NOT NULL,
  "discount_amt" numeric(9,2) NOT NULL,
  "tax_pct" numeric(5,2) NOT NULL,
  "tax_amt" numeric(9,2) NOT NULL,
  "line_total" numeric(9,2) NOT NULL
);

CREATE TABLE "transactions" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "debit_ledger_id" int NOT NULL,
  "credit_ledger_id" int NOT NULL,
  "quantity" numeric(9,3) NOT NULL,
  "amount" numeric(12,2) NOT NULL,
  "tran_desc" varchar(255),
  "ref_num" varchar(16),
  "ref_date" date,
  "journal_id" int,
  "invoice_id" int
);

ALTER TABLE "users" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "ledger_groups" ADD FOREIGN KEY ("parent_id") REFERENCES "ledger_groups" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "organizations" ADD FOREIGN KEY ("address_id") REFERENCES "addresses" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "organizations" ADD FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "ledgers" ADD FOREIGN KEY ("group_id") REFERENCES "ledger_groups" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "ledgers" ADD FOREIGN KEY ("org_id") REFERENCES "organizations" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "banks" ADD FOREIGN KEY ("ledger_id") REFERENCES "ledgers" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "banks" ADD FOREIGN KEY ("address_id") REFERENCES "addresses" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "banks" ADD FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "parties" ADD FOREIGN KEY ("ledger_id") REFERENCES "ledgers" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "parties" ADD FOREIGN KEY ("address_id") REFERENCES "addresses" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "parties" ADD FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "units" ADD FOREIGN KEY ("org_id") REFERENCES "organizations" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "product_categories" ADD FOREIGN KEY ("parent_id") REFERENCES "product_categories" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "product_categories" ADD FOREIGN KEY ("org_id") REFERENCES "organizations" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "products" ADD FOREIGN KEY ("category_id") REFERENCES "product_categories" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "products" ADD FOREIGN KEY ("unit_id") REFERENCES "units" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "products" ADD FOREIGN KEY ("org_id") REFERENCES "organizations" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "account_years" ADD FOREIGN KEY ("org_id") REFERENCES "organizations" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "ledger_balances" ADD FOREIGN KEY ("year_id") REFERENCES "account_years" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "serial_numbers" ADD FOREIGN KEY ("year_id") REFERENCES "account_years" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "journals" ADD FOREIGN KEY ("ledger_id") REFERENCES "ledgers" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "journals" ADD FOREIGN KEY ("year_id") REFERENCES "account_years" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "journal_items" ADD FOREIGN KEY ("journal_id") REFERENCES "journals" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "journal_items" ADD FOREIGN KEY ("ledger_id") REFERENCES "ledgers" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "consumers" ADD FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "consumers" ADD FOREIGN KEY ("address_id") REFERENCES "addresses" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "invoices" ADD FOREIGN KEY ("party_id") REFERENCES "ledgers" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "invoices" ADD FOREIGN KEY ("consumer_id") REFERENCES "consumers" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "invoices" ADD FOREIGN KEY ("year_id") REFERENCES "account_years" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "inventory" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "inventory" ADD FOREIGN KEY ("unit_id") REFERENCES "units" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "invoice_items" ADD FOREIGN KEY ("invoice_id") REFERENCES "invoices" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "invoice_items" ADD FOREIGN KEY ("inventory_id") REFERENCES "inventory" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "invoice_items" ADD FOREIGN KEY ("unit_id") REFERENCES "units" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "transactions" ADD FOREIGN KEY ("debit_ledger_id") REFERENCES "ledgers" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "transactions" ADD FOREIGN KEY ("credit_ledger_id") REFERENCES "ledgers" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE "transactions" ADD FOREIGN KEY ("journal_id") REFERENCES "journals" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;

ALTER TABLE "transactions" ADD FOREIGN KEY ("invoice_id") REFERENCES "invoices" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;
