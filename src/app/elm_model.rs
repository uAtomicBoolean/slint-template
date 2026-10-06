use crate::app::types::Outcome;

/// Define the contract to be used by an ElmModel.
pub trait ElmModel: Default {
    /// The model's messages.
    type Message;
    /// Slint component on which the model will display its data.
    /// It could be the main window, which is then could be used to get the globals.
    type Ui;

    fn new() -> Self {
        Self::default()
    }

    /// Applies a message to the model's state.
    /// This is where the application's logic resides or is linked.
    /// However, it does not touch the UI at all.
    fn update(&mut self, msg: Self::Message) -> Outcome;

    /// Update the UI with the current state.
    fn view(&self, ui: &Self::Ui);

    /// Called once to link the UI's callbacks to the dispatch.
    fn bind(&self, ui: &Self::Ui, dispatch: impl Fn(Self::Message) -> Outcome + Clone + 'static);
}
