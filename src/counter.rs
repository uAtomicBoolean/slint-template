use crate::app;
use crate::app::{ElmModel, Outcome};
use crate::{CounterIntents, CounterState, SlintTemplate};

use slint::Global;

pub enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Into<app::Message> for Message {
    fn into(self) -> app::Message {
        app::Message::Counter(self)
    }
}

#[derive(Default)]
pub struct State {
    count: i32,
}

impl ElmModel for State {
    type Message = Message;
    type Ui = SlintTemplate;

    fn update(&mut self, msg: Self::Message) -> Outcome {
        match msg {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
        };
        Ok(())
    }

    fn view(&self, ui: &Self::Ui) {
        let counter_state = CounterState::get(&ui);
        counter_state.set_count(self.count);
    }

    fn bind(&self, ui: &Self::Ui, dispatch: impl Fn(Self::Message) -> Outcome + Clone + 'static) {
        let intents = CounterIntents::get(&ui);
        let d = dispatch.clone();
        intents.on_increment(move || {
            let _ = d(Message::Increment);
        });
        let d = dispatch.clone();
        intents.on_decrement(move || {
            let _ = d(Message::Decrement);
        });
        let d = dispatch.clone();
        intents.on_reset(move || {
            let _ = d(Message::Reset);
        });
    }
}
