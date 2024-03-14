use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Base {
    Id,
    CreatedAt,
    UpdatedAt,
}

pub struct BaseTable {}

impl BaseTable {
    pub fn create<T: IntoTableRef>(table: T) -> TableCreateStatement {
        Table::create()
            .table(table)
            .col(
                ColumnDef::new(Base::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(PgFunc::gen_random_uuid()),
            )
            .col(
                ColumnDef::new(Base::CreatedAt)
                    .timestamp()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Base::UpdatedAt)
                    .timestamp()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned()
    }
}
