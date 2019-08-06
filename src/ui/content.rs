use gtk::*;

pub struct Content {
    pub container: Box,
}
impl Content {
    pub fn new() -> Content {
        let container = Box::new(Orientation::Vertical, 0);

        let calculation_pane = Box::new(Orientation::Horizontal, 2);
        let calc_entry = Entry::new();
        let calc_result = Label::new(Some("0"));

        calculation_pane.pack_start(&calc_entry, true, true, 0);
        calculation_pane.pack_start(&calc_result, true, false, 0);

        let inputs_pane = Grid::new();
        inputs_pane.set_column_homogeneous(true);
        inputs_pane.set_row_homogeneous(true);
        inputs_pane.set_column_spacing(2);
        inputs_pane.set_row_spacing(2);

        let n: Vec<&str> = vec![
            "1", "2", "3", "/", "4", "5", "6", "*", "7", "8", "9", "-", "0", ".", "=", "+",
        ];
        let mut n = n.iter();

        for i in 0..4 {
            for j in 0..4 {
                let token = n.next().unwrap();
                let btn = Button::new_with_label(token);

                inputs_pane.attach(&btn, j, i, 1, 1);
            }
        }

        container.pack_start(&calculation_pane, false, false, 0);
        container.pack_start(&inputs_pane, true, true, 0);

        Content { container }
    }
}
