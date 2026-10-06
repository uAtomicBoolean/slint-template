mod elm_model;
mod types;

pub use elm_model::ElmModel;
pub use types::Outcome;

use crate::SlintTemplate;
use crate::counter;

use slint::ComponentHandle;

use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

// MULTI-THREADING: Used to send message into the elm loop when working in another thread.
thread_local! {
    static DISPATCH: OnceCell<Rc<dyn Fn(Message) -> Outcome>> = const { OnceCell::new() };
}

pub enum Message {
    Counter(counter::Message),
}

#[derive(Default)]
pub struct App {
    counter: counter::State,
}

impl App {
    pub fn init(self, ui: &SlintTemplate) {
        // Required to be able to call app in the dispatch closure.
        let app = Rc::new(RefCell::new(self));

        let dispatch = {
            let ui = ui.as_weak();
            let app = app.clone();
            move |msg| {
                let Some(ui) = ui.upgrade() else {
                    return Ok(());
                };
                let mut app = app.borrow_mut();
                let outcome = app.update(msg);
                app.view(&ui);
                outcome
            }
        };

        // MULTI-THREADING:
        DISPATCH.with(|d| {
            let _ = d.set(Rc::new(dispatch.clone()));
        });

        {
            let app = app.borrow();
            app.bind(&ui, dispatch);
            app.view(&ui);
        }
    }

    /// MULTI-THREADING: Send a message into the elm loop from any thread. The `update` method
    /// is then called from the UI thread, which means that the `Outcome is lost.`
    /// Only used with the `App::send()` method.
    pub fn send(msg: impl Into<Message>) -> Result<(), slint::EventLoopError> {
        let msg = msg.into();
        slint::invoke_from_event_loop(move || {
            DISPATCH.with(|dispatch| {
                if let Some(dispatch) = dispatch.get() {
                    let _ = dispatch(msg);
                }
            });
        })
    }
}

impl ElmModel for App {
    type Message = Message;
    type Ui = SlintTemplate;

    fn update(&mut self, msg: Self::Message) -> Outcome {
        match msg {
            Message::Counter(msg) => self.counter.update(msg),
        }
    }

    fn view(&self, ui: &Self::Ui) {
        self.counter.view(&ui);
    }

    fn bind(&self, ui: &Self::Ui, dispatch: impl Fn(Self::Message) -> Outcome + Clone + 'static) {
        let d = dispatch.clone();
        self.counter.bind(&ui, move |msg| d(Message::Counter(msg)));
    }
}
