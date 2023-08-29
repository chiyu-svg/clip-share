use clipshare::data::AppDataBase;
use clipshare::RocketConfig;
use clipshare::get_rocket;
fn main() {
    let rt = tokio::runtime::Runtime::new().expect("faild to spawn tokio runtime");
    let database = rt.block_on(async move {
        AppDataBase::new("sqlite:data.db").await
    });
    let config = RocketConfig {
        database
    };
    rt.block_on(async move {
        get_rocket(config).launch().await.expect("")
    });
}