//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub external_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339")]
    pub drafted_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub retracted_at: Option<TimeDateTimeWithTimeZone>,
    pub sender_id: i64,
    #[sea_orm(column_type = "Text")]
    pub subject: String,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub wikitext_hash: Vec<u8>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub compiled_hash: Vec<u8>,
    #[serde(with = "time::serde::rfc3339")]
    pub compiled_at: TimeDateTimeWithTimeZone,
    #[sea_orm(column_type = "Text")]
    pub compiled_generator: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub reply_to: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub forwarded_from: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
    #[sea_orm(has_many = "super::message_recipient::Entity")]
    MessageRecipient,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ForwardedFrom",
        to = "Column::ExternalId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef2,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ReplyTo",
        to = "Column::ExternalId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef1,
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::CompiledHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text2,
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::WikitextHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text1,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SenderId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::message_recipient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageRecipient.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
