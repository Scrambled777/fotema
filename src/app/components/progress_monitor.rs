// SPDX-FileCopyrightText: © 2024 David Bliss
//
// SPDX-License-Identifier: GPL-3.0-or-later

use relm4::Reducible;

/// Media types
#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Photo,
    Video,
}

/// Different kinds of background task that have a progress bar
/// Note that some background tasks just have the banner and spinner.
#[derive(Debug, Clone, Copy)]
pub enum TaskName {
    Thumbnail(MediaType),
    Transcode,

    /// FIXME figure out if 'Idle' will be used.
    Idle,
}

#[derive(Debug)]
pub enum ProgressMonitorInput {
    Start(TaskName, usize),
    Advance,
    Complete,

    /// FIXME figure out if 'Idle' will be used.
    Idle,
}

/// Monitors the progress of a task and informs subscribers about changes.
pub struct ProgressMonitor {
    // Background task progress is for. None if idle.
    pub task_name: TaskName,

    /// Current progress
    pub current_count: usize,

    // Final progress
    end_count: usize,
}

impl ProgressMonitor {
    pub fn fraction(&self) -> f64 {
        self.current_count as f64 / self.end_count as f64
    }
}

impl Reducible for ProgressMonitor {
    type Input = ProgressMonitorInput;

    fn init() -> Self {
        Self {task_name: TaskName::Idle, current_count: 0, end_count: 0}
    }

    fn reduce(&mut self, input: Self::Input) -> bool {
        match input {
            ProgressMonitorInput::Start(task_name, end_count) => {
                self.task_name = task_name;
                self.end_count = end_count;
                self.current_count = 0;
            }
            ProgressMonitorInput::Advance =>  {
                self.current_count += 1;
            }
            ProgressMonitorInput::Complete =>  {
                self.current_count = self.end_count;
            }
            ProgressMonitorInput::Idle =>  {
                self.task_name = TaskName::Idle;
                self.end_count = 0;
                self.current_count = 0;
            }
        }
        true // subscribers only notified if 'true' is returned
    }
}

