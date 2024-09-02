CREATE TABLE exchanges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chain TEXT NOT NULL,
    factory_address TEXT,
    exchange_name TEXT NOT NULL,
    exchange_type TEXT NOT NULL
);