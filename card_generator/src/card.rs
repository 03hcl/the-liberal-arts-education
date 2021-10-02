use serde::{Deserialize, Serialize};
use crate::raw_card::{Actions, Card};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CardType {
  ActionSubject(Subject),
  ActionEffect,
  ActionTrouble,
  ActionCheat,
}

impl CardType {
  pub fn from_raw(card: &Card) -> Result<CardType, String> {
    let s: &str = &card.card_type;
    match s {
      "教科カード" => Ok(CardType::ActionSubject(Subject::from_str(&card.subject).unwrap())),
      "特殊カード" => Ok(CardType::ActionEffect),
      "トラブルカード" => Ok(CardType::ActionTrouble),
      "インチキカード" => Ok(CardType::ActionCheat),
      _ => Err(String::from("invalid card type.")),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Subject {
  Math,
  Physics,
  Chemistry,
  EarthScience,
  Biology,
  ComputerScience,
  English,
  Japanese,
  SocialStudies,
  History,
  Geography,
  Ethics,
  PoliticsAndEconomy,
  Music,
  Art,
  CraftsProduction,
  PhysicalEducation,
  Health,
  // Unknown,
}

impl Subject {
  pub fn from_str(subject: &Option<String>) -> Result<Subject, String> {
    if let Some(s) = subject {
      let s: &str = &s;
      match s {
        "数学" => Ok(Subject::Math),
        "物理" => Ok(Subject::Physics),
        "化学" => Ok(Subject::Chemistry),
        "地学" => Ok(Subject::EarthScience),
        "生物" => Ok(Subject::Biology),
        "情報" => Ok(Subject::ComputerScience),
        "英語" => Ok(Subject::English),
        "国語" => Ok(Subject::Japanese),
        "社会" => Ok(Subject::SocialStudies),
        "歴史" => Ok(Subject::History),
        "地理" => Ok(Subject::Geography),
        "倫理" => Ok(Subject::Ethics),
        "政経" => Ok(Subject::PoliticsAndEconomy),
        "音楽" => Ok(Subject::Music),
        "美術" => Ok(Subject::Art),
        "工芸" => Ok(Subject::CraftsProduction),
        "体育" => Ok(Subject::PhysicalEducation),
        "保健" => Ok(Subject::Health),
        // _ => Ok(Subject::Unknown),
        _ => Err(String::from("unmatched subject string.")),
      }
    } else {
      // Ok(Subject::Unknown)
      Err(String::from("undefined subject string.")) 
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionCard {
  pub id: i32,
  pub subject: Subject,
  pub name: String,
  pub numbers_in_rikei: u32,
  pub numbers_in_bunkei: u32,
  pub condition: Vec<String>,
  pub effect: Vec<String>,
  pub flavor: Vec<String>,
}

impl ActionCard {
  pub fn from_raw(raw: &Card) -> Result<ActionCard, String> {
    let subject = Subject::from_str(&raw.subject)?;
    Ok(ActionCard {
      id: raw.id,
      subject: subject,
      name: raw.name.clone(),
      numbers_in_rikei: raw.numbers_in_rikei.unwrap_or(0),
      numbers_in_bunkei: raw.numbers_in_bunkei.unwrap_or(0),
      condition: clone_or_default_option_vectors(&raw.condition),
      effect: raw.effect.clone(),
      flavor: clone_or_default_option_vectors(&raw.flavor),
    })
  }
}

fn clone_or_default_option_vectors<T: Clone>(vec: &Option<Vec<T>>) -> Vec<T> {
  match vec {
    Some(v) => v.clone(),
    None => vec![],
  }
}

impl Actions {
  pub fn get_action_subjects(&self) -> Vec<ActionCard> {
    self.card.iter()
      .filter(|&card| match CardType::from_raw(&card).unwrap() { CardType::ActionSubject(_) => true, _ => false})
      .map(|c| ActionCard::from_raw(&c).unwrap())
      .collect::<Vec<_>>()
  }
}
