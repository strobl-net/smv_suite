-- Your SQL goes here
-- TODO: add currency and location TYPE. write own cate?
CREATE TYPE currency AS ENUM ('EUR', 'USD');
CREATE TYPE branch AS ENUM ('DIGITAL', 'CASH');

CREATE TABLE persons (
    id              SERIAL                  ,
    name            TEXT        NOT NULL    ,
    email           TEXT                    ,
    phone           VARCHAR(30)             ,
    tags            TEXT[]      NOT NULL    ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)
);

-- TODO: add location type. write own crate?
CREATE TABLE organisations (
    id              SERIAL                  ,
    name            TEXT        NOT NULL    ,
    description     TEXT                    ,
    site            TEXT                    ,
    location        TEXT                    ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)
);

CREATE TABLE products (
    id              SERIAL                  ,
    name            TEXT                    ,
    description     TEXT                    ,
    change          BIGINT                  ,
    currency        CURRENCY                ,
    provider        INT                     ,
    tags            TEXT[]                  ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)                        ,
    CONSTRAINT fk_provider
        FOREIGN KEY (provider)
            REFERENCES organisations(id)
            ON DELETE SET NULL
);

CREATE TABLE transaction_entities (
    id              SERIAL                  ,
    description     TEXT                    ,
    organisation    INT                     ,
    person          INT                     ,
    iban            VARCHAR(40)             ,
    bic             VARCHAR(15)             ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)                        ,
    CONSTRAINT fk_organisation
        FOREIGN KEY (organisation)
                REFERENCES organisations(id)
                ON DELETE CASCADE           ,
    CONSTRAINT  fk_person
        FOREIGN KEY (person)
                REFERENCES persons(id)
                ON DELETE CASCADE
);

CREATE TABLE money_nodes (
    id              SERIAL                  ,
    branch          BRANCH      NOT NULL    ,
    change          BIGINT      NOT NULL    ,
    currency        CURRENCY    NOT NULL    ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)
);

CREATE TABLE transactions (
    id              SERIAL                  ,
    description     TEXT                    ,
    sender          INT         NOT NULL    ,
    sender_local    BOOLEAN     NOT NULL    ,
    receiver        INT         NOT NULL    ,
    receiver_local  BOOLEAN     NOT NULL    ,
    money_node      INT         NOT NULL    ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)                        ,
    CONSTRAINT fk_sender
        FOREIGN KEY (sender)
                REFERENCES transaction_entities(id)
                ON DELETE SET NULL          ,
    CONSTRAINT  fk_receiver
        FOREIGN KEY (receiver)
                REFERENCES transaction_entities(id)
                ON DELETE SET NULL          ,
    CONSTRAINT fk_money_node
        FOREIGN KEY (money_node)
                REFERENCES money_nodes(id)
                ON DELETE SET NULL
);

CREATE TABLE depodraw (
    id              SERIAL                  ,
    description     TEXT                    ,
    transaction     INT                     ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)                        ,
    CONSTRAINT fk_transaction
        FOREIGN KEY (transaction)
            REFERENCES transactions(id)
            ON DELETE SET NULL
);

CREATE TABLE bills (
    id              SERIAL                  ,
    received        TIMESTAMP   NOT NULL    ,
    processed       TIMESTAMP               ,
    products        INT[]                   ,
    responsible     INT                     ,
    organisation    INT                     ,
    change          BIGINT      NOT NULL    ,
    currency        CURRENCY    NOT NULL    ,
    transaction     INT         NOT NULL    ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)                        ,
    CONSTRAINT  fk_responsible
        FOREIGN KEY (responsible)
            REFERENCES persons(id)
            ON DELETE SET NULL              ,
    CONSTRAINT fk_organisation
        FOREIGN KEY (organisation)
            REFERENCES organisations(id)
            ON DELETE SET NULL              ,
    CONSTRAINT fk_transaction
        FOREIGN KEY (transaction)
            REFERENCES transactions(id)
            ON DELETE SET NULL
);

CREATE TABLE statement_of_accounts (
    id              SERIAL                  ,
    description     TEXT                    ,
    starting        TIMESTAMP               ,
    ending          TIMESTAMP               ,
    added           TIMESTAMP   NOT NULL    ,
    changed         TIMESTAMP               ,
    PRIMARY KEY (id)
);

