// SPDX-FileCopyrightText: © 2024 David Bliss
//
// SPDX-License-Identifier: GPL-3.0-or-later

use relm4::prelude::*;
use relm4::Worker;
use std::sync::{Arc, Mutex};
use photos_core::Result;

#[derive(Debug)]
pub enum GeneratePreviewsInput {
    Generate,
}

#[derive(Debug)]
pub enum GeneratePreviewsOutput {
    PreviewsGenerated,
}

pub struct GeneratePreviews {
    scan: photos_core::Scanner,
    previewer: photos_core::Previewer,
    repo: Arc<Mutex<photos_core::Repository>>,
}

impl GeneratePreviews {

    fn update_previews(&self) -> Result<()> {
        let pics = self.repo.lock().unwrap().all()?;

        let pics = pics
            .into_iter()
            .filter(|p| p.square_preview_path.is_none())
            .collect::<Vec<photos_core::repo::Picture>>();

        for mut pic in pics {
            let result = self.previewer.set_preview(&mut pic);
            if let Err(e) = result {
                println!("Failed set_preview: {:?}", e);
                continue;
            }

            self.repo.lock().unwrap().add_preview(&pic)?;

        }

        Ok(())
    }
}

impl Worker for GeneratePreviews {
    type Init = (photos_core::Scanner, photos_core::Previewer, Arc<Mutex<photos_core::Repository>>);
    type Input = GeneratePreviewsInput;
    type Output = GeneratePreviewsOutput;

    fn init((scan, previewer, repo): Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self { scan, previewer, repo }
    }

    fn update(&mut self, msg: GeneratePreviewsInput, sender: ComponentSender<Self>) {
        match msg {
            GeneratePreviewsInput::Generate => {
                println!("Generating previews...");

                self.update_previews();

                sender.output(GeneratePreviewsOutput::PreviewsGenerated);
            }
        };
    }
}
