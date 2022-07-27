use std::cmp::min;
use std::fs;

use rand::Rng;

use eframe::egui;
use egui_extras::image::RetainedImage;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Left Right Center Game",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

enum GameState {
    Setup,
    Rules,
    NameSetup,
    Play,
    ShowDice,
    End,
}

enum Die {
    Dot,
    Left,
    Right,
    Center,
}

struct MyApp {
    state: GameState,
    player_count: usize,
    player_names: Vec<String>,
    player_chips: Vec<u8>,
    current_player: usize,
    dice: Vec<Die>,
    die_dot: RetainedImage,
    die_left: RetainedImage,
    die_right: RetainedImage,
    die_center: RetainedImage,
    rules: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: GameState::Setup,
            player_count: 3,
            player_names: Vec::new(),
            player_chips: Vec::new(),
            current_player: 0,
            dice: Vec::new(),
            die_dot: RetainedImage::from_image_bytes(
                "Die_Dot.png",
                include_bytes!("../res/Die_Dot.png"),
            )
            .unwrap(),
            die_left: RetainedImage::from_image_bytes(
                "Die_Left.png",
                include_bytes!("../res/Die_Left.png"),
            )
            .unwrap(),
            die_right: RetainedImage::from_image_bytes(
                "Die_Right.png",
                include_bytes!("../res/Die_Right.png"),
            )
            .unwrap(),
            die_center: RetainedImage::from_image_bytes(
                "Die_Center.png",
                include_bytes!("../res/Die_Center.png"),
            )
            .unwrap(),
            rules: fs::read_to_string("res/LCRRules.txt").expect("Failed to read rules file"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| match self.state {
            GameState::Setup => {
                egui::Area::new("top_left_area")
                    .movable(false)
                    .anchor(egui::Align2::LEFT_TOP, [20.0, 20.0])
                    .show(ctx, |ui| {
                        if ui.button("Rules").clicked() {
                            self.state = GameState::Rules;
                        }
                    });

                egui::Area::new("top_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -30.0])
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Game Setup");
                            ui.label("Number of players: ");
                        });
                    });

                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [10.0, 0.0])
                    .show(ctx, |ui| {
                        ui.add(egui::Slider::new(&mut self.player_count, 3..=50));
                    });

                egui::Area::new("bottom_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 30.0])
                    .show(ctx, |ui| {
                        if ui.button("Continue").clicked() {
                            for _player in 0..self.player_count {
                                self.player_names.push(String::new());
                                self.player_chips.push(3);
                            }

                            self.state = GameState::NameSetup;
                        }
                    });
            }
            GameState::Rules => {
                egui::Area::new("top_left_area")
                    .movable(false)
                    .anchor(egui::Align2::LEFT_TOP, [20.0, 20.0])
                    .show(ctx, |ui| {
                        if ui.button("Back").clicked() {
                            self.state = GameState::Setup;
                        }
                    });

                egui::Area::new("top_center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_TOP, [0.0, 80.0])
                    .show(ctx, |ui| {
                        ui.heading("Rules of Left Right Center");
                    });

                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [35.0, 0.0])
                    .show(ctx, |ui| {
                        ui.label(self.rules.to_string());
                    });
            }
            GameState::NameSetup => {
                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Player Names");
                            ui.add_space(15.0);

                            ui.label(format!(
                                "Enter a name for player {}:",
                                self.current_player + 1
                            ));
                            ui.add_space(5.0);
                            ui.add(
                                egui::TextEdit::singleline(
                                    &mut self.player_names[self.current_player],
                                )
                                .hint_text("Enter your name here"),
                            );
                            ui.add_space(20.0);

                            if ui.button("Continue").clicked() {
                                if self.current_player < self.player_count - 1 {
                                    self.current_player += 1;
                                } else {
                                    self.current_player = 0;
                                    self.state = GameState::Play;
                                }
                            }
                        });
                    });
            }
            GameState::Play => {
                while self.player_chips[self.current_player] == 0 {
                    self.current_player += 1;
                    if self.current_player >= self.player_count {
                        self.current_player = 0;
                    }
                }

                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 1.0])
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(format!(
                                "{}'s Turn",
                                self.player_names[self.current_player]
                            ));
                            ui.add_space(15.0);

                            ui.label(format!(
                                "You have {} chips.",
                                self.player_chips[self.current_player]
                            ));
                            ui.add_space(10.0);

                            if ui.button("Roll dice").clicked() {
                                let available_chips =
                                    min(self.player_chips[self.current_player], 3);

                                self.dice.clear();
                                for _chip in 1..available_chips + 1 {
                                    self.dice.push(dice_roll(
                                        &mut self.player_chips,
                                        self.current_player,
                                    ));
                                }

                                self.current_player += 1;
                                if self.current_player >= self.player_count {
                                    self.current_player = 0;
                                }

                                self.state = GameState::ShowDice;
                            }
                        });
                    });

                if check_winner(&self.player_chips) {
                    self.state = GameState::End;
                }
            }
            GameState::ShowDice => {
                egui::Area::new("top_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -60.0])
                    .show(ctx, |ui| {
                        ui.heading("You rolled");
                    });

                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            for die in &self.dice {
                                match die {
                                    Die::Dot => self.die_dot.show(ui),
                                    Die::Left => self.die_left.show(ui),
                                    Die::Right => self.die_right.show(ui),
                                    Die::Center => self.die_center.show(ui),
                                };
                            }
                        });
                    });

                egui::Area::new("bottom_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 60.0])
                    .show(ctx, |ui| {
                        if ui.button("Continue").clicked() {
                            self.state = GameState::Play;
                        }
                    });
            }
            GameState::End => {
                egui::Area::new("center_area")
                    .movable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(format!(
                                "Congratulations {}, you are the winner!",
                                get_winner(
                                    &self.player_names,
                                    &self.player_chips,
                                    self.player_count
                                )
                            ));
                            ui.add_space(15.0);

                            if ui.button("Restart").clicked() {
                                self.player_names.clear();
                                self.player_chips.clear();

                                self.state = GameState::Setup;
                            }
                        });
                    });
            }
        });
    }
}

fn check_winner(player_chips: &[u8]) -> bool {
    let mut players_have_chips: u8 = 0;

    for item in player_chips.iter() {
        if item > &0 {
            players_have_chips += 1;
        }
    }

    if players_have_chips == 1 {
        return true;
    }

    false
}

fn get_winner(player_names: &[String], player_chips: &[u8], player_count: usize) -> String {
    for i in 0..player_count {
        if player_chips[i] > 0 {
            return player_names[i].clone();
        }
    }

    String::new()
}

fn dice_roll(player_chips: &mut Vec<u8>, player_index: usize) -> Die {
    let mut rng = rand::thread_rng();
    let roll: u8 = rng.gen_range(1..=6);
    let total_players = player_chips.len();

    if roll == 1 {
        player_chips[player_index] -= 1;

        if player_index != 0 {
            player_chips[player_index - 1] += 1;
        } else {
            player_chips[total_players - 1] += 1;
        }

        return Die::Left;
    } else if roll == 2 {
        player_chips[player_index] -= 1;

        if player_index != total_players - 1 {
            player_chips[player_index + 1] += 1;
        } else {
            player_chips[0] += 0;
        }

        return Die::Right;
    } else if roll == 3 {
        player_chips[player_index] -= 1;

        return Die::Center;
    }

    Die::Dot
}
