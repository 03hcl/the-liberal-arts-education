use image::{ImageBuffer, RgbaImage, Rgba};
use image::imageops::overlay;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

mod raw_card;
use raw_card::{CardSet, read_cardset};

mod card;
use card::{CardType};

mod file_io;
use file_io::read;

use crate::card::ActionCard;

fn get_background_color(card_type: &CardType) -> Rgba<u8> {
  match card_type {
    CardType::ActionSubject(_) => Rgba::<u8>([0, 128, 0, 50]),
    CardType::ActionEffect => Rgba::<u8>([0, 0, 0, 50]),
    CardType::ActionTrouble => Rgba::<u8>([0, 0, 0, 50]),
    CardType::ActionCheat => Rgba::<u8>([0, 0, 0, 50]),
  }
}

fn main() {
  println!("Hello, world!");

  let test = raw_card::create_test_accounts();
  eprintln!("{}", toml::to_string(&test).unwrap());

  let cardset: CardSet = read_cardset(
    read(String::from("U:\\OneDrive\\Games\\TheLiberalArtsEducation\\GitHub\\cards.toml")).unwrap());

  eprintln!("{:?}", cardset.actions);
  let action_subject_cards = cardset.actions.card.iter()
  .filter(|&card| match CardType::from_raw(&card).unwrap() { CardType::ActionSubject(_) => true, _ => false})
  .map(|c| ActionCard::from_raw(&c).unwrap())
  .collect::<Vec<_>>();
  eprintln!("action_subjects.len = {}", action_subject_cards.len());
  eprintln!("{:?}", action_subject_cards);
  
  // let mut image = image::open("U:\\OneDrive\\Games\\TheLiberalArtsEducation\\card\\dummy_1299x2150.png").unwrap();
  // let font = Vec::from(include_bytes!("C:\\Windows\\Fonts\\IBMPlexSansJP-Light.otf") as &[u8]);
  // let font = Font::try_from_vec(font).unwrap();

  // // let image1 = image.to_rgba8();

  // let height = 72.0;
  // let scale = Scale {
  //     x: height,
  //     y: height,
  // };

  // let text = "テスト";
  // draw_text_mut(&mut image, Rgba([50u8, 50u8, 50u8, 255u8]), 50, 0, scale, &font, text);

  // image.save("U:\\OneDrive\\Games\\TheLiberalArtsEducation\\card\\dummy_out1.png").unwrap();
  // eprintln!("create i2.png");
  // let mut i2 = create_background_image(CardType::ActionSubject);
  // i2.save("U:\\OneDrive\\Games\\TheLiberalArtsEducation\\card\\i2.png").unwrap();
}

fn create_background_image(card_type: CardType) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let width = 55.0;
  let height = 91.0;
  
  let mut image = ImageBuffer::from_fn(
    mm_to_pixel(width), mm_to_pixel(height), |x, y| {
    if ((x / 25) % 2) ^ ((y / 25) % 2) == 0 {
      Rgba([255, 255, 255, 255])
    } else {
      Rgba([230, 230, 230, 255])
    }
  });

  add_rectangle(&mut image, 0.0, 0.0, width, height, get_background_color(&card_type));
  add_rectangle(&mut image, 3.5, 7.0, 48.0, 8.0, get_background_color(&card_type));
  add_rectangle(&mut image, 3.5, 16.0, 48.0, 40.0, Rgba([255, 255, 255, 255]));
  add_rectangle(&mut image, 3.5, 59.0, 48.0, 28.0, Rgba([255, 255, 255, 128]));
  image
}

fn mm_to_pixel(x: f32) -> u32 { (x * 600.0 / 25.4).round() as u32 }

fn add_rectangle(
  image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
  x: f32, y: f32, width: f32, height: f32, color: Rgba<u8>)
{
  let x = mm_to_pixel(x);
  let y = mm_to_pixel(y);
  let width = mm_to_pixel(width);
  let height = mm_to_pixel(height);
  eprintln!("左上座標 = ({}, {}), サイズ = ({}, {})", x, y, width, height);
  let rect = RgbaImage::from_pixel(width, height, color);
  overlay(image, &rect, x, y);
}