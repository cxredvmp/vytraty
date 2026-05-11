use toasty::Db;
use uuid::Uuid;

use crate::model::{self, category::Category, currency::Currency};

pub async fn connect_db(db_url: &str) -> Db {
    let db = toasty::Db::builder()
        .models(model::models())
        .connect(db_url)
        .await
        .expect("failed to connect to database");
    eprintln!("database connection established");

    db
}

pub async fn setup_db(db: &mut Db) {
    db.push_schema().await.expect("failed to push schema");
    eprintln!("schema pushed");

    seed(db).await;
    eprintln!("seeded database");
}

async fn seed(db: &mut Db) {
    seed_currencies(db).await;
    seed_categories(db).await;
}

async fn seed_currencies(db: &mut Db) {
    let default_currencies = vec![
        ("USD", "US Dollar", "$"),
        ("EUR", "Euro", "€"),
        ("UAH", "Ukrainian Hryvnia", "₴"),
    ];

    for (code, name, symbol) in default_currencies {
        let exists = Currency::filter(Currency::fields().code().eq(code))
            .get(db)
            .await
            .is_ok();

        if !exists {
            let _ = toasty::create!(Currency {
                code: code.to_string(),
                name: name.to_string(),
                symbol: symbol.to_string(),
            })
            .exec(db)
            .await;
            eprintln!("seeded default currency {}", code);
        }
    }
}

async fn seed_categories(db: &mut Db) {
    let default_categories = vec![
        "Groceries",
        "Transport",
        "Utilities",
        "Entertainment",
        "Healthcare",
    ];

    for name in default_categories {
        let exists = Category::filter(
            Category::fields()
                .user_id()
                .is_none()
                .and(Category::fields().name().eq(name)),
        )
        .get(db)
        .await
        .is_ok();

        if !exists {
            let _ = toasty::create!(Category {
                id: Uuid::new_v4(),
                user_id: None,
                name: name.to_string(),
            })
            .exec(db)
            .await;
            eprintln!("seeded default category: {}", name);
        }
    }
}
