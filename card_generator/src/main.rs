use image::{ImageBuffer, RgbaImage, Rgba};
use image::imageops::overlay;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

mod raw_card;
mod card;
mod file_io;
mod color;

use crate::raw_card::{CardSet, read_cardset};
use crate::card::{CardType, Subject};
use crate::card::ActionCard;
use crate::file_io::{read, write};
use crate::color::get_background_color;

use std::env;

fn main() {
  println!("Hello, world!");
  let path = env::current_dir().unwrap();
  println!("starting dir: {}", path.display());

  let test = raw_card::create_test_accounts();
  eprintln!("{}", toml::to_string(&test).unwrap());

  let cardset: CardSet = read_cardset(
    read(String::from("..\\cards.toml")).unwrap());

  // eprintln!("{:?}", cardset.actions);
  let action_subject_cards: Vec<ActionCard> = cardset.actions.get_action_subjects();
  eprintln!("action_subjects.len = {}", action_subject_cards.len());
  // eprintln!("{:?}", action_subject_cards);
  write(toml::to_string(&action_subject_cards).unwrap(),
   String::from("..\\..\\cards\\action_subject_cards.toml")).unwrap();

  let mut image = image::open("..\\..\\cards\\dummy_1299x2150.png").unwrap();
  let font = Vec::from(include_bytes!("C:\\Windows\\Fonts\\IBMPlexSansJP-Light.otf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

  // let image1 = image.to_rgba8();

  let height = 72.0;
  let scale = Scale {
      x: height,
      y: height,
  };

  // let subject_ = action_subject_cards.iter().enumerate();


  let text = "テスト";
  draw_text_mut(&mut image, Rgba([50u8, 50u8, 50u8, 255u8]), 50, 0, scale, &font, text);

  image.save("..\\..\\cards\\dummy_out1.png").unwrap();
  eprintln!("create i2.png");
  let mut i2 = create_background_image(CardType::ActionSubject(Subject::Math));
  i2.save("..\\..\\cards\\dummy_out2.png").unwrap();
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
  add_rectangle(&mut image, 3.5, 3.5, 48.0, 3.5, Rgba([255, 255, 255, 128]));
  add_rectangle(&mut image, 3.5, 7.0, 48.0, 5.5, get_background_color(&card_type));
  add_rectangle(&mut image, 3.5, 14.0, 48.0, 36.0, Rgba([255, 255, 255, 255]));
  add_rectangle(&mut image, 3.5, 51.5, 48.0, 36.0, Rgba([255, 255, 255, 128]));
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