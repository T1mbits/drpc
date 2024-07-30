use crate::as_string;
use discord_rich_presence::activity::{Activity, Assets, Button};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub trait Template<'a, A> {
    /// Convert the template into the actual struct.
    fn evaluate(&'a self) -> A;
    /// Run a function over every `Option<String>` field in the struct. Should also call foreach_field on any other fields implementing `Template`.
    fn foreach_field<F>(&mut self, func: F)
    where
        F: Fn(&mut Option<String>);
    /// Whether or not every Option field in the struct is None
    fn is_empty(&self) -> bool;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub activity: ActivityTemplate,
    // pub processes: ProcessesConfig,
}

/// A struct closely mirroring [`discord_rich_presence::activity::Activity`], but can be both serialized and deserialized for storage in a config file.
///
/// May also have text templates such as {{process.name}} that needs to be evaluated before being converted to [`discord_rich_presence::activity::Activity`]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ActivityTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<AssetsTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<ButtonsTemplate>,
    // TODO add timestamps
    // pub timestamps: TimestampsTemplate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl<'a> Template<'a, Activity<'a>> for ActivityTemplate {
    fn evaluate(&self) -> Activity {
        let mut activity = Activity::new();

        if let Some(assets) = &self.assets {
            activity = activity.assets(assets.evaluate());
        }
        if let Some(buttons) = &self.buttons {
            activity = activity.buttons(buttons.evaluate());
        }
        if let Some(details) = &self.details {
            activity = activity.details(details);
        }
        if let Some(state) = &self.state {
            activity = activity.state(state);
        }

        activity
    }

    fn foreach_field<F>(&mut self, func: F)
    where
        F: Fn(&mut Option<String>),
    {
        if let Some(assets) = self.assets.as_mut() {
            assets.foreach_field(&func);
        }
        if let Some(buttons) = self.buttons.as_mut() {
            buttons.foreach_field(&func);
        }
        func(&mut self.details);
        func(&mut self.state);
    }

    fn is_empty(&self) -> bool {
        self.assets.is_none()
            && self.buttons.is_none()
            && self.details.is_none()
            && self.state.is_none()
    }
}

impl Display for ActivityTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "details: {}\nstate: {}\nassets: \n\t{}\nbuttons: \n\t{}",
            as_string(&self.details),
            as_string(&self.state),
            as_string(&self.assets),
            as_string(&self.buttons),
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AssetsTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

impl<'a> Template<'a, Assets<'a>> for AssetsTemplate {
    fn evaluate(&self) -> Assets {
        let mut assets = Assets::new();

        if let Some(large_image) = &self.large_image {
            assets = assets.large_image(large_image);
        }
        if let Some(large_text) = &self.large_text {
            assets = assets.large_text(large_text);
        }
        if let Some(small_image) = &self.small_image {
            assets = assets.small_image(small_image);
        }
        if let Some(small_text) = &self.small_text {
            assets = assets.small_text(small_text);
        }

        assets
    }

    fn foreach_field<F>(&mut self, func: F)
    where
        F: Fn(&mut Option<String>),
    {
        func(&mut self.large_image);
        func(&mut self.large_text);
        func(&mut self.small_image);
        func(&mut self.small_text);
    }

    fn is_empty(&self) -> bool {
        self.large_image.is_none()
            && self.large_text.is_none()
            && self.small_image.is_none()
            && self.small_text.is_none()
    }
}

impl Display for AssetsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "large image: {}\n\tlarge image text: {}\n\tsmall image: {}\n\tsmall image text: {}",
            as_string(&self.large_image),
            as_string(&self.large_text),
            as_string(&self.small_image),
            as_string(&self.small_text),
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ButtonsTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url2: Option<String>,
}

impl<'b> Template<'b, Vec<Button<'b>>> for ButtonsTemplate {
    fn evaluate(&self) -> Vec<Button> {
        let mut buttons: Vec<Button> = Vec::new();

        if let (Some(label), Some(url)) = (&self.label1, &self.url1) {
            buttons.push(Button::new(label, url));
        }
        if let (Some(label), Some(url)) = (&self.label2, &self.url2) {
            buttons.push(Button::new(label, url));
        }

        buttons
    }
    fn foreach_field<F>(&mut self, func: F)
    where
        F: Fn(&mut Option<String>),
    {
        func(&mut self.label1);
        func(&mut self.url1);
        func(&mut self.label2);
        func(&mut self.url2);
    }

    fn is_empty(&self) -> bool {
        self.label1.is_none() && self.url1.is_none() && self.label2.is_none() && self.url2.is_none()
    }
}

impl Display for ButtonsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "button 1 label: {}\n\tbutton 1 url: {}\n\tbutton 2 label: {}\n\tbutton 2 url: {}",
            as_string(&self.label1),
            as_string(&self.url1),
            as_string(&self.label2),
            as_string(&self.url2),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcessesConfig {
    pub idle_image: String,
    pub idle_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<ProcessConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcessConfig {
    pub image: String,
    pub name: String,
    pub text: String,
}
