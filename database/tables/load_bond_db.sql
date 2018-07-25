
-- Create custom types

CREATE TYPE AssetType AS ENUM ('CURRENCY', 'BOND');
CREATE TYPE QuoteStatus AS ENUM ('OPEN', 'CLOSED');
CREATE TYPE OrderStatus AS ENUM ('OPEN', 'MATCHED', 'SETTLED');
CREATE TYPE OrderType AS ENUM ('MARKET', 'LIMIT');
CREATE TYPE OrderAction AS ENUM ('BUY', 'SELL');
CREATE TYPE OrganizationType AS ENUM ('TRADING_FIRM', 'PRICING_SOURCE');
CREATE TYPE Role AS ENUM ('MARKET_MAKER', 'TRADER');
CREATE TYPE PaymentType AS ENUM ('COUPON', 'REDEMPTION');
CREATE TYPE CouponType AS ENUM ('FIXED', 'FLOATING');
CREATE TYPE CouponFrequency AS ENUM ('QUARTERLY', 'MONTHLY', 'DAILY');


-- Create tables

CREATE TABLE IF NOT EXISTS bonds (
  id                        bigserial   PRIMARY KEY,
  bond_id                   varchar,
  issuing_organization_id   varchar,
  amount_outstanding        bigint,
  coupon_rate               bigint,
  first_coupon_date         bigint,
  first_settlement_date     bigint,
  maturity_date             bigint,
  face_value                bigint,
  coupon_type               CouponType,
  coupon_frequency          CouponFrequency,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS corporate_debt_ratings (
  id                        bigserial   PRIMARY KEY,
  bond_id                   varchar,
  agency                    varchar,
  rating                    varchar,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS organizations (
  id                        bigserial   PRIMARY KEY,
  organization_id           varchar,
  industry                  varchar,
  name                      varchar,
  organization_type         OrganizationType,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS authorizations (
  id                        bigserial   PRIMARY KEY,
  participant_public_key    varchar,
  organization_id           varchar,
  role                      Role,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS receipts (
  id                        bigserial   PRIMARY KEY,
  payee_organization_id     varchar,
  bond_id                   varchar,
  payment_type              PaymentType,
  coupon_date               bigint,
  amount                    bigint,
  timestamp                 bigint,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS holdings (
  id                        bigserial   PRIMARY KEY,
  owner_organization_id     varchar,
  asset_id                  varchar,
  asset_type                AssetType,
  amount                    bigint,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS participants (
  id                        bigserial   PRIMARY KEY,
  public_key                varchar,
  organization_id           varchar,
  username                  varchar,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS auth (
  username                  varchar  PRIMARY KEY,
  hashed_password           varchar,
  encrypted_private_key     varchar
);

CREATE TABLE IF NOT EXISTS quotes (
  id                        bigserial   PRIMARY KEY,
  bond_id                   varchar,
  organization_id           varchar,
  ask_price                 bigint,
  ask_qty                   bigint,
  bid_price                 bigint,
  bid_qty                   bigint,
  quote_id                  varchar,
  status                    QuoteStatus,
  timestamp                 bigint,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS orders (
  id                        bigserial   PRIMARY KEY,
  bond_id                   varchar,
  ordering_organization_id  varchar,
  order_id                  varchar,
  limit_price               bigint,
  limit_yield               bigint,
  quantity                  bigint,
  quote_id                  varchar,
  status                    OrderStatus,
  action                    OrderAction,
  order_type                OrderType,
  timestamp                 bigint,
  start_block_num           bigint,
  end_block_num             bigint
);


CREATE TABLE IF NOT EXISTS settlements (
  id                        bigserial   PRIMARY KEY,
  order_id                  varchar,
  ordering_organization_id  varchar,
  quoting_organization_id   varchar,
  action                    OrderAction,
  bond_quantity             bigint,
  currency_amount           bigint,
  start_block_num           bigint,
  end_block_num             bigint
);

CREATE TABLE IF NOT EXISTS blocks (
  block_num                bigint   PRIMARY KEY,
  block_id                 varchar
);
