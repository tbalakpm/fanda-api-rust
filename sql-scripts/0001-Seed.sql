INSERT INTO roles (id, role_name, is_enabled) VALUES
(1, 'Administrators', true),
(2, 'Managers', true),
(3, 'Staff', true);

INSERT INTO users (id, login_name, email_address, mobile_number, first_name, last_name, password_hash, password_salt, role_id, is_enabled)
VALUES
(1, 'admin', 'admin@fandatech.net', '9999999999', 'System', 'Administrator', '123:hash', '123:salt', 1, true);

INSERT INTO ledger_groups (id, code, group_name, group_type, parent_id) VALUES
(100, 'AUSPIC', 'AUSPICIOUS ACCOUNTS', 1, NULL),
(200, 'CAPITAL', 'CAPITAL ACCOUNTS', 1, NULL),
(300, 'CURRLIA', 'CURRENT LIABILITIES', 2, NULL),
(400, 'LOANS', 'LOANS (LIABILITY)', 2, NULL),
(500, 'FIXAS', 'FIXED ASSETS', 1, NULL),
(600, 'INVEST', 'INVESTMENTS (ASSET)', 1, NULL),
(700, 'CURRAS', 'CURRENT ASSETS', 1, NULL),
(800, 'REVACC', 'REVENUE ACCOUNTS', 3, NULL),

(310, 'TAXES', 'Duties and Taxes', 2, 300),
(320, 'SUNCRS', 'Sundry Creditors', 2, 300),

(710, 'BANKS', 'Bank Accounts', 1, 700),
(720, 'CASH', 'Cash-in-hand', 1, 700),
(730, 'DEPOSIT', 'Deposits', 1, 700),
(740, 'ADVANCE', 'Advances', 1, 700),
(750, 'STOCK', 'Stock-in-hand', 1, 700),
(760, 'SUNDRS', 'Sundry Debtors', 1, 700),

(810, 'PURCHASE', 'Purchase Accounts', 3, 800),
(820, 'SALES', 'Sales Accounts', 3, 800),
(830, 'INCOME', 'Income Accounts', 4, 800),
(840, 'EXPENSE', 'Expenses Accounts', 5, 800);


/*('00000000-0000-0000-0000-000000000001', 'MISCEXP', 'MISCELLANEOUS EXPENSES (ASSET)', NULL, 1, NULL),
('00000000-0000-0000-0000-000000000001', 'BR/DIV', 'BRANCHES/DIVISIONS', NULL, 1, NULL),
('00000000-0000-0000-0000-000000000001', 'PRO/LOS', 'PROFIT & LOSS', NULL, 1, NULL),
('00000000-0000-0000-0000-000000000001', 'SUSPACC', 'SUSPENSE ACCOUNTS', NULL, 1, NULL),*/