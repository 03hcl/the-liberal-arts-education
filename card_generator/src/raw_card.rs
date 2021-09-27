use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CardSet {
  // pub credits: Credits,
  pub actions: Actions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credits {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Actions {
  pub card: Vec<Card>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
  pub id: i32,
  pub card_type: String,
  pub subject: Option<String>,
  pub name: String,
  pub numbers_in_rikei: Option<u32>,
  pub numbers_in_bunkei: Option<u32>,
  pub condition: Option<Vec<String>>,
  pub effect: Vec<String>,
  pub flavor: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardTypeStruct {
  pub name: String,
  pub subject: String,
}

pub fn create_test_accounts() -> CardSet {
  read_cardset(String::from(r#"
  [actions]
  [[actions.card]]
    id = 1
    card_type = "教科カード"
    subject = "数学"
    name = "授業に出席"
    numbers_in_rikei = 8
    effect = [
      "数学 +2",
      "数学の教科カードの代わりに 情報 +1 もしくは 物理 +1 として扱える。",
    ]
  "#))
}

pub fn read_cardset(toml_str: String) -> CardSet {
  let cardset = toml::from_str::<CardSet>(&toml_str).unwrap();
  println!("actions.card.len = {}", cardset.actions.card.len());
  cardset
}
