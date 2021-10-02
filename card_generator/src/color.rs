use image::Rgba;

use crate::card::CardType;

pub fn get_background_color(card_type: &CardType) -> Rgba<u8> {
  match card_type {
    CardType::ActionSubject(_) => Rgba::<u8>([0, 128, 0, 50]),
    CardType::ActionEffect => Rgba::<u8>([0, 0, 0, 50]),
    CardType::ActionTrouble => Rgba::<u8>([0, 0, 0, 50]),
    CardType::ActionCheat => Rgba::<u8>([0, 0, 0, 50]),
  }
}

