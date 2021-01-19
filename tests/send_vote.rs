use std::vec;

use reqwest;
use service_serving_layer::routes::post_vote;
use service_serving_layer::routes::post_vote::Vote;
use service_serving_layer::ADDRESS;
use actix_web::{test, web, App};

#[actix_rt::test]
    async fn test_index_post() {

        let vote = Vote {
            name: "Albert Einstein".to_owned(),
            country: "Deutschland".to_owned(),
            candidate: "Red".to_owned(),
            date: "1611051908741".to_owned()
        };

        let data = vec![vote];

        let mut app = test::init_service(App::new().service(post_vote::exec)).await;
        let req = test::TestRequest::post().uri("/post").set_json(&data).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }