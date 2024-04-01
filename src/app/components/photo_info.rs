// SPDX-FileCopyrightText: © 2024 David Bliss
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// Properties view for a photo.
/// Deeply inspired by how Loupe displays its property view.

use photos_core::Scanner;
use photos_core::scanner::Picture;
use gtk::prelude::OrientableExt;
use relm4::gtk;
use relm4::*;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::adw::prelude::ActionRowExt;
use relm4::gtk::prelude::WidgetExt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PhotoInfoInput {
    ShowInfo(PathBuf),
}

#[derive(Debug)]
pub struct PhotoInfo {
    scanner: Scanner,

    folder: adw::ActionRow,
}


#[relm4::component(pub)]
impl SimpleComponent for PhotoInfo {
    type Init = Scanner;
    type Input = PhotoInfoInput;
    type Output = ();

    view! {
       gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 10,

            adw::PreferencesGroup {

                #[local_ref]
                folder -> adw::ActionRow {
                    set_title: "Folder",
                    //set_subtitle: &model.path,
                    add_css_class: "property",
                    set_subtitle_selectable: true,
                },

                adw::ActionRow {
                    set_title: "Another Title",
                },
            }
        }
    }

    fn init(
        scanner: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let folder = adw::ActionRow::new();

        let model = PhotoInfo {
            scanner,
            folder: folder.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PhotoInfoInput::ShowInfo(ref path) => {
                println!("Received {:?}", msg);
                self.update_pic_info(path);
            }
        }
    }
}

/// Value row subtitle when value absent.
const FALLBACK: &str = "–";

impl PhotoInfo {

    fn update_pic_info(&mut self, path: &PathBuf) {
        let result = self.scanner.scan_one(path);
        let Ok(pic) = result else {
            println!("Failed scanning picture: {:?}", result);
            return;
        };

        Self::update_row(&self.folder, Self::folder_name(path));
    }

    /// Borrowed from Loupe.
    /// Updates a row to be visible if it has a value to display, and returns
    /// visibility status.
    fn update_row(row: &adw::ActionRow, value: Option<impl AsRef<str>>) -> bool {
        if let Some(value) = value {
            row.set_subtitle(value.as_ref());
            row.set_visible(true);
            true
        } else {
            row.set_subtitle(FALLBACK);
            row.set_visible(false);
            false
        }
    }

    fn folder_name(path: &PathBuf) -> Option<String> {
        path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy())
            .map(|n| n.to_string())
    }

}