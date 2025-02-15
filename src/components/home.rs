use color_eyre::eyre::Ok;
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use std::{collections::HashMap, os::macos::raw::stat};
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::info;

use super::Component;
use crate::components::widgets::Events;
use crate::{action::Action, config::Config, tui::Event};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    events: Events,
}

impl Home {
    pub fn new() -> Result<Self> {
        let mut home = Self::default();
        home.set_items();
        Ok(home)
    }

    pub fn set_items(&mut self) {
        self.events = Events::new(vec![String::from("item 1"), String::from("item 2")]);
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        let action = match event {
            Some(Event::Key(key_event)) => self.handle_key_event(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event)?,
            _ => None,
        };
        Ok(action)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up => {
                self.events.previous();
            }
            KeyCode::Down => {
                self.events.next();
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {

        let items: Vec<ListItem> = self
            .events
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if Some(i) == self.events.state.selected() {
                    Style::default()
                        .fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new::<&str>(item.as_ref()).style(style)
            })
            .collect();
        
        let list = List::new(items);

        frame.render_stateful_widget(list.clone(), area, &mut self.events.state.clone());
        Ok(())
    }
}
