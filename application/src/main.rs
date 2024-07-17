// Derive traits
use druid::{widget::{Button, Flex, Label}, AppLauncher, Data, Env, Widget, WindowDesc};

#[derive(Clone, Data)]
struct RandomData {
    num : i32
}

fn ui_builder() -> impl Widget<RandomData> {
    let label = Label::new(|data: &RandomData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+").on_click(|_ctx, data: &mut RandomData, _:&Env| data.num += 1);
    let decrement = Button::new("-").on_click(|_ctx, data: &mut RandomData, _:&Env| {
        if data.num > 0 {
            data.num -=1
        }
    } );

    // return the basic structure
    Flex::column().with_child(label).with_child(Flex::row().with_child(increment).with_child(decrement))
}

fn main () {
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)
    .log_to_console().launch(RandomData{num:0}).unwrap();
}