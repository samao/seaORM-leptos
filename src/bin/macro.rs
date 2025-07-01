use std::fmt::Debug;

use sea_orm::DbErr;
struct Taobao;
impl Taobao {
    fn create() -> Self {
        println!("create_taobao");
        Taobao
    }
    fn table(&self, name: impl IntoIden + Debug) -> &Self {
        println!("taobao_table = {:?}", name);
        name.say();
        self
    }

    fn to_owned(&self) -> &Self {
        println!("taobao to_owned");
        self
    }

    fn col(&self, _col: Col) -> &Self {
        self
    }
}
struct Mgr;
impl Mgr {
    async fn create_table(&self, _tb: &Taobao) -> Result<(), DbErr> {
        println!("create_table");
        Ok(())
    }
}

struct Col;
impl Col {
    fn new(name: impl IntoIden + Debug) -> Self {
        println!("Col={:?}", name);
        Col
    }
}

trait IntoIden {
    fn say(&self) {
        println!("Into Iden say");
    }
}

macro_rules! table {
    ($table_name:ident, {$($col_name:tt => $col_fn: expr),*}) => {
        #[derive(Debug)]
        pub enum $table_name {
            Table,
            $($col_name,)*
        }

        impl IntoIden for $table_name {}

        impl $table_name {
            async fn up(&self, manager: &Mgr) -> Result<(), DbErr> {
                manager.create_table(
                    Taobao::create()
                        .table($table_name::Table)
                        $(.col(Col::new($table_name::$col_name)))*
                        .to_owned()
                ).await
            }
        }
    };
}

table! (
    User,
    {
        Id => 1+1,
        Name => 2 + 2
    }
);

table! {
    Person,
    {
        Age => 87,
        Name => "GOOD"
    }
}

#[tokio::main]
async fn main() {
    User::Table.up(&Mgr).await.unwrap();
    Person::Name.up(&Mgr).await.unwrap();
}
