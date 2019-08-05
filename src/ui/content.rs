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

        calculation_pane.pack_start(&calc_entry, true, false, 0);
        calculation_pane.pack_start(&calc_result, false, false, 0);

        let inputs_pane = Grid::new();
        inputs_pane.set_column_homogeneous(true);
        inputs_pane.set_row_homogeneous(true);

        let zero = Button::new_with_label("0");
        let one = Button::new_with_label("1");
        let two = Button::new_with_label("2");
        let three = Button::new_with_label("3");
        let four = Button::new_with_label("4");
        let five = Button::new_with_label("5");
        let six = Button::new_with_label("6");
        let seven = Button::new_with_label("7");
        let eight = Button::new_with_label("8");
        let nine = Button::new_with_label("9");

        // gtk::GridExt::attach(&child, left, top, width, height)
        inputs_pane.attach(&one, 0, 0, 1, 1);
        inputs_pane.attach(&two, 1, 0, 1, 1);
        inputs_pane.attach(&three, 2, 0, 1, 1);

        inputs_pane.attach(&four, 0, 1, 1, 1);
        inputs_pane.attach(&five, 1, 1, 1, 1);
        inputs_pane.attach(&six, 2, 1, 1, 1);

        inputs_pane.attach(&seven, 0, 2, 1, 1);
        inputs_pane.attach(&eight, 1, 2, 1, 1);
        inputs_pane.attach(&nine, 2, 2, 1, 1);

        inputs_pane.attach(&zero, 1, 3, 1, 1);

        container.pack_start(&calculation_pane, false, false, 0);
        container.pack_start(&inputs_pane, true, true, 0);

        Content { container }
    }
}
