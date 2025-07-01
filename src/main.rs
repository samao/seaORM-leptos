use bakery_backend::{
    entities::{prelude::*, *},
    run,
};
use sea_orm::*;
use sea_orm_migration::prelude::*;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|er| DbErr::Custom(format!("{:?}", er)))?;

    let db = &run().await?;

    // update
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::set(res.last_insert_id),
        name: ActiveValue::set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;

    // insert
    let john = chef::ActiveModel {
        name: ActiveValue::set("John".to_owned()),
        bakery_id: ActiveValue::set(res.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(db).await?;

    //find
    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    info!("find.all: {:?}", bakeries);
    let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
    info!("find_by_id.one: {:?}", sad_bakery);
    let sad_bakery: Option<bakery::Model> = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db)
        .await?;
    info!("find.filter.one: {:?}", sad_bakery);

    // delete
    let john = chef::ActiveModel {
        id: ActiveValue::set(1),
        ..Default::default()
    };
    john.delete(db).await?;
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::set(1),
        ..Default::default()
    };
    sad_bakery.delete(db).await?;

    // related insert
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::set(chef_name.to_owned()),
            bakery_id: ActiveValue::set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }

    //related select
    let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
        .one(db)
        .await?
        .unwrap();
    let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(db).await?;
    let mut chef_names: Vec<String> = chefs.into_iter().map(|cf| cf.name).collect();
    chef_names.sort_unstable();
    info!("related: {:?}", chef_names);

    // change name
    let bakery: bakery::ActiveModel = bakery::ActiveModel {
        id: ActiveValue::set(12),
        name: ActiveValue::set("beijing".to_owned()),
        profit_margin: ActiveValue::not_set(),
    };
    bakery.update(db).await?;

    patch_related(db).await?;
    get_top_2(db).await?;
    Ok(())
}

async fn get_top_2(db: &DatabaseConnection) -> Result<(), DbErr> {
    let bk: Vec<bakery::Model> = Bakery::find()
        .limit(2)
        .order_by_desc(bakery::Column::Id)
        .all(db)
        .await?;
    info!("查询TOP 2\n {:#?}", bk);
    Ok(())
}

async fn patch_related(db: &DatabaseConnection) -> Result<(), DbErr> {
    let bakeries = Bakery::find()
        .filter(
            Condition::any()
                .add(bakery::Column::Id.eq(12))
                .add(bakery::Column::Id.eq(14)),
        )
        .all(db)
        .await?;
    let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;

    bakeries
        .into_iter()
        .zip(chefs.into_iter())
        .for_each(|(bakery, chefs)| {
            let chefs: Vec<String> = chefs.into_iter().map(|chef| chef.name).collect();
            info!("bakery:{} has list: {:?}", bakery.name, chefs);
        });
    Ok(())
}
