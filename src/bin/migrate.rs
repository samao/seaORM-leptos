use std::env;

use bakery_backend::{migrator, run};
use sea_orm::DbErr;
use sea_orm_migration::MigratorTrait;
use tokio::{fs::File, process};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    File::create("sqlite.db")
        .await
        .map_err(|_err| DbErr::Custom("create database fail!".to_string()))?;
    let db = run().await?;
    migrator::Migrator::refresh(&db).await?;

    println!("{:?}", env::current_dir().unwrap());
    let status = process::Command::new("sea-orm-cli")
        .arg("generate")
        .arg("entity")
        .arg("-u")
        .arg("sqlite:./sqlite.db")
        .arg("-o")
        .arg("src/entities")
        .status()
        .await
        .map_err(|err| DbErr::Custom(format!("{:?}", err)))?;

    if status.success() {
        println!("✅ 实体代码已成功生成到目录");
    } else {
        println!("命令执行失败，退出码: {:?}", status.code())
    }

    Ok(())
}
