// SPDX-FileCopyrightText: © 2024 David Bliss
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// Properties view for a photo.
/// Deeply inspired by how Loupe displays its property view.

use fotema_core::photo;
use fotema_core::Library;
use fotema_core::VisualId;
use gtk::prelude::OrientableExt;

use relm4::gtk;
use relm4::*;
use relm4::adw::prelude::*;
use std::path::PathBuf;
use humansize::{format_size, DECIMAL};
use glycin::{ImageInfo, ImageInfoDetails};
use std::fs;
use std::sync::Arc;
use chrono::prelude::*;
use chrono::format::*;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};


#[derive(Debug)]
pub enum PhotoInfoInput {
    Photo(VisualId, ImageInfo),
    Video(VisualId),
}

pub struct PhotoInfo {
    photo_scan: photo::Scanner,
    library: Library,

    folder: adw::ActionRow,

    // FIXME what timestamps to show for live photos that have an image an a video?
    date_time_details: adw::PreferencesGroup,
    created_at: adw::ActionRow,
    modified_at: adw::ActionRow,

    image_details: adw::PreferencesGroup,
    image_size: adw::ActionRow,
    image_format: adw::ActionRow,
    image_file_size: adw::ActionRow,

    exif_details: adw::PreferencesGroup,
    exif_originally_created_at: adw::ActionRow,
    exif_originally_modified_at: adw::ActionRow,

    video_details: adw::PreferencesGroup,
    video_container_format: adw::ActionRow,
    video_file_size: adw::ActionRow,
    video_originally_created_at: adw::ActionRow,
    video_duration: adw::ActionRow,
}


#[relm4::component(pub)]
impl SimpleComponent for PhotoInfo {
    type Init = (Library, photo::Scanner);
    type Input = PhotoInfoInput;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 12,

                adw::PreferencesGroup {
                    #[local_ref]
                    folder -> adw::ActionRow {
                        set_title: "Folder",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },
                },

                #[local_ref]
                date_time_details -> adw::PreferencesGroup {
                    #[local_ref]
                    created_at -> adw::ActionRow {
                        set_title: "File Created",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    modified_at -> adw::ActionRow {
                        set_title: "File Modified",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },
                },

                #[local_ref]
                image_details -> adw::PreferencesGroup {
                    #[local_ref]
                    image_size -> adw::ActionRow {
                        set_title: "Image Size",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    image_format -> adw::ActionRow {
                        set_title: "Image Format",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    image_file_size -> adw::ActionRow {
                        set_title: "File Size",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },
                },

                #[local_ref]
                exif_details -> adw::PreferencesGroup {
                    #[local_ref]
                    exif_originally_created_at -> adw::ActionRow {
                        set_title: "Originally Created",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    exif_originally_modified_at -> adw::ActionRow {
                        set_title: "Originally Modified",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },
                },


                #[local_ref]
                video_details -> adw::PreferencesGroup {
                    #[local_ref]
                    video_duration -> adw::ActionRow {
                        set_title: "Duration",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    video_file_size -> adw::ActionRow {
                        set_title: "File Size",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    video_originally_created_at -> adw::ActionRow {
                        set_title: "Originally Created",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },

                    #[local_ref]
                    video_container_format -> adw::ActionRow {
                        set_title: "Container Format",
                        add_css_class: "property",
                        set_subtitle_selectable: true,
                    },
                },
            }
        }
    }

    fn init(
        (library, photo_scan): Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let folder = adw::ActionRow::new();

        let date_time_details = adw::PreferencesGroup::new();
        let created_at = adw::ActionRow::new();
        let modified_at = adw::ActionRow::new();

        let image_details = adw::PreferencesGroup::new();
        let image_size = adw::ActionRow::new();
        let image_format = adw::ActionRow::new();
        let image_file_size = adw::ActionRow::new();

        let exif_details = adw::PreferencesGroup::new();
        let exif_originally_created_at = adw::ActionRow::new();
        let exif_originally_modified_at = adw::ActionRow::new();

        let video_details = adw::PreferencesGroup::new();
        let video_duration = adw::ActionRow::new();
        let video_container_format = adw::ActionRow::new();
        let video_file_size = adw::ActionRow::new();
        let video_originally_created_at = adw::ActionRow::new();

        let model = PhotoInfo {
            library,
            photo_scan,
            folder: folder.clone(),

            date_time_details: date_time_details.clone(),
            created_at: created_at.clone(),
            modified_at: modified_at.clone(),

            image_details: image_details.clone(),
            image_size: image_size.clone(),
            image_format: image_format.clone(),
            image_file_size: image_file_size.clone(),

            exif_details: exif_details.clone(),
            exif_originally_created_at: exif_originally_created_at.clone(),
            exif_originally_modified_at: exif_originally_modified_at.clone(),

            video_details: video_details.clone(),
            video_file_size: video_file_size.clone(),
            video_originally_created_at: video_originally_created_at.clone(),
            video_duration: video_duration.clone(),
            video_container_format: video_container_format.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PhotoInfoInput::Photo(visual_id, ref image_info) => {
                println!("Received {:?}", msg);
                let result = self.library.get(visual_id);
                let Some(ref vis) = result else {
                    println!("No visual item");
                    return;
                };

                self.update_file_details(vis.clone());

                if vis.picture_id.is_some() {
                    self.update_photo_details(vis.clone(), image_info);
                }

                if vis.video_id.is_some() {
                    self.update_video_details(vis.clone());
                }
            }
            PhotoInfoInput::Video(visual_id) => {
                println!("Received {:?}", msg);
                let result = self.library.get(visual_id);
                let Some(ref vis) = result else {
                    println!("No visual item");
                    return;
                };

                self.image_details.set_visible(false);
                self.exif_details.set_visible(false);

                self.update_file_details(vis.clone());

                if vis.video_id.is_some() {
                    self.update_video_details(vis.clone());
                }
            }
        }
    }
}

/// Value row subtitle when value absent.
const FALLBACK: &str = "–";

impl PhotoInfo {

    fn update_file_details(&mut self, vis: Arc<fotema_core::visual::Visual>) -> Result<(), String> {
        let Some(ref path) = vis.path() else {
            return Err("No picture or video path".to_string());
        };

        Self::update_row(&self.folder, Self::folder_name(&vis.parent_path));

        // FIXME duplicated from Scanner
        let file = fs::File::open(path).map_err(|e| e.to_string())?;

        let metadata = file.metadata().map_err(|e| e.to_string())?;

        let fs_created_at: Option<String> = metadata
            .created()
            .map(|x| Into::<DateTime<Utc>>::into(x))
            .map(|x| x.format("%Y-%m-%d %H:%M:%S %:z").to_string())
            .map_err(|e| e.to_string())
            .ok();


        let fs_modified_at: Option<String> = metadata
            .modified()
            .map(|x| Into::<DateTime<Utc>>::into(x))
            .map(|x| x.format("%Y-%m-%d %H:%M:%S %:z").to_string())
            .map_err(|e| e.to_string())
            .ok();

        let fs_file_size_bytes = metadata.len();

        let has_date_time_details = [
            Self::update_row(&self.created_at, fs_created_at),
            Self::update_row(&self.modified_at, fs_modified_at),
        ]
        .into_iter()
        .any(|x| x);

        self.date_time_details.set_visible(has_date_time_details);

        Ok(())
    }

    fn update_photo_details(&mut self, vis: Arc<fotema_core::visual::Visual>, image_info: &ImageInfo) -> Result<(), String> {
        let Some(ref picture_path) = vis.picture_path else {
            return Err("No picture path".to_string());
        };

        // FIXME duplicated from Scanner
        let file = fs::File::open(picture_path).map_err(|e| e.to_string())?;
        let metadata = file.metadata().map_err(|e| e.to_string())?;

        let fs_file_size_bytes = metadata.len();

        let image_size = format!("{} x {}", image_info.width, image_info.height);

        let has_image_details = [
            Self::update_row(&self.image_size, Some(image_size)),
            Self::update_row(&self.image_format, image_info.details.format_name.as_ref()),
            Self::update_row(&self.image_file_size, Some(format_size(fs_file_size_bytes, DECIMAL))),
        ]
        .into_iter()
        .any(|x| x);

        self.image_details.set_visible(has_image_details);

        if let Some(Ok(exif)) = image_info.details.exif.as_ref().map(|x| x.get_full()) {
            let metadata = fotema_core::photo::Metadata::from(exif).ok();

            let created_at: Option<String> = metadata
                .clone()
                .and_then(|x| x.created_at)
                .map(|x| x.format("%Y-%m-%d %H:%M:%S %:z").to_string());

            let modified_at: Option<String> = metadata
                .clone()
                .and_then(|x| x.modified_at)
                .map(|x| x.format("%Y-%m-%d %H:%M:%S %:z").to_string());

            let has_exif_details = [
                Self::update_row(&self.exif_originally_created_at, created_at),
                Self::update_row(&self.exif_originally_modified_at, modified_at),
            ]
            .into_iter()
            .any(|x| x);

            self.exif_details.set_visible(has_exif_details);
        } else {
            self.exif_details.set_visible(false);
        }

        Ok(())
    }

    fn update_video_details(&mut self, vis: Arc<fotema_core::visual::Visual>) -> Result<(), String> {
        let Some(ref video_path) = vis.video_path else {
            return Err("No video path".to_string());
        };

        // FIXME duplicated from Scanner
        //let file = fs::File::open(video_path).map_err(|e| e.to_string())?;

        //let video_size = format!("{} x {}", image_info.width, image_info.height);

        let metadata = fotema_core::video::Metadata::from(video_path).ok();
        println!("video meta = {:?}", metadata);

        let created_at: Option<String> = metadata
            .as_ref()
            .and_then(|x| x.created_at)
            .map(|x| x.format("%Y-%m-%d %H:%M:%S %:z").to_string());

        let duration = metadata
            .as_ref()
            .and_then(|x| x.duration)
            .map(|x| x.to_string());

        let container_format = metadata.and_then(|x| x.container_format);

        let has_video_details = [
            Self::update_row(&self.video_originally_created_at, created_at),
            Self::update_row(&self.video_duration, duration),
            //Self::update_row(&self.image_size, Some(image_size)),
            Self::update_row(&self.video_container_format, container_format),
            //Self::update_row(&self.image_file_size, Some(format_size(fs_file_size_bytes, DECIMAL))),
        ]
        .into_iter()
        .any(|x| x);

        self.video_details.set_visible(has_video_details);

        Ok(())
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
