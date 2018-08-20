use crate::tests::common::run_db_test_parallel;

#[test]
fn get_player_status() {
    run_db_test_parallel(|context, config, _uid_generator| async move {
        let games = config.game_apis.as_ref().unwrap();
        let response = context
            .http_client
            .get(format!("{}/players/%2382L0C9YG", games.clash_royale.url.as_str()).as_str())
            .bearer_auth(games.clash_royale.token.as_str())
            .send()
            .await
            .unwrap();

        let text = response.text().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let value = vec![
            json["currentDeck"][0]["name"].clone(),
            json["currentDeck"][1]["name"].clone(),
            json["currentDeck"][2]["name"].clone(),
            json["currentDeck"][3]["name"].clone(),
            json["currentDeck"][4]["name"].clone(),
            json["currentDeck"][5]["name"].clone(),
            json["currentDeck"][6]["name"].clone(),
            json["currentDeck"][7]["name"].clone(),
        ];
        println!("{:?}", value)
    });
}
