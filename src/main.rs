// use std::collections::HashMap;
// use std::rc::Weak;

// trait EventSubscriber {
//     fn notify(&self, event: &str, message: &str);
// }

// struct EventDispatcher {
//     subs: HashMap<String, Vec<Weak<dyn EventSubscriber>>>,
// }

// impl EventDispatcher {
//     fn new() -> Self {
//         EventDispatcher {
//             subs: HashMap::new(),
//         }
//     }

//     fn subscribe(&mut self, subscriber: Weak<dyn EventSubscriber>, event: &str) {
//         let item = self.subs.get_mut(event);
//         match item {
//             Some(entry) => entry.push(subscriber),
//             None => {
//                 let mut vec = Vec::<Weak<dyn EventSubscriber>>::new();
//                 vec.push(subscriber);
//                 self.subs.insert(event.to_string(), vec);
//             }
//         }
//     }

//     fn notify(&self, event: &str, _message: &str) {
//         let subscribers = self.subs.get(event);
//         match subscribers {
//             Some(subs) => {
//                 for sub in subs {
//                     let s = sub.upgrade();
//                     match s {
//                         Some(subscriber) => subscriber.notify("event", "message"),
//                         None => (),
//                     }
//                 }
//             }
//             None => (),
//         }
//     }
// }

// struct AppEventListener {}

// // impl AppEventListener {
// //     fn new() -> Self {}
// // }

// impl EventSubscriber for AppEventListener {
//     fn notify(&self, event: &str, message: &str) {
//         match event {
//             "quit" => println!("QUIT THE APPLICATION"),
//             _ => println!("Unknown event {}", event),
//         }
//     }
// }

use sfml::{
    graphics::{
        Color, Drawable, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
    SfBox,
};

struct MenuItem {
    label: String,
    font: SfBox<Font>,
    text_size: u32,
    position: u32,
    selected: bool,
    action: fn(&str),
}

impl MenuItem {
    fn new(label: &str, text_size: u32, position: u32, selected: bool) -> Self {
        let font = Font::from_file("./resources/fonts/Pacmania.otf").unwrap();
        MenuItem {
            label: label.to_owned(),
            font: font.to_owned(),
            text_size: text_size,
            position: position,
            selected: selected,
            action: |label| println!("{} item action", label),
        }
    }

    fn get_height(&self) -> f32 {
        self.font.line_spacing(self.text_size)
    }

    fn on_enter(&self) {
        (self.action)(&self.label);
    }
}

impl Drawable for MenuItem {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut message = Text::new(&self.label, &self.font, self.text_size);
        match self.selected {
            true => message.set_fill_color(Color::YELLOW),
            false => message.set_fill_color(Color::WHITE),
        }
        let position = Vector2f::new(
            ((target.size().x - message.global_bounds().width as u32) / 2) as f32,
            self.position as f32,
        );
        message.set_position(position);
        target.draw(&message);
    }
}

struct Menu {
    items: Vec<MenuItem>,
    cursor_position: usize,
}

impl Menu {
    fn new() -> Self {
        let mut items = Vec::new();
        items.push(MenuItem::new("New game", 64, 0, true));
        items.push(MenuItem::new(
            "High score",
            64,
            items[0].get_height() as u32,
            false,
        ));
        items.push(MenuItem::new(
            "Exit",
            64,
            (items[0].get_height() * 2.) as u32,
            false,
        ));
        Menu {
            items: items,
            cursor_position: 0,
        }
    }

    fn cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position -= 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    fn cursor_down(&mut self) {
        if self.cursor_position < self.items.len() - 1 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position += 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    fn select_action(&self) {
        self.items[self.cursor_position].on_enter();
    }
}

impl Drawable for Menu {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for item in &self.items {
            item.draw(target, states);
        }
    }
}

struct PacMan {
    window: RenderWindow,
    menu: Menu,
    quit_loop: bool,
    // event_dispatcher: EventDispatcher,
    // event_listener: Rc<AppEventListener>,
}

impl PacMan {
    fn new() -> Self {
        PacMan {
            window: RenderWindow::new((800, 600), "PAC-MAN", Style::CLOSE, &Default::default()),
            menu: Menu::new(),
            quit_loop: false,
            // event_dispatcher: EventDispatcher::new(),
            // event_listener: AppEventListener {}.into(),
        }
    }

    fn setup(&mut self) {
        self.window.set_vertical_sync_enabled(true);
        // self.event_dispatcher
        //     .subscribe(Rc::downgrade(&self.event_listener), "quit_app")
    }

    fn run(&mut self) {
        loop {
            while let Some(event) = &self.window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => self.quit(),
                    Event::KeyPressed { code: Key::Up, .. } => self.menu.cursor_up(),
                    Event::KeyPressed {
                        code: Key::Down, ..
                    } => self.menu.cursor_down(),
                    Event::KeyPressed {
                        code: Key::Enter, ..
                    } => self.menu.select_action(),
                    _ => {}
                }
                if self.quit_loop {
                    return;
                }
            }

            self.window.clear(Color::BLACK);
            self.window.draw(&self.menu);
            self.window.display()
        }
    }

    fn quit(&mut self) {
        self.quit_loop = true;
    }
}

fn main() {
    let mut pac_man = PacMan::new();
    pac_man.setup();
    pac_man.run();
}