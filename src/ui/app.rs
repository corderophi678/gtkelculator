use super::content::Content;
use super::header::Header;
use gtk::*;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("GTKelculator");
        window.set_default_size(300, 500);
        Window::set_default_icon_name("iconname");
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }
}
