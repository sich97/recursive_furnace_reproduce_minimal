use recursive_furnace_reproduce_minimal::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_model() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // query your model, e.g.:
    //
    // let item = models::posts::Model::find_by_pid(
    //     &boot.app_context.db,
    //     "11111111-1111-1111-1111-111111111111",
    // )
    // .await;

    // snapshot the result:
    // assert_debug_snapshot!(item);
}
