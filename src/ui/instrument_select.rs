use crate::{Message, State};
use iced::widget::{ComboBox, combo_box};

pub fn render_select(state: &State) -> ComboBox<String, Message> {
    let combo_box = combo_box(
        &state.symbol_select_state,
        "Select a crypto a pair...",
        state.selected_symbol.as_ref(),
        Message::AddSymbol,
    )
    .on_input(Message::FilterInput);

    combo_box
}
