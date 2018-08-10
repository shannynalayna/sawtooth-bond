// Copyright 2018 Bitwise IO
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

// Role

#[derive(SqlType)]
#[postgres(type_name = "role")]
pub struct Role;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Role"]
pub enum RoleEnum {
    MAKERTMAKER,
    TRADER,
}

impl ToSql<Role, Pg> for RoleEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            RoleEnum::MAKERTMAKER => out.write_all(b"MAKERTMAKER")?,
            RoleEnum::TRADER => out.write_all(b"TRADER")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Role, Pg> for RoleEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"MAKERTMAKER" => Ok(RoleEnum::MAKERTMAKER),
            b"TRADER" => Ok(RoleEnum::TRADER),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// AssetType

#[derive(SqlType)]
#[postgres(type_name = "asset_type")]
pub struct AssetType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "AssetType"]
pub enum AssetTypeEnum {
    CURRENCY,
    BOND,
}

impl ToSql<AssetType, Pg> for AssetTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            AssetTypeEnum::CURRENCY => out.write_all(b"CURRENCY")?,
            AssetTypeEnum::BOND => out.write_all(b"BOND")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<AssetType, Pg> for AssetTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"CURRENCY" => Ok(AssetTypeEnum::CURRENCY),
            b"BOND" => Ok(AssetTypeEnum::BOND),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// QuoteStatus

#[derive(SqlType)]
#[postgres(type_name = "quote_status")]
pub struct QuoteStatus;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "QuoteStatus"]
pub enum QuoteStatusEnum {
    OPEN,
    CLOSED,
}

impl ToSql<QuoteStatus, Pg> for QuoteStatusEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            QuoteStatusEnum::OPEN => out.write_all(b"OPEN")?,
            QuoteStatusEnum::CLOSED => out.write_all(b"CLOSED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<QuoteStatus, Pg> for QuoteStatusEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"OPEN" => Ok(QuoteStatusEnum::OPEN),
            b"CLOSED" => Ok(QuoteStatusEnum::CLOSED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// OrderStatus

#[derive(SqlType)]
#[postgres(type_name = "order_status")]
pub struct OrderStatus;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "OrderStatus"]
pub enum OrderStatusEnum {
    OPEN,
    MATCHED,
    SETTLED,
}

impl ToSql<OrderStatus, Pg> for OrderStatusEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OrderStatusEnum::OPEN => out.write_all(b"OPEN")?,
            OrderStatusEnum::MATCHED => out.write_all(b"MATCHED")?,
            OrderStatusEnum::SETTLED => out.write_all(b"SETTLED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrderStatus, Pg> for OrderStatusEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"OPEN" => Ok(OrderStatusEnum::OPEN),
            b"MATCHED" => Ok(OrderStatusEnum::MATCHED),
            b"SETTLED" => Ok(OrderStatusEnum::SETTLED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// OrderType

#[derive(SqlType)]
#[postgres(type_name = "order_type")]
pub struct OrderType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "OrderType"]
pub enum OrderTypeEnum {
    MARKET,
    LIMIT,
}

impl ToSql<OrderType, Pg> for OrderTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OrderTypeEnum::MARKET => out.write_all(b"MARKET")?,
            OrderTypeEnum::LIMIT => out.write_all(b"LIMIT")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrderType, Pg> for OrderTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"MARKET" => Ok(OrderTypeEnum::MARKET),
            b"LIMIT" => Ok(OrderTypeEnum::LIMIT),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// OrderAction

#[derive(SqlType)]
#[postgres(type_name = "order_action")]
pub struct OrderAction;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "OrderAction"]
pub enum OrderActionEnum {
    BUY,
    SELL,
}

impl ToSql<OrderAction, Pg> for OrderActionEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OrderActionEnum::BUY => out.write_all(b"BUY")?,
            OrderActionEnum::SELL => out.write_all(b"SELL")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrderAction, Pg> for OrderActionEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"BUY" => Ok(OrderActionEnum::BUY),
            b"SELL" => Ok(OrderActionEnum::SELL),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// OrganizationType

#[derive(SqlType)]
#[postgres(type_name = "organization_type")]
pub struct OrganizationType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "OrganizationType"]
pub enum OrganizationTypeEnum {
    TRADINGFIRM,
    PRICINGSOURCE,
}

impl ToSql<OrganizationType, Pg> for OrganizationTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OrganizationTypeEnum::TRADINGFIRM => out.write_all(b"TRADING_FIRM")?,
            OrganizationTypeEnum::PRICINGSOURCE => out.write_all(b"PRICING_SOURCE")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrganizationType, Pg> for OrganizationTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"TRADING_FIRM" => Ok(OrganizationTypeEnum::TRADINGFIRM),
            b"PRICING_SOURCE" => Ok(OrganizationTypeEnum::PRICINGSOURCE),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// PaymentType

#[derive(SqlType)]
#[postgres(type_name = "payment_type")]
pub struct PaymentType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "PaymentType"]
pub enum PaymentTypeEnum {
    COUPON,
    REDEMPTION,
}

impl ToSql<PaymentType, Pg> for PaymentTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            PaymentTypeEnum::COUPON => out.write_all(b"COUPON")?,
            PaymentTypeEnum::REDEMPTION => out.write_all(b"REDEMPTION")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PaymentType, Pg> for PaymentTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"COUPON" => Ok(PaymentTypeEnum::COUPON),
            b"REDEMPTION" => Ok(PaymentTypeEnum::REDEMPTION),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// CouponType

#[derive(SqlType)]
#[postgres(type_name = "coupon_type")]
pub struct CouponType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "CouponType"]
pub enum CouponTypeEnum {
    FIXED,
    FLOATING,
}

impl ToSql<CouponType, Pg> for CouponTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            CouponTypeEnum::FIXED => out.write_all(b"FIXED")?,
            CouponTypeEnum::FLOATING => out.write_all(b"FLOATING")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<CouponType, Pg> for CouponTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"FIXED" => Ok(CouponTypeEnum::FIXED),
            b"FLOATING" => Ok(CouponTypeEnum::FLOATING),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// CouponFrequency

#[derive(SqlType)]
#[postgres(type_name = "coupon_frequency")]
pub struct CouponFrequency;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "CouponFrequency"]
pub enum CouponFrequencyEnum {
    QUARTERLY,
    MONTHLY,
    DAILY,
}

impl ToSql<CouponFrequency, Pg> for CouponFrequencyEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            CouponFrequencyEnum::QUARTERLY => out.write_all(b"QUARTERLY")?,
            CouponFrequencyEnum::MONTHLY => out.write_all(b"MONTHLY")?,
            CouponFrequencyEnum::DAILY => out.write_all(b"DAILY")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<CouponFrequency, Pg> for CouponFrequencyEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"QUARTERLY" => Ok(CouponFrequencyEnum::QUARTERLY),
            b"MONTHLY" => Ok(CouponFrequencyEnum::MONTHLY),
            b"DAILY" => Ok(CouponFrequencyEnum::DAILY),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
