use std::collections::HashMap;

pub fn body() {
    let mut scores = HashMap::new();
    scores.insert("lenin".to_string(), 23);
    scores.insert("karen".to_string(), 24);
    dbg!(&scores);
    let team = "karen".to_string();
    let team_score = scores.get(&team);
    dbg!(&team_score);
    let team_score = team_score.copied();
    dbg!(&team_score);
    let team_score = team_score.unwrap_or(0);
    dbg!(&team_score);

    for (k, v) in &scores {
        dbg!(k, v);
    }
    dbg!(&scores);
    scores.insert("alex".to_string(), 10);
    dbg!(&scores);
    scores.insert("alex".to_string(), 20);
    dbg!(&scores);

    scores.entry("moka".to_string()).or_insert(1);
    scores.entry("karen".to_string()).or_insert(1);
    dbg!(&scores);

    let hello = "hello world wonderful world".to_string();
    let mut map: HashMap<String, u32> = HashMap::new();
    for word in hello.split_whitespace() {
        println!("{word}");
        let counter = map.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }
    dbg!(&map);

}