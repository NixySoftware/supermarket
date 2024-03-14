use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::base::{Base, BaseTable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                BaseTable::create(User::Table)
                    .col(ColumnDef::new(User::Name).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                BaseTable::create(EmailAddress::Table)
                    .col(
                        ColumnDef::new(EmailAddress::Email)
                            .string_len(254)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(EmailAddress::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(EmailAddress::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(EmailAddress::VerificationToken).string_len(32))
                    .col(ColumnDef::new(EmailAddress::VerifiedAt).timestamp())
                    .col(ColumnDef::new(EmailAddress::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(EmailAddress::FkEmailAddressUser.to_string())
                            .from(EmailAddress::Table, EmailAddress::UserId)
                            .to(User::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ProviderType::Table)
                    .values([
                        ProviderType::Apple,
                        ProviderType::Google,
                        ProviderType::Microsoft,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                BaseTable::create(Provider::Table)
                    .col(
                        ColumnDef::new(Provider::Type)
                            .custom(ProviderType::Table)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                BaseTable::create(ProviderConnection::Table)
                    .col(
                        ColumnDef::new(ProviderConnection::Identifier)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderConnection::AccessToken).text())
                    .col(ColumnDef::new(ProviderConnection::RefreshToken).text())
                    .col(ColumnDef::new(ProviderConnection::ExpiresAt).timestamp())
                    .col(
                        ColumnDef::new(ProviderConnection::ProviderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderConnection::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(ProviderConnection::FkProviderConnectionProvider.to_string())
                            .from(ProviderConnection::Table, ProviderConnection::ProviderId)
                            .to(Provider::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ProviderConnection::FkProviderConnectionUser.to_string())
                            .from(ProviderConnection::Table, ProviderConnection::UserId)
                            .to(User::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name(ProviderConnection::UniqueIdentifierProviderId.to_string())
                            .col(ProviderConnection::Identifier)
                            .col(ProviderConnection::ProviderId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderConnection::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Provider::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ProviderType::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(EmailAddress::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,

    Name,
}

#[derive(DeriveIden)]
enum EmailAddress {
    Table,

    Email,
    IsPrimary,
    IsVerified,
    VerificationToken,
    VerifiedAt,

    UserId,

    FkEmailAddressUser,
}

#[derive(DeriveIden)]
enum ProviderType {
    Table,

    Apple,
    Google,
    Microsoft,
}

#[derive(DeriveIden)]
enum Provider {
    Table,

    Type,
}

#[derive(DeriveIden)]
enum ProviderConnection {
    Table,

    Identifier,
    AccessToken,
    RefreshToken,
    ExpiresAt,

    ProviderId,
    UserId,

    FkProviderConnectionProvider,
    FkProviderConnectionUser,
    UniqueIdentifierProviderId,
}
