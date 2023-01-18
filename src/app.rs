/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct Ehinter {
    label: String,
    ticket: String,
    auto_paste_enabled: bool,
    auto_copy_enabled: bool,
    auto_clear_enabled: bool,
    password_mode_enabled: bool,
    sidepanel_enabled: bool,
    enable_signature: bool,
    tick_escalated: bool,
    tick_chat: bool,
    reg_tick_set: Vec<String>,
    chat_tick_set: Vec<String>,
    esc_tick_set: Vec<String>,
    
    ticket_list_str: String,
    allowed_to_clear: bool, 
    show_clear_confirm: bool,

    chat_tick_list_str: String,
    chat_show_clear: bool,

    esc_tick_list_str: String,
    esc_show_clear: bool,

    #[serde(skip)]
    value: f32,
}

impl Default for Ehinter {
    fn default() -> Self {
        Self {
            
            label: "".to_owned(),
            ticket: "".to_owned(),
            value: 0.0,
            auto_paste_enabled: false,
            auto_copy_enabled: false,
            auto_clear_enabled: false,
            password_mode_enabled: false,
            sidepanel_enabled: true,
            enable_signature: true,
            tick_escalated: false,
            tick_chat: false,
            reg_tick_set: [].to_vec(),
            chat_tick_set: [].to_vec(),
            esc_tick_set: [].to_vec(),
            ticket_list_str: "".to_owned(),
            
            allowed_to_clear: false,
            show_clear_confirm: false,

            chat_tick_list_str: "".to_owned(),
            chat_show_clear: false,
        
            esc_tick_list_str: "".to_owned(),
            esc_show_clear: false,
        }
    }
}

use crate::ticket_tracker_functions::vec_iter_reg; 
use crate::ehinter_functions::fn_hint;
use crate::ehinter_functions::fn_copy;
use crate::ehinter_functions::fn_paste;
use eframe::egui;

impl Ehinter {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Ehinter {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {label, value, auto_clear_enabled, 
            auto_copy_enabled, auto_paste_enabled, password_mode_enabled, 
            sidepanel_enabled, enable_signature, ticket,
            tick_escalated, tick_chat, reg_tick_set, 
            chat_tick_set, esc_tick_set, ticket_list_str,
            allowed_to_clear, show_clear_confirm, chat_tick_list_str,
            chat_show_clear, esc_tick_list_str, esc_show_clear} = self;


        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                // appearance settings here
                ui.menu_button("Appearance", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.checkbox(
                        &mut self.sidepanel_enabled,
                        "Enable Sidepanel",
                    );

                    ui.checkbox(
                        &mut self.enable_signature,
                        "Enable Signature",
                    );

                });

                // options section will only appear if sidepanel is disabled, they show the same settings
                if !self.sidepanel_enabled {
                ui.menu_button("Options", |ui| {
                    ui.checkbox(
                    &mut self.auto_paste_enabled,
                    "Auto-Paste",
                    );

                    ui.checkbox(
                        &mut self.auto_copy_enabled,
                        "Auto-Copy",
                    );

                    ui.checkbox(
                        &mut self.auto_clear_enabled,
                        "Auto-Clear"
                    );

                    ui.checkbox(
                        &mut self.password_mode_enabled,
                        "Password Mode [Enhanced Security]"
                    );
                });
            }
        });
    });

        // sidepanel with settings, can optionally disable via checkbox
        if self.sidepanel_enabled {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
           
           // email hinting options
            ui.heading("Email Hinting Options");
            
            ui.vertical(|ui| {
                ui.separator();
            });
            
            // email hinting options go here \\
            ui.checkbox(
              &mut self.auto_paste_enabled,
              "Auto-Paste",
            );

            ui.checkbox(
                &mut self.auto_copy_enabled,
                "Auto-Copy",
            );

            ui.checkbox(
                &mut self.auto_clear_enabled,
                "Auto-Clear"
            );

            ui.checkbox(
                &mut self.password_mode_enabled,
                "Password Mode [Enhanced Security]"
            );
            
            //////////////////|\\\\\\\\\\\\\\\\\\
            


            // ticket tracking options
            //ui.heading("Ticket Tracking Options");

            //ui.vertical(|ui| {
            //    ui.separator();
            //});
            // ticket tracking options go here \\
            //ui.label("Test: Options");
            
            ///////////////////|\\\\\\\\\\\\\\\\\\\


        });
    }

        // central panel region, main screen
        egui::CentralPanel::default().show(ctx, |ui| {

            // text settings import
            use egui::{RichText, FontId, Color32};

            // header for central panel
            ui.label(RichText::new("Email Hinting").font(FontId::proportional(35.0)));
            ui.vertical(|ui| {
                ui.separator();
            });

            // Email hinting UI
            ui.horizontal(|ui| {
                ui.label(RichText::new("Email: ").font(FontId::proportional(25.0)));
                let email_text_box = ui.add(egui::TextEdit::singleline(label).hint_text("Example: abc.def@protonmail.com").password(self.password_mode_enabled));
                
                // auto-paste functionality
                if email_text_box.clicked() && label.is_empty() && self.auto_paste_enabled {
                    label.clear();
                    label.clone_from(&fn_paste());
                }

                // clear button functionality, will show if something is written in textbox only
                if !label.is_empty() {
                    if ui.button("Clear").clicked() {
                        label.clear();
                    }
                }   
            });

            // Email hint UI
            ui.horizontal(|ui| {
                ui.label(RichText::new("Hint: ").font(FontId::proportional(25.0)));
                let ehint = String::from(fn_hint(label.to_string()));

                // only show the ehint if it exists
                if !ehint.is_empty() {
                    // if the program caluculates incorrectly, produce an error message
                    if ehint.len() != label.len() && !ehint.contains("Error: Email is invalid; Cannot index NULL!") {
                        let ehint_label = ui.label(RichText::new("Critical Error: fn_hint miscalculation (For dev: Please check your math)").font(FontId::proportional(12.0)));
                    }
                    else {    
                let ehint_label = ui.label(RichText::new(&ehint).font(FontId::proportional(12.0)));
                    }
                }

                // otherwise a static message is in it's place for convenience
                else if ehint.is_empty() {
                    let ehint_label = ui.label(RichText::new("Waiting for Valid Email... ").font(FontId::proportional(12.0)).color(Color32::RED));
                }
                
                // copy button functionality, will appear when ehint is not empty only
                // Note: ehint is only referring to the data that has been hinted, not the label; ehint_label != ehint
                if !ehint.is_empty() && !ehint.contains("Error: Email is invalid; Cannot index NULL!") 
                                     && !ehint.contains("Critical Error: fn_hint miscalculation (For dev: Please check your math)") {
                    if ui.button("Copy to Clipboard").clicked() {
                        fn_copy(ehint);
                    }
                }
            });            

            // Ticket tracking UI
            ui.vertical(|ui| {
                ui.separator();
            });
            ui.label(RichText::new("Ticket Tracking").font(FontId::proportional(35.0)));
            
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.tick_escalated,
                    "Escalated",
                );

                ui.checkbox(
                    &mut self.tick_chat,
                    "Chat Ticket",
                );
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Ticket: ").font(FontId::proportional(25.0)));
                let ticket_text_box = ui.add(egui::TextEdit::singleline(ticket).hint_text("Example: #123456"));
                if ticket.len() >= 4 && ticket.contains('#') {
                    
                    // save button and enter key functionality
                    if ui.button("Save").clicked() || 
                    (ticket_text_box.lost_focus() 
                    && ticket_text_box.ctx.input().key_pressed(egui::Key::Enter)) {
                        // what to do when save ->
                        if !self.tick_chat && !self.tick_escalated {
                            self.reg_tick_set.push(ticket.to_string());
                            ticket.clear();
                        }
                        else if !self.tick_chat && self.tick_escalated {
                            self.reg_tick_set.push(ticket.to_string());
                            self.esc_tick_set.push(ticket.to_string());
                            ticket.clear();
                        }
                        else if self.tick_chat && !self.tick_escalated {
                            self.chat_tick_set.push(ticket.to_string());
                            ticket.clear();
                        }
                        else if self.tick_chat && self.tick_escalated {
                            self.chat_tick_set.push(ticket.to_string());
                            self.esc_tick_set.push(ticket.to_string());
                            ticket.clear();
                        }
                    }
                }
                else if ticket.len() >= 4 && !ticket.contains('#') {
                    ui.label(RichText::new(" Ticket needs to contain '#' symbol!" ).font(FontId::proportional(12.5)).color(Color32::RED));
                }
            });

            // Ticket info UI

            ui.vertical(|ui| {
                ui.separator();
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Regular Tickets: ").font(FontId::proportional(15.0)));
                if !self.reg_tick_set.is_empty() {
                    self.ticket_list_str = vec_iter_reg(&mut self.reg_tick_set.to_vec());
                    let tick_list = ui.label(RichText::new(self.ticket_list_str.to_string()).font(FontId::proportional(12.5)));
                }
                else {
                    let tick_list = ui.label(RichText::new("No Regular Tickets Saved.").font(FontId::proportional(12.5)));
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Copy to Clipboard").clicked() {
                    fn_copy(self.ticket_list_str.to_string());
                }

                if ui.button("Clear").clicked() {
                    self.show_clear_confirm = true;
                }
            });

            // regular ticket clear confirm
            if self.show_clear_confirm {
                egui::Window::new("Are you sure?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_clear_confirm = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.reg_tick_set.clear();
                            self.show_clear_confirm = false;
                        }
                    });
                });
            }

            ui.vertical(|ui| {
                ui.separator();
            });

            // chat ticket stuff

            ui.horizontal(|ui| {
                ui.label(RichText::new("Chat Tickets: ").font(FontId::proportional(15.0)));
                if !self.chat_tick_set.is_empty() {
                    self.chat_tick_list_str = vec_iter_reg(&mut self.chat_tick_set.to_vec());
                    let chat_tick_list = ui.label(RichText::new(self.chat_tick_list_str.to_string()).font(FontId::proportional(12.5)));
                }
                else {
                    let chat_tick_list = ui.label(RichText::new("No Chat Tickets Saved.").font(FontId::proportional(12.5)));
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Copy to Clipboard").clicked() {
                    fn_copy(self.chat_tick_list_str.to_string());
                }

                if ui.button("Clear").clicked() {
                    self.chat_show_clear = true;
                }
            });

            // chat ticket clear confirm
            if self.chat_show_clear {
                egui::Window::new("Are you sure?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.chat_show_clear = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.chat_tick_set.clear();
                            self.chat_show_clear = false;
                        }
                    });
                });
            }

            ui.vertical(|ui| {
                ui.separator();
            });

            // escalated ticket stuff

            ui.horizontal(|ui| {
                ui.label(RichText::new("Escalated Tickets: ").font(FontId::proportional(15.0)));
                if !self.esc_tick_set.is_empty() {
                    self.esc_tick_list_str = vec_iter_reg(&mut self.esc_tick_set.to_vec());
                    let esc_tick_list = ui.label(RichText::new(self.esc_tick_list_str.to_string()).font(FontId::proportional(12.5)));
                }
                else {
                    let esc_tick_list = ui.label(RichText::new("No Escalated Tickets Saved.").font(FontId::proportional(12.5)));
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Copy to Clipboard").clicked() {
                    fn_copy(self.esc_tick_list_str.to_string());
                }

                if ui.button("Clear").clicked() {
                    self.esc_show_clear = true;
                }
            });

            // escalate ticket clear confirm
            if self.esc_show_clear {
                egui::Window::new("Are you sure?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.esc_show_clear = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.esc_tick_set.clear();
                            self.esc_show_clear = false;
                        }
                    });
                });
            }

            ui.vertical(|ui| {
                ui.separator();
            });

            // ticket totals

            ui.horizontal(|ui| {
                ui.label(RichText::new("Amount of Regular Tickets: ").font(FontId::proportional(12.5)));
                ui.label(RichText::new(self.reg_tick_set.len().to_string()).font(FontId::proportional(12.5)));
                if ui.button("Copy Total").clicked() {
                    fn_copy(self.reg_tick_set.len().to_string());
                }
            });
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Amount of Chat Tickets: ").font(FontId::proportional(12.5)));
                ui.label(RichText::new(self.chat_tick_set.len().to_string()).font(FontId::proportional(12.5)));
                if ui.button("Copy Total").clicked() {
                    fn_copy(self.chat_tick_set.len().to_string());
                }
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Amount of Escalated Tickets: ").font(FontId::proportional(12.5)));
                ui.label(RichText::new(self.esc_tick_set.len().to_string()).font(FontId::proportional(12.5)));
                if ui.button("Copy Total").clicked() {
                    fn_copy(self.esc_tick_set.len().to_string());
                }
            });

            // debug warning
            egui::warn_if_debug_build(ui);

            // bottom signature
            if self.enable_signature {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Created by ");
                        ui.hyperlink_to("Dillan Cress", "https://github.com/Dcress1156");
                        ui.label(".");
                        });
                    });
            }   
        });


        
    }
}
