#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    text: String,
    socket: TcpStream, 
    messages: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let sock = TcpStream::connect("127.0.0.1:8000").unwrap();
        sock.set_nonblocking(true).unwrap();
        
        Self {
            text: String::from(""),
            socket: sock,
            messages: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Client {}", self.socket.local_addr().unwrap().to_string()));
            let response = ui.add(egui::TextEdit::singleline(&mut self.text));

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                println!("{}", self.text);
                match self.socket.write(format!("{}: {}", self.socket.local_addr().unwrap().port(), self.text).as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                self.text = String::from("");
            }

            let text: &mut [u8] = &mut [0; 127];
            match self.socket.read(text) {
                Ok(_) => {
                    self.messages.push(std::str::from_utf8(text).unwrap().to_owned());
                }
                Err(_) => {}
            }

            for message in &self.messages {
                ui.label(format!("{}", message));
            }

            ctx.request_repaint();
        });
    }
}
