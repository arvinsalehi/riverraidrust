use ratatui::widgets::{ListItem, ListState};



#[derive(Default)]
pub struct Events {
	pub state: ListState,
	// pub items: Vec<ListItem<'a>>,
	pub items: Vec<String>,
}


// impl<'a> Events<'a> {
impl<'a> Events {
	
	pub fn new(items: Vec<String>) -> Self {
		Self {
			state: ListState::default(),
			items,
		}
	}

	pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
    }


	pub fn first(&mut self) {
		self.state.select(Some(0));
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len().saturating_sub(1) {
					0
				} else {
					i.saturating_add(1)
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len().saturating_sub(1)
				} else {
					i.saturating_sub(1)
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}
}