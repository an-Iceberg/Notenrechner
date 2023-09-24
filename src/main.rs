#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::ops::{Div, Mul};

use eframe::{egui::{self, Slider, Button}, epaint::Color32};

fn main() -> Result<(), eframe::Error>
{ let options = eframe::NativeOptions
  { initial_window_size: Some(egui::vec2(400.0, 600.0)),
    ..Default::default()
  };

  #[derive(PartialEq, Eq)]
  enum Language { EN, DE }

  // Our application state:
  let mut grades = Vec::<f32>::new();
  grades.push(5.5);
  grades.push(4.5);

  let mut example_grade = 4.5;
  let mut grade_to_edit: Option<usize> = None;
  let mut marked_for_deletion: Option<usize> = None;
  let mut language = Language::DE;

  eframe::run_simple_native("Notenrechner", options, move |ctx, _frame|
  { egui::CentralPanel::default().show(ctx, |ui|
    { ui.horizontal(|ui|
      { ui.heading("Notenrechner");
        ui.separator();
        ui.selectable_value(&mut language, Language::DE, "Deutsch");
        ui.selectable_value(&mut language, Language::EN, "English");
      });

      ui.separator();

      ui.label(format!("{}: {:.2}", match language
      { Language::EN => "Median",
        Language::DE => "Median"
      }, "NaN"
      ));

      ui.label(format!("{}: {:.2}", match language
      { Language::EN => "Arithmetic average",
        Language::DE => "Arithmetisches Mittel"
      }, grades.iter().sum::<f32>() / grades.len() as f32
      ));

      ui.label(format!("{}: {:.2}", match language
      { Language::EN => "Quadratic average",
        Language::DE => "Quadratisches Mittel"
      }, grades.iter().fold(0.0, |accumulator, grade| accumulator + grade.powi(2)).div(grades.len() as f32).sqrt()
      ));

      // TOFIX
      ui.label(format!("{}: {:.2}", match language
      { Language::EN => "Geometric average",
        Language::DE => "Geometrisches Mittel"
      }, grades.iter().fold(1.0, |accumulator, grade| accumulator * grade).powf(1.0 / grades.len() as f32)
      ));

      // TOFIX
      ui.label(format!("{}: {:.2}", match language
      { Language::EN => "Harmonic average",
        Language::DE => "Harmonisches Mittel"
      }, grades.iter().fold(0.0, |accumulator, grade| accumulator + (1.0 / grade)).powi(-1).mul(grades.len() as f32)
      ));

      ui.separator();

      // Add new grade
      ui.horizontal(|ui|
      { ui.horizontal(|ui|
        { ui.label("Note:");
          ui.add(Slider::new(&mut example_grade, 1.0..=6.0).step_by(0.05));
        });

        // TODO: change language
        if ui.add(Button::new(match language
        { Language::EN => "add",
          Language::DE => "hinzufügen"
        }).fill(Color32::from_rgb(0, 128, 0))).clicked()
        { grades.push(example_grade);
        }
      });

      ui.separator();

      for (index, grade) in grades.iter_mut().enumerate()
      { ui.horizontal(|ui|
        { ui.label(format!("{:.2}", grade));

          if ui.add(Button::new(match language
          { Language::EN => "edit",
            Language::DE => "bearbeiten"
          }).fill(Color32::from_rgb(0, 128, 255))).clicked()
          {
            let old_grade = grade_to_edit;
            grade_to_edit = Some(index);

            if old_grade == grade_to_edit
            { grade_to_edit = None;
            }
          }

          if ui.add(Button::new(match language
          { Language::EN => "delete",
            Language::DE => "löschen"
          }).fill(Color32::from_rgb(255, 0, 0))).clicked()
          { marked_for_deletion = Some(index);
          }
        });

        if let Some(grade_to_edit) = grade_to_edit
        { if index == grade_to_edit
          { ui.add(Slider::new(grade, 1.0..=6.0).step_by(0.05));
          }
        }

      };

      if let Some(marked_index) = marked_for_deletion
      { grades.remove(marked_index);
        marked_for_deletion = None;
      }
    });
  })
}

fn median(vec: &Vec<f32>) -> f32
{ if vec.len() == 0 { return 0.0; }

  // Even
  if vec.len() % 2 == 0
  { return (vec[vec.len() / 2] + vec[(vec.len() / 2) + 1]) / 2.0;
  }
  // Odd
  else
  { return vec[(vec.len() / 2) + 1];
  }
}
