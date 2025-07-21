/*!
This crate integrates [`winit`] into the Magma API in order to manage application windows.

# Example

```
# use std::error::Error;
# use magma_app::{magma_ecs::entities::Entity, App, SystemType, World};
# use magma_windowing::Window;
# use magma_winit::WinitModule;
fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.add_module(WinitModule);
    // Add the system to close created windows.
    // Windows should not be closed in a startup system, bc it might cause the app to hang.
    app.add_systems(SystemType::Update, &[(close_windows, "close_windows", &[])]);
    // create a window
    // The winit module will create a single window on startup. That means there will now be two.
    app.world.create_entity((Window::new().with_title("test"),))?;
    app.run();
    Ok(())
}

// system for closing the opened windows
fn close_windows(world: &World) {
    // close windows
    world
        .query::<(Window,)>()
        .unwrap()
        .iter()
        .for_each(|window| window.delete());
}
```
*/

use magma_app::{App, events::Events, module::Module};
use magma_input::input_event::{
    KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput,
};
use magma_math::{IVec2, UVec2, Vec2};
use magma_windowing::monitor::VideoMode;
use magma_windowing::window::WindowTheme;
use magma_windowing::{ClosingWindow, Monitor, PrimaryMonitor, window_event::*};
use magma_windowing::{Window, WindowingModule};
use windows::Windows;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

mod systems;
mod windows;

/**
The [`WinitModule`] adds winit as a backend for [magma_windowing](https://crates.io/crates/magma_windowing). It also automatically creates one window on application start.
*/
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(self, app: &mut magma_app::App) {
        app.set_runner(winit_event_loop);
        app.add_module(WindowingModule);

        // default event handling
        app.add_event_systems::<WindowCloseRequested>(&[(
            systems::mark_closed_windows,
            "winit_mark_closed",
            &[],
        )])
        .unwrap();
        app.add_event_systems::<WindowResized>(&[(systems::resized, "winit_resized", &[])])
            .unwrap();
        app.add_event_systems::<WindowMoved>(&[(systems::moved, "winit_moved", &[])])
            .unwrap();
        app.add_event_systems::<WindowFocused>(&[(systems::focused, "winit_focused", &[])])
            .unwrap();
    }
}

struct WrappedApp {
    app: App,
    windows: Windows,
}

impl ApplicationHandler for WrappedApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let primary_monitor = event_loop.primary_monitor();
        for (id, winit_monitor) in event_loop.available_monitors().enumerate() {
            let monitor = Monitor {
                name: winit_monitor.name(),
                height: winit_monitor.size().height,
                width: winit_monitor.size().width,
                position: IVec2::new(winit_monitor.position().x, winit_monitor.position().y),
                refresh_rate: winit_monitor.refresh_rate_millihertz(),
                scale_factor: winit_monitor.scale_factor(),
                video_modes: winit_monitor
                    .video_modes()
                    .map(|video_mode_handle| VideoMode {
                        size: UVec2::new(
                            video_mode_handle.size().width,
                            video_mode_handle.size().height,
                        ),
                        bit_depth: video_mode_handle.bit_depth(),
                        refresh_rate: video_mode_handle.refresh_rate_millihertz(),
                    })
                    .collect(),
                id,
            };
            if primary_monitor.as_ref() == Some(&winit_monitor) {
                self.app
                    .world
                    .create_entity((monitor, PrimaryMonitor))
                    .unwrap();
            } else {
                self.app.world.create_entity((monitor,)).unwrap();
            }
        }
    }

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // convert winit events to app events
        match event {
            WindowEvent::Resized(physical_size) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowResized {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    width: physical_size.width,
                    height: physical_size.height,
                })
                .unwrap(),
            WindowEvent::Moved(physical_position) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowMoved {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    position: IVec2 {
                        x: physical_position.x,
                        y: physical_position.y,
                    },
                })
                .unwrap(),
            WindowEvent::CloseRequested => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowCloseRequested {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::Destroyed => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowDestroyed)
                .unwrap(),
            WindowEvent::DroppedFile(path_buf) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::Dropped {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    path: path_buf,
                })
                .unwrap(),
            WindowEvent::HoveredFile(path_buf) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::Hovered {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    path: path_buf,
                })
                .unwrap(),
            WindowEvent::HoveredFileCancelled => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::HoverCanceled {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::Focused(focus) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowFocused {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    focus,
                })
                .unwrap(),
            WindowEvent::CursorMoved { position, .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorMoved {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    position: IVec2 {
                        x: position.x as i32,
                        y: position.y as i32,
                    },
                })
                .unwrap(),
            WindowEvent::CursorEntered { .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorEntered {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::CursorLeft { .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorLeft {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::ThemeChanged(theme) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowThemeChanged {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    theme: match theme {
                        winit::window::Theme::Light => WindowTheme::Light,
                        winit::window::Theme::Dark => WindowTheme::Dark,
                    },
                })
                .unwrap(),
            WindowEvent::Occluded(occlusion) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(match occlusion {
                    true => WindowOcclusion::Occluded {
                        window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    },
                    false => WindowOcclusion::NotOccluded {
                        window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    },
                })
                .unwrap(),
            WindowEvent::RedrawRequested => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(RedrawRequested)
                .unwrap(),
            WindowEvent::KeyboardInput { event, .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(KeyboardInput {
                    key: match event.logical_key {
                        winit::keyboard::Key::Named(named_key) => match named_key {
                            winit::keyboard::NamedKey::Alt => magma_input::keyboard::Key::Alt,
                            winit::keyboard::NamedKey::AltGraph => {
                                magma_input::keyboard::Key::AltGraph
                            }
                            winit::keyboard::NamedKey::CapsLock => {
                                magma_input::keyboard::Key::CapsLock
                            }
                            winit::keyboard::NamedKey::Control => {
                                magma_input::keyboard::Key::Control
                            }
                            winit::keyboard::NamedKey::Fn => magma_input::keyboard::Key::Fn,
                            winit::keyboard::NamedKey::FnLock => magma_input::keyboard::Key::FnLock,
                            winit::keyboard::NamedKey::NumLock => {
                                magma_input::keyboard::Key::NumLock
                            }
                            winit::keyboard::NamedKey::ScrollLock => {
                                magma_input::keyboard::Key::ScrollLock
                            }
                            winit::keyboard::NamedKey::Shift => magma_input::keyboard::Key::Shift,
                            winit::keyboard::NamedKey::Symbol => magma_input::keyboard::Key::Symbol,
                            winit::keyboard::NamedKey::SymbolLock => {
                                magma_input::keyboard::Key::SymbolLock
                            }
                            winit::keyboard::NamedKey::Meta => magma_input::keyboard::Key::Meta,
                            winit::keyboard::NamedKey::Hyper => magma_input::keyboard::Key::Hyper,
                            winit::keyboard::NamedKey::Super => magma_input::keyboard::Key::Super,
                            winit::keyboard::NamedKey::Enter => magma_input::keyboard::Key::Enter,
                            winit::keyboard::NamedKey::Tab => magma_input::keyboard::Key::Tab,
                            winit::keyboard::NamedKey::Space => magma_input::keyboard::Key::Space,
                            winit::keyboard::NamedKey::ArrowDown => {
                                magma_input::keyboard::Key::ArrowDown
                            }
                            winit::keyboard::NamedKey::ArrowLeft => {
                                magma_input::keyboard::Key::ArrowLeft
                            }
                            winit::keyboard::NamedKey::ArrowRight => {
                                magma_input::keyboard::Key::ArrowRight
                            }
                            winit::keyboard::NamedKey::ArrowUp => {
                                magma_input::keyboard::Key::ArrowUp
                            }
                            winit::keyboard::NamedKey::End => magma_input::keyboard::Key::End,
                            winit::keyboard::NamedKey::Home => magma_input::keyboard::Key::Home,
                            winit::keyboard::NamedKey::PageDown => {
                                magma_input::keyboard::Key::PageDown
                            }
                            winit::keyboard::NamedKey::PageUp => magma_input::keyboard::Key::PageUp,
                            winit::keyboard::NamedKey::Backspace => {
                                magma_input::keyboard::Key::Backspace
                            }
                            winit::keyboard::NamedKey::Clear => magma_input::keyboard::Key::Clear,
                            winit::keyboard::NamedKey::Copy => magma_input::keyboard::Key::Copy,
                            winit::keyboard::NamedKey::CrSel => magma_input::keyboard::Key::CrSel,
                            winit::keyboard::NamedKey::Cut => magma_input::keyboard::Key::Cut,
                            winit::keyboard::NamedKey::Delete => magma_input::keyboard::Key::Delete,
                            winit::keyboard::NamedKey::EraseEof => {
                                magma_input::keyboard::Key::EraseEof
                            }
                            winit::keyboard::NamedKey::ExSel => magma_input::keyboard::Key::ExSel,
                            winit::keyboard::NamedKey::Insert => magma_input::keyboard::Key::Insert,
                            winit::keyboard::NamedKey::Paste => magma_input::keyboard::Key::Paste,
                            winit::keyboard::NamedKey::Redo => magma_input::keyboard::Key::Redo,
                            winit::keyboard::NamedKey::Undo => magma_input::keyboard::Key::Undo,
                            winit::keyboard::NamedKey::Accept => magma_input::keyboard::Key::Accept,
                            winit::keyboard::NamedKey::Again => magma_input::keyboard::Key::Again,
                            winit::keyboard::NamedKey::Attn => magma_input::keyboard::Key::Attn,
                            winit::keyboard::NamedKey::Cancel => magma_input::keyboard::Key::Cancel,
                            winit::keyboard::NamedKey::ContextMenu => {
                                magma_input::keyboard::Key::ContextMenu
                            }
                            winit::keyboard::NamedKey::Escape => magma_input::keyboard::Key::Escape,
                            winit::keyboard::NamedKey::Execute => {
                                magma_input::keyboard::Key::Execute
                            }
                            winit::keyboard::NamedKey::Find => magma_input::keyboard::Key::Find,
                            winit::keyboard::NamedKey::Help => magma_input::keyboard::Key::Help,
                            winit::keyboard::NamedKey::Pause => magma_input::keyboard::Key::Pause,
                            winit::keyboard::NamedKey::Play => magma_input::keyboard::Key::Play,
                            winit::keyboard::NamedKey::Props => magma_input::keyboard::Key::Props,
                            winit::keyboard::NamedKey::Select => magma_input::keyboard::Key::Select,
                            winit::keyboard::NamedKey::ZoomIn => magma_input::keyboard::Key::ZoomIn,
                            winit::keyboard::NamedKey::ZoomOut => {
                                magma_input::keyboard::Key::ZoomOut
                            }
                            winit::keyboard::NamedKey::BrightnessDown => {
                                magma_input::keyboard::Key::BrightnessDown
                            }
                            winit::keyboard::NamedKey::BrightnessUp => {
                                magma_input::keyboard::Key::BrightnessUp
                            }
                            winit::keyboard::NamedKey::Eject => magma_input::keyboard::Key::Eject,
                            winit::keyboard::NamedKey::LogOff => magma_input::keyboard::Key::LogOff,
                            winit::keyboard::NamedKey::Power => magma_input::keyboard::Key::Power,
                            winit::keyboard::NamedKey::PowerOff => {
                                magma_input::keyboard::Key::PowerOff
                            }
                            winit::keyboard::NamedKey::PrintScreen => {
                                magma_input::keyboard::Key::PrintScreen
                            }
                            winit::keyboard::NamedKey::Hibernate => {
                                magma_input::keyboard::Key::Hibernate
                            }
                            winit::keyboard::NamedKey::Standby => {
                                magma_input::keyboard::Key::Standby
                            }
                            winit::keyboard::NamedKey::WakeUp => magma_input::keyboard::Key::WakeUp,
                            winit::keyboard::NamedKey::AllCandidates => {
                                magma_input::keyboard::Key::AllCandidates
                            }
                            winit::keyboard::NamedKey::Alphanumeric => {
                                magma_input::keyboard::Key::Alphanumeric
                            }
                            winit::keyboard::NamedKey::CodeInput => {
                                magma_input::keyboard::Key::CodeInput
                            }
                            winit::keyboard::NamedKey::Compose => {
                                magma_input::keyboard::Key::Compose
                            }
                            winit::keyboard::NamedKey::Convert => {
                                magma_input::keyboard::Key::Convert
                            }
                            winit::keyboard::NamedKey::FinalMode => {
                                magma_input::keyboard::Key::FinalMode
                            }
                            winit::keyboard::NamedKey::GroupFirst => {
                                magma_input::keyboard::Key::GroupFirst
                            }
                            winit::keyboard::NamedKey::GroupLast => {
                                magma_input::keyboard::Key::GroupLast
                            }
                            winit::keyboard::NamedKey::GroupNext => {
                                magma_input::keyboard::Key::GroupNext
                            }
                            winit::keyboard::NamedKey::GroupPrevious => {
                                magma_input::keyboard::Key::GroupPrevious
                            }
                            winit::keyboard::NamedKey::ModeChange => {
                                magma_input::keyboard::Key::ModeChange
                            }
                            winit::keyboard::NamedKey::NextCandidate => {
                                magma_input::keyboard::Key::NextCandidate
                            }
                            winit::keyboard::NamedKey::NonConvert => {
                                magma_input::keyboard::Key::NonConvert
                            }
                            winit::keyboard::NamedKey::PreviousCandidate => {
                                magma_input::keyboard::Key::PreviousCandidate
                            }
                            winit::keyboard::NamedKey::Process => {
                                magma_input::keyboard::Key::Process
                            }
                            winit::keyboard::NamedKey::SingleCandidate => {
                                magma_input::keyboard::Key::SingleCandidate
                            }
                            winit::keyboard::NamedKey::HangulMode => {
                                magma_input::keyboard::Key::HangulMode
                            }
                            winit::keyboard::NamedKey::HanjaMode => {
                                magma_input::keyboard::Key::HanjaMode
                            }
                            winit::keyboard::NamedKey::JunjaMode => {
                                magma_input::keyboard::Key::JunjaMode
                            }
                            winit::keyboard::NamedKey::Eisu => magma_input::keyboard::Key::Eisu,
                            winit::keyboard::NamedKey::Hankaku => {
                                magma_input::keyboard::Key::Hankaku
                            }
                            winit::keyboard::NamedKey::Hiragana => {
                                magma_input::keyboard::Key::Hiragana
                            }
                            winit::keyboard::NamedKey::HiraganaKatakana => {
                                magma_input::keyboard::Key::HiraganaKatakana
                            }
                            winit::keyboard::NamedKey::KanaMode => {
                                magma_input::keyboard::Key::KanaMode
                            }
                            winit::keyboard::NamedKey::KanjiMode => {
                                magma_input::keyboard::Key::KanjiMode
                            }
                            winit::keyboard::NamedKey::Katakana => {
                                magma_input::keyboard::Key::Katakana
                            }
                            winit::keyboard::NamedKey::Romaji => magma_input::keyboard::Key::Romaji,
                            winit::keyboard::NamedKey::Zenkaku => {
                                magma_input::keyboard::Key::Zenkaku
                            }
                            winit::keyboard::NamedKey::ZenkakuHankaku => {
                                magma_input::keyboard::Key::ZenkakuHankaku
                            }
                            winit::keyboard::NamedKey::Soft1 => magma_input::keyboard::Key::Soft1,
                            winit::keyboard::NamedKey::Soft2 => magma_input::keyboard::Key::Soft2,
                            winit::keyboard::NamedKey::Soft3 => magma_input::keyboard::Key::Soft3,
                            winit::keyboard::NamedKey::Soft4 => magma_input::keyboard::Key::Soft4,
                            winit::keyboard::NamedKey::ChannelDown => {
                                magma_input::keyboard::Key::ChannelDown
                            }
                            winit::keyboard::NamedKey::ChannelUp => {
                                magma_input::keyboard::Key::ChannelUp
                            }
                            winit::keyboard::NamedKey::Close => magma_input::keyboard::Key::Close,
                            winit::keyboard::NamedKey::MailForward => {
                                magma_input::keyboard::Key::MailForward
                            }
                            winit::keyboard::NamedKey::MailReply => {
                                magma_input::keyboard::Key::MailReply
                            }
                            winit::keyboard::NamedKey::MailSend => {
                                magma_input::keyboard::Key::MailSend
                            }
                            winit::keyboard::NamedKey::MediaClose => {
                                magma_input::keyboard::Key::MediaClose
                            }
                            winit::keyboard::NamedKey::MediaFastForward => {
                                magma_input::keyboard::Key::MediaFastForward
                            }
                            winit::keyboard::NamedKey::MediaPause => {
                                magma_input::keyboard::Key::MediaPause
                            }
                            winit::keyboard::NamedKey::MediaPlay => {
                                magma_input::keyboard::Key::MediaPlay
                            }
                            winit::keyboard::NamedKey::MediaPlayPause => {
                                magma_input::keyboard::Key::MediaPlayPause
                            }
                            winit::keyboard::NamedKey::MediaRecord => {
                                magma_input::keyboard::Key::MediaRecord
                            }
                            winit::keyboard::NamedKey::MediaRewind => {
                                magma_input::keyboard::Key::MediaRewind
                            }
                            winit::keyboard::NamedKey::MediaStop => {
                                magma_input::keyboard::Key::MediaStop
                            }
                            winit::keyboard::NamedKey::MediaTrackNext => {
                                magma_input::keyboard::Key::MediaTrackNext
                            }
                            winit::keyboard::NamedKey::MediaTrackPrevious => {
                                magma_input::keyboard::Key::MediaTrackPrevious
                            }
                            winit::keyboard::NamedKey::New => magma_input::keyboard::Key::New,
                            winit::keyboard::NamedKey::Open => magma_input::keyboard::Key::Open,
                            winit::keyboard::NamedKey::Print => magma_input::keyboard::Key::Print,
                            winit::keyboard::NamedKey::Save => magma_input::keyboard::Key::Save,
                            winit::keyboard::NamedKey::SpellCheck => {
                                magma_input::keyboard::Key::SpellCheck
                            }
                            winit::keyboard::NamedKey::Key11 => magma_input::keyboard::Key::Key11,
                            winit::keyboard::NamedKey::Key12 => magma_input::keyboard::Key::Key12,
                            winit::keyboard::NamedKey::AudioBalanceLeft => {
                                magma_input::keyboard::Key::AudioBalanceLeft
                            }
                            winit::keyboard::NamedKey::AudioBalanceRight => {
                                magma_input::keyboard::Key::AudioBalanceRight
                            }
                            winit::keyboard::NamedKey::AudioBassBoostDown => {
                                magma_input::keyboard::Key::AudioBassBoostDown
                            }
                            winit::keyboard::NamedKey::AudioBassBoostToggle => {
                                magma_input::keyboard::Key::AudioBassBoostToggle
                            }
                            winit::keyboard::NamedKey::AudioBassBoostUp => {
                                magma_input::keyboard::Key::AudioBassBoostUp
                            }
                            winit::keyboard::NamedKey::AudioFaderFront => {
                                magma_input::keyboard::Key::AudioFaderFront
                            }
                            winit::keyboard::NamedKey::AudioFaderRear => {
                                magma_input::keyboard::Key::AudioFaderRear
                            }
                            winit::keyboard::NamedKey::AudioSurroundModeNext => {
                                magma_input::keyboard::Key::AudioSurroundModeNext
                            }
                            winit::keyboard::NamedKey::AudioTrebleDown => {
                                magma_input::keyboard::Key::AudioTrebleDown
                            }
                            winit::keyboard::NamedKey::AudioTrebleUp => {
                                magma_input::keyboard::Key::AudioTrebleUp
                            }
                            winit::keyboard::NamedKey::AudioVolumeDown => {
                                magma_input::keyboard::Key::AudioVolumeDown
                            }
                            winit::keyboard::NamedKey::AudioVolumeUp => {
                                magma_input::keyboard::Key::AudioVolumeUp
                            }
                            winit::keyboard::NamedKey::AudioVolumeMute => {
                                magma_input::keyboard::Key::AudioVolumeMute
                            }
                            winit::keyboard::NamedKey::MicrophoneToggle => {
                                magma_input::keyboard::Key::MicrophoneToggle
                            }
                            winit::keyboard::NamedKey::MicrophoneVolumeDown => {
                                magma_input::keyboard::Key::MicrophoneVolumeDown
                            }
                            winit::keyboard::NamedKey::MicrophoneVolumeUp => {
                                magma_input::keyboard::Key::MicrophoneVolumeDown
                            }
                            winit::keyboard::NamedKey::MicrophoneVolumeMute => {
                                magma_input::keyboard::Key::MicrophoneVolumeMute
                            }
                            winit::keyboard::NamedKey::SpeechCorrectionList => {
                                magma_input::keyboard::Key::SpeechCorrectionList
                            }
                            winit::keyboard::NamedKey::SpeechInputToggle => {
                                magma_input::keyboard::Key::SpeechInputToggle
                            }
                            winit::keyboard::NamedKey::LaunchApplication1 => {
                                magma_input::keyboard::Key::LaunchApplication1
                            }
                            winit::keyboard::NamedKey::LaunchApplication2 => {
                                magma_input::keyboard::Key::LaunchApplication2
                            }
                            winit::keyboard::NamedKey::LaunchCalendar => {
                                magma_input::keyboard::Key::LaunchCalendar
                            }
                            winit::keyboard::NamedKey::LaunchContacts => {
                                magma_input::keyboard::Key::LaunchContacts
                            }
                            winit::keyboard::NamedKey::LaunchMail => {
                                magma_input::keyboard::Key::LaunchMail
                            }
                            winit::keyboard::NamedKey::LaunchMediaPlayer => {
                                magma_input::keyboard::Key::LaunchMediaPlayer
                            }
                            winit::keyboard::NamedKey::LaunchMusicPlayer => {
                                magma_input::keyboard::Key::LaunchMusicPlayer
                            }
                            winit::keyboard::NamedKey::LaunchPhone => {
                                magma_input::keyboard::Key::LaunchPhone
                            }
                            winit::keyboard::NamedKey::LaunchScreenSaver => {
                                magma_input::keyboard::Key::LaunchScreenSaver
                            }
                            winit::keyboard::NamedKey::LaunchSpreadsheet => {
                                magma_input::keyboard::Key::LaunchSpreadsheet
                            }
                            winit::keyboard::NamedKey::LaunchWebBrowser => {
                                magma_input::keyboard::Key::LaunchWebBrowser
                            }
                            winit::keyboard::NamedKey::LaunchWebCam => {
                                magma_input::keyboard::Key::LaunchWebCam
                            }
                            winit::keyboard::NamedKey::LaunchWordProcessor => {
                                magma_input::keyboard::Key::LaunchWordProcessor
                            }
                            winit::keyboard::NamedKey::BrowserBack => {
                                magma_input::keyboard::Key::BrowserBack
                            }
                            winit::keyboard::NamedKey::BrowserFavorites => {
                                magma_input::keyboard::Key::BrowserFavorites
                            }
                            winit::keyboard::NamedKey::BrowserForward => {
                                magma_input::keyboard::Key::BrowserForward
                            }
                            winit::keyboard::NamedKey::BrowserHome => {
                                magma_input::keyboard::Key::BrowserHome
                            }
                            winit::keyboard::NamedKey::BrowserRefresh => {
                                magma_input::keyboard::Key::BrowserRefresh
                            }
                            winit::keyboard::NamedKey::BrowserSearch => {
                                magma_input::keyboard::Key::BrowserSearch
                            }
                            winit::keyboard::NamedKey::BrowserStop => {
                                magma_input::keyboard::Key::BrowserStop
                            }
                            winit::keyboard::NamedKey::AppSwitch => {
                                magma_input::keyboard::Key::AppSwitch
                            }
                            winit::keyboard::NamedKey::Call => magma_input::keyboard::Key::Call,
                            winit::keyboard::NamedKey::Camera => magma_input::keyboard::Key::Camera,
                            winit::keyboard::NamedKey::CameraFocus => {
                                magma_input::keyboard::Key::CameraFocus
                            }
                            winit::keyboard::NamedKey::EndCall => {
                                magma_input::keyboard::Key::EndCall
                            }
                            winit::keyboard::NamedKey::GoBack => magma_input::keyboard::Key::GoBack,
                            winit::keyboard::NamedKey::GoHome => magma_input::keyboard::Key::GoHome,
                            winit::keyboard::NamedKey::HeadsetHook => {
                                magma_input::keyboard::Key::HeadsetHook
                            }
                            winit::keyboard::NamedKey::LastNumberRedial => {
                                magma_input::keyboard::Key::LastNumberRedial
                            }
                            winit::keyboard::NamedKey::Notification => {
                                magma_input::keyboard::Key::Notification
                            }
                            winit::keyboard::NamedKey::MannerMode => {
                                magma_input::keyboard::Key::MannerMode
                            }
                            winit::keyboard::NamedKey::VoiceDial => {
                                magma_input::keyboard::Key::VoiceDial
                            }
                            winit::keyboard::NamedKey::TV => magma_input::keyboard::Key::TV,
                            winit::keyboard::NamedKey::TV3DMode => {
                                magma_input::keyboard::Key::TV3DMode
                            }
                            winit::keyboard::NamedKey::TVAntennaCable => {
                                magma_input::keyboard::Key::TVAntennaCable
                            }
                            winit::keyboard::NamedKey::TVAudioDescription => {
                                magma_input::keyboard::Key::TVAudioDescription
                            }
                            winit::keyboard::NamedKey::TVAudioDescriptionMixDown => {
                                magma_input::keyboard::Key::TVAudioDescriptionMixDown
                            }
                            winit::keyboard::NamedKey::TVAudioDescriptionMixUp => {
                                magma_input::keyboard::Key::TVAudioDescriptionMixUp
                            }
                            winit::keyboard::NamedKey::TVContentsMenu => {
                                magma_input::keyboard::Key::TVContentsMenu
                            }
                            winit::keyboard::NamedKey::TVDataService => {
                                magma_input::keyboard::Key::TVDataService
                            }
                            winit::keyboard::NamedKey::TVInput => {
                                magma_input::keyboard::Key::TVInput
                            }
                            winit::keyboard::NamedKey::TVInputComponent1 => {
                                magma_input::keyboard::Key::TVInputComponent1
                            }
                            winit::keyboard::NamedKey::TVInputComponent2 => {
                                magma_input::keyboard::Key::TVInputComponent2
                            }
                            winit::keyboard::NamedKey::TVInputComposite1 => {
                                magma_input::keyboard::Key::TVInputComposite1
                            }
                            winit::keyboard::NamedKey::TVInputComposite2 => {
                                magma_input::keyboard::Key::TVInputComposite2
                            }
                            winit::keyboard::NamedKey::TVInputHDMI1 => {
                                magma_input::keyboard::Key::TVInputHDMI1
                            }
                            winit::keyboard::NamedKey::TVInputHDMI2 => {
                                magma_input::keyboard::Key::TVInputHDMI2
                            }
                            winit::keyboard::NamedKey::TVInputHDMI3 => {
                                magma_input::keyboard::Key::TVInputHDMI3
                            }
                            winit::keyboard::NamedKey::TVInputHDMI4 => {
                                magma_input::keyboard::Key::TVInputHDMI4
                            }
                            winit::keyboard::NamedKey::TVInputVGA1 => {
                                magma_input::keyboard::Key::TVInputVGA1
                            }
                            winit::keyboard::NamedKey::TVMediaContext => {
                                magma_input::keyboard::Key::TVMediaContext
                            }
                            winit::keyboard::NamedKey::TVNetwork => {
                                magma_input::keyboard::Key::TVNetwork
                            }
                            winit::keyboard::NamedKey::TVNumberEntry => {
                                magma_input::keyboard::Key::TVNumberEntry
                            }
                            winit::keyboard::NamedKey::TVPower => {
                                magma_input::keyboard::Key::TVPower
                            }
                            winit::keyboard::NamedKey::TVRadioService => {
                                magma_input::keyboard::Key::TVRadioService
                            }
                            winit::keyboard::NamedKey::TVSatellite => {
                                magma_input::keyboard::Key::TVSatellite
                            }
                            winit::keyboard::NamedKey::TVSatelliteBS => {
                                magma_input::keyboard::Key::TVSatelliteBS
                            }
                            winit::keyboard::NamedKey::TVSatelliteCS => {
                                magma_input::keyboard::Key::TVSatelliteCS
                            }
                            winit::keyboard::NamedKey::TVSatelliteToggle => {
                                magma_input::keyboard::Key::TVSatelliteToggle
                            }
                            winit::keyboard::NamedKey::TVTerrestrialAnalog => {
                                magma_input::keyboard::Key::TVTerrestrialAnalog
                            }
                            winit::keyboard::NamedKey::TVTerrestrialDigital => {
                                magma_input::keyboard::Key::TVTerrestrialDigital
                            }
                            winit::keyboard::NamedKey::TVTimer => {
                                magma_input::keyboard::Key::TVTimer
                            }
                            winit::keyboard::NamedKey::AVRInput => {
                                magma_input::keyboard::Key::AVRInput
                            }
                            winit::keyboard::NamedKey::AVRPower => {
                                magma_input::keyboard::Key::AVRPower
                            }
                            winit::keyboard::NamedKey::ColorF0Red => {
                                magma_input::keyboard::Key::ColorF0Red
                            }
                            winit::keyboard::NamedKey::ColorF1Green => {
                                magma_input::keyboard::Key::ColorF1Green
                            }
                            winit::keyboard::NamedKey::ColorF2Yellow => {
                                magma_input::keyboard::Key::ColorF2Yellow
                            }
                            winit::keyboard::NamedKey::ColorF3Blue => {
                                magma_input::keyboard::Key::ColorF3Blue
                            }
                            winit::keyboard::NamedKey::ColorF4Grey => {
                                magma_input::keyboard::Key::ColorF4Grey
                            }
                            winit::keyboard::NamedKey::ColorF5Brown => {
                                magma_input::keyboard::Key::ColorF5Brown
                            }
                            winit::keyboard::NamedKey::ClosedCaptionToggle => {
                                magma_input::keyboard::Key::ClosedCaptionToggle
                            }
                            winit::keyboard::NamedKey::Dimmer => magma_input::keyboard::Key::Dimmer,
                            winit::keyboard::NamedKey::DisplaySwap => {
                                magma_input::keyboard::Key::DisplaySwap
                            }
                            winit::keyboard::NamedKey::DVR => magma_input::keyboard::Key::DVR,
                            winit::keyboard::NamedKey::Exit => magma_input::keyboard::Key::Exit,
                            winit::keyboard::NamedKey::FavoriteClear0 => {
                                magma_input::keyboard::Key::FavoriteClear0
                            }
                            winit::keyboard::NamedKey::FavoriteClear1 => {
                                magma_input::keyboard::Key::FavoriteClear1
                            }
                            winit::keyboard::NamedKey::FavoriteClear2 => {
                                magma_input::keyboard::Key::FavoriteClear2
                            }
                            winit::keyboard::NamedKey::FavoriteClear3 => {
                                magma_input::keyboard::Key::FavoriteClear3
                            }
                            winit::keyboard::NamedKey::FavoriteRecall0 => {
                                magma_input::keyboard::Key::FavoriteRecall0
                            }
                            winit::keyboard::NamedKey::FavoriteRecall1 => {
                                magma_input::keyboard::Key::FavoriteRecall1
                            }
                            winit::keyboard::NamedKey::FavoriteRecall2 => {
                                magma_input::keyboard::Key::FavoriteRecall2
                            }
                            winit::keyboard::NamedKey::FavoriteRecall3 => {
                                magma_input::keyboard::Key::FavoriteRecall3
                            }
                            winit::keyboard::NamedKey::FavoriteStore0 => {
                                magma_input::keyboard::Key::FavoriteStore0
                            }
                            winit::keyboard::NamedKey::FavoriteStore1 => {
                                magma_input::keyboard::Key::FavoriteStore1
                            }
                            winit::keyboard::NamedKey::FavoriteStore2 => {
                                magma_input::keyboard::Key::FavoriteStore2
                            }
                            winit::keyboard::NamedKey::FavoriteStore3 => {
                                magma_input::keyboard::Key::FavoriteStore3
                            }
                            winit::keyboard::NamedKey::Guide => magma_input::keyboard::Key::Guide,
                            winit::keyboard::NamedKey::GuideNextDay => {
                                magma_input::keyboard::Key::GuideNextDay
                            }
                            winit::keyboard::NamedKey::GuidePreviousDay => {
                                magma_input::keyboard::Key::GuidePreviousDay
                            }
                            winit::keyboard::NamedKey::Info => magma_input::keyboard::Key::Info,
                            winit::keyboard::NamedKey::InstantReplay => {
                                magma_input::keyboard::Key::InstantReplay
                            }
                            winit::keyboard::NamedKey::Link => magma_input::keyboard::Key::Link,
                            winit::keyboard::NamedKey::ListProgram => {
                                magma_input::keyboard::Key::ListProgram
                            }
                            winit::keyboard::NamedKey::LiveContent => {
                                magma_input::keyboard::Key::LiveContent
                            }
                            winit::keyboard::NamedKey::Lock => magma_input::keyboard::Key::Lock,
                            winit::keyboard::NamedKey::MediaApps => {
                                magma_input::keyboard::Key::MediaApps
                            }
                            winit::keyboard::NamedKey::MediaAudioTrack => {
                                magma_input::keyboard::Key::MediaAudioTrack
                            }
                            winit::keyboard::NamedKey::MediaLast => {
                                magma_input::keyboard::Key::MediaLast
                            }
                            winit::keyboard::NamedKey::MediaSkipBackward => {
                                magma_input::keyboard::Key::MediaSkipBackward
                            }
                            winit::keyboard::NamedKey::MediaSkipForward => {
                                magma_input::keyboard::Key::MediaSkipForward
                            }
                            winit::keyboard::NamedKey::MediaStepBackward => {
                                magma_input::keyboard::Key::MediaStepBackward
                            }
                            winit::keyboard::NamedKey::MediaStepForward => {
                                magma_input::keyboard::Key::MediaStepForward
                            }
                            winit::keyboard::NamedKey::MediaTopMenu => {
                                magma_input::keyboard::Key::MediaTopMenu
                            }
                            winit::keyboard::NamedKey::NavigateIn => {
                                magma_input::keyboard::Key::NavigateIn
                            }
                            winit::keyboard::NamedKey::NavigateNext => {
                                magma_input::keyboard::Key::NavigateNext
                            }
                            winit::keyboard::NamedKey::NavigateOut => {
                                magma_input::keyboard::Key::NavigateOut
                            }
                            winit::keyboard::NamedKey::NavigatePrevious => {
                                magma_input::keyboard::Key::NavigatePrevious
                            }
                            winit::keyboard::NamedKey::NextFavoriteChannel => {
                                magma_input::keyboard::Key::NextFavoriteChannel
                            }
                            winit::keyboard::NamedKey::NextUserProfile => {
                                magma_input::keyboard::Key::NextUserProfile
                            }
                            winit::keyboard::NamedKey::OnDemand => {
                                magma_input::keyboard::Key::OnDemand
                            }
                            winit::keyboard::NamedKey::Pairing => {
                                magma_input::keyboard::Key::Pairing
                            }
                            winit::keyboard::NamedKey::PinPDown => {
                                magma_input::keyboard::Key::PinPDown
                            }
                            winit::keyboard::NamedKey::PinPMove => {
                                magma_input::keyboard::Key::PinPMove
                            }
                            winit::keyboard::NamedKey::PinPToggle => {
                                magma_input::keyboard::Key::PinPToggle
                            }
                            winit::keyboard::NamedKey::PinPUp => magma_input::keyboard::Key::PinPUp,
                            winit::keyboard::NamedKey::PlaySpeedDown => {
                                magma_input::keyboard::Key::PlaySpeedDown
                            }
                            winit::keyboard::NamedKey::PlaySpeedReset => {
                                magma_input::keyboard::Key::PlaySpeedReset
                            }
                            winit::keyboard::NamedKey::PlaySpeedUp => {
                                magma_input::keyboard::Key::PlaySpeedUp
                            }
                            winit::keyboard::NamedKey::RandomToggle => {
                                magma_input::keyboard::Key::RandomToggle
                            }
                            winit::keyboard::NamedKey::RcLowBattery => {
                                magma_input::keyboard::Key::RcLowBattery
                            }
                            winit::keyboard::NamedKey::RecordSpeedNext => {
                                magma_input::keyboard::Key::RecordSpeedNext
                            }
                            winit::keyboard::NamedKey::RfBypass => {
                                magma_input::keyboard::Key::RfBypass
                            }
                            winit::keyboard::NamedKey::ScanChannelsToggle => {
                                magma_input::keyboard::Key::ScanChannelsToggle
                            }
                            winit::keyboard::NamedKey::ScreenModeNext => {
                                magma_input::keyboard::Key::ScreenModeNext
                            }
                            winit::keyboard::NamedKey::Settings => {
                                magma_input::keyboard::Key::Settings
                            }
                            winit::keyboard::NamedKey::SplitScreenToggle => {
                                magma_input::keyboard::Key::SplitScreenToggle
                            }
                            winit::keyboard::NamedKey::STBInput => {
                                magma_input::keyboard::Key::STBInput
                            }
                            winit::keyboard::NamedKey::STBPower => {
                                magma_input::keyboard::Key::STBPower
                            }
                            winit::keyboard::NamedKey::Subtitle => {
                                magma_input::keyboard::Key::Subtitle
                            }
                            winit::keyboard::NamedKey::Teletext => {
                                magma_input::keyboard::Key::Teletext
                            }
                            winit::keyboard::NamedKey::VideoModeNext => {
                                magma_input::keyboard::Key::VideoModeNext
                            }
                            winit::keyboard::NamedKey::Wink => magma_input::keyboard::Key::Wink,
                            winit::keyboard::NamedKey::ZoomToggle => {
                                magma_input::keyboard::Key::ZoomToggle
                            }
                            winit::keyboard::NamedKey::F1 => magma_input::keyboard::Key::F1,
                            winit::keyboard::NamedKey::F2 => magma_input::keyboard::Key::F2,
                            winit::keyboard::NamedKey::F3 => magma_input::keyboard::Key::F3,
                            winit::keyboard::NamedKey::F4 => magma_input::keyboard::Key::F4,
                            winit::keyboard::NamedKey::F5 => magma_input::keyboard::Key::F5,
                            winit::keyboard::NamedKey::F6 => magma_input::keyboard::Key::F6,
                            winit::keyboard::NamedKey::F7 => magma_input::keyboard::Key::F7,
                            winit::keyboard::NamedKey::F8 => magma_input::keyboard::Key::F8,
                            winit::keyboard::NamedKey::F9 => magma_input::keyboard::Key::F9,
                            winit::keyboard::NamedKey::F10 => magma_input::keyboard::Key::F10,
                            winit::keyboard::NamedKey::F11 => magma_input::keyboard::Key::F11,
                            winit::keyboard::NamedKey::F12 => magma_input::keyboard::Key::F12,
                            winit::keyboard::NamedKey::F13 => magma_input::keyboard::Key::F13,
                            winit::keyboard::NamedKey::F14 => magma_input::keyboard::Key::F14,
                            winit::keyboard::NamedKey::F15 => magma_input::keyboard::Key::F15,
                            winit::keyboard::NamedKey::F16 => magma_input::keyboard::Key::F16,
                            winit::keyboard::NamedKey::F17 => magma_input::keyboard::Key::F17,
                            winit::keyboard::NamedKey::F18 => magma_input::keyboard::Key::F18,
                            winit::keyboard::NamedKey::F19 => magma_input::keyboard::Key::F19,
                            winit::keyboard::NamedKey::F20 => magma_input::keyboard::Key::F20,
                            winit::keyboard::NamedKey::F21 => magma_input::keyboard::Key::F21,
                            winit::keyboard::NamedKey::F22 => magma_input::keyboard::Key::F22,
                            winit::keyboard::NamedKey::F23 => magma_input::keyboard::Key::F23,
                            winit::keyboard::NamedKey::F24 => magma_input::keyboard::Key::F24,
                            winit::keyboard::NamedKey::F25 => magma_input::keyboard::Key::F25,
                            winit::keyboard::NamedKey::F26 => magma_input::keyboard::Key::F26,
                            winit::keyboard::NamedKey::F27 => magma_input::keyboard::Key::F27,
                            winit::keyboard::NamedKey::F28 => magma_input::keyboard::Key::F28,
                            winit::keyboard::NamedKey::F29 => magma_input::keyboard::Key::F29,
                            winit::keyboard::NamedKey::F30 => magma_input::keyboard::Key::F30,
                            winit::keyboard::NamedKey::F31 => magma_input::keyboard::Key::F31,
                            winit::keyboard::NamedKey::F32 => magma_input::keyboard::Key::F32,
                            winit::keyboard::NamedKey::F33 => magma_input::keyboard::Key::F33,
                            winit::keyboard::NamedKey::F34 => magma_input::keyboard::Key::F34,
                            winit::keyboard::NamedKey::F35 => magma_input::keyboard::Key::F35,
                            _ => magma_input::keyboard::Key::Unidentified,
                        },
                        winit::keyboard::Key::Character(string) => {
                            magma_input::keyboard::Key::Character(string.to_string())
                        }
                        winit::keyboard::Key::Unidentified(_) => {
                            magma_input::keyboard::Key::Unidentified
                        }
                        winit::keyboard::Key::Dead(key) => magma_input::keyboard::Key::Dead(key),
                    },
                    key_code: match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                            winit::keyboard::KeyCode::Backquote => {
                                magma_input::keyboard::KeyCode::Backquote
                            }
                            winit::keyboard::KeyCode::Backslash => {
                                magma_input::keyboard::KeyCode::Backslash
                            }
                            winit::keyboard::KeyCode::BracketLeft => {
                                magma_input::keyboard::KeyCode::BracketLeft
                            }
                            winit::keyboard::KeyCode::BracketRight => {
                                magma_input::keyboard::KeyCode::BracketRight
                            }
                            winit::keyboard::KeyCode::Comma => {
                                magma_input::keyboard::KeyCode::Comma
                            }
                            winit::keyboard::KeyCode::Digit0 => {
                                magma_input::keyboard::KeyCode::Digit0
                            }
                            winit::keyboard::KeyCode::Digit1 => {
                                magma_input::keyboard::KeyCode::Digit1
                            }
                            winit::keyboard::KeyCode::Digit2 => {
                                magma_input::keyboard::KeyCode::Digit2
                            }
                            winit::keyboard::KeyCode::Digit3 => {
                                magma_input::keyboard::KeyCode::Digit3
                            }
                            winit::keyboard::KeyCode::Digit4 => {
                                magma_input::keyboard::KeyCode::Digit4
                            }
                            winit::keyboard::KeyCode::Digit5 => {
                                magma_input::keyboard::KeyCode::Digit5
                            }
                            winit::keyboard::KeyCode::Digit6 => {
                                magma_input::keyboard::KeyCode::Digit6
                            }
                            winit::keyboard::KeyCode::Digit7 => {
                                magma_input::keyboard::KeyCode::Digit7
                            }
                            winit::keyboard::KeyCode::Digit8 => {
                                magma_input::keyboard::KeyCode::Digit8
                            }
                            winit::keyboard::KeyCode::Digit9 => {
                                magma_input::keyboard::KeyCode::Digit9
                            }
                            winit::keyboard::KeyCode::Equal => {
                                magma_input::keyboard::KeyCode::Equal
                            }
                            winit::keyboard::KeyCode::IntlBackslash => {
                                magma_input::keyboard::KeyCode::IntlBackslash
                            }
                            winit::keyboard::KeyCode::IntlRo => {
                                magma_input::keyboard::KeyCode::IntlRo
                            }
                            winit::keyboard::KeyCode::IntlYen => {
                                magma_input::keyboard::KeyCode::IntlYen
                            }
                            winit::keyboard::KeyCode::KeyA => magma_input::keyboard::KeyCode::KeyA,
                            winit::keyboard::KeyCode::KeyB => magma_input::keyboard::KeyCode::KeyB,
                            winit::keyboard::KeyCode::KeyC => magma_input::keyboard::KeyCode::KeyC,
                            winit::keyboard::KeyCode::KeyD => magma_input::keyboard::KeyCode::KeyD,
                            winit::keyboard::KeyCode::KeyE => magma_input::keyboard::KeyCode::KeyE,
                            winit::keyboard::KeyCode::KeyF => magma_input::keyboard::KeyCode::KeyF,
                            winit::keyboard::KeyCode::KeyG => magma_input::keyboard::KeyCode::KeyG,
                            winit::keyboard::KeyCode::KeyH => magma_input::keyboard::KeyCode::KeyH,
                            winit::keyboard::KeyCode::KeyI => magma_input::keyboard::KeyCode::KeyI,
                            winit::keyboard::KeyCode::KeyJ => magma_input::keyboard::KeyCode::KeyJ,
                            winit::keyboard::KeyCode::KeyK => magma_input::keyboard::KeyCode::KeyK,
                            winit::keyboard::KeyCode::KeyL => magma_input::keyboard::KeyCode::KeyL,
                            winit::keyboard::KeyCode::KeyM => magma_input::keyboard::KeyCode::KeyM,
                            winit::keyboard::KeyCode::KeyN => magma_input::keyboard::KeyCode::KeyN,
                            winit::keyboard::KeyCode::KeyO => magma_input::keyboard::KeyCode::KeyO,
                            winit::keyboard::KeyCode::KeyP => magma_input::keyboard::KeyCode::KeyP,
                            winit::keyboard::KeyCode::KeyQ => magma_input::keyboard::KeyCode::KeyQ,
                            winit::keyboard::KeyCode::KeyR => magma_input::keyboard::KeyCode::KeyR,
                            winit::keyboard::KeyCode::KeyS => magma_input::keyboard::KeyCode::KeyS,
                            winit::keyboard::KeyCode::KeyT => magma_input::keyboard::KeyCode::KeyT,
                            winit::keyboard::KeyCode::KeyU => magma_input::keyboard::KeyCode::KeyU,
                            winit::keyboard::KeyCode::KeyV => magma_input::keyboard::KeyCode::KeyV,
                            winit::keyboard::KeyCode::KeyW => magma_input::keyboard::KeyCode::KeyW,
                            winit::keyboard::KeyCode::KeyX => magma_input::keyboard::KeyCode::KeyX,
                            winit::keyboard::KeyCode::KeyY => magma_input::keyboard::KeyCode::KeyY,
                            winit::keyboard::KeyCode::KeyZ => magma_input::keyboard::KeyCode::KeyZ,
                            winit::keyboard::KeyCode::Minus => {
                                magma_input::keyboard::KeyCode::Minus
                            }
                            winit::keyboard::KeyCode::Period => {
                                magma_input::keyboard::KeyCode::Period
                            }
                            winit::keyboard::KeyCode::Quote => {
                                magma_input::keyboard::KeyCode::Quote
                            }
                            winit::keyboard::KeyCode::Semicolon => {
                                magma_input::keyboard::KeyCode::Semicolon
                            }
                            winit::keyboard::KeyCode::Slash => {
                                magma_input::keyboard::KeyCode::Slash
                            }
                            winit::keyboard::KeyCode::AltLeft => {
                                magma_input::keyboard::KeyCode::AltLeft
                            }
                            winit::keyboard::KeyCode::AltRight => {
                                magma_input::keyboard::KeyCode::AltRight
                            }
                            winit::keyboard::KeyCode::Backspace => {
                                magma_input::keyboard::KeyCode::Backspace
                            }
                            winit::keyboard::KeyCode::CapsLock => {
                                magma_input::keyboard::KeyCode::CapsLock
                            }
                            winit::keyboard::KeyCode::ContextMenu => {
                                magma_input::keyboard::KeyCode::ContextMenu
                            }
                            winit::keyboard::KeyCode::ControlLeft => {
                                magma_input::keyboard::KeyCode::ControlLeft
                            }
                            winit::keyboard::KeyCode::ControlRight => {
                                magma_input::keyboard::KeyCode::ControlRight
                            }
                            winit::keyboard::KeyCode::Enter => {
                                magma_input::keyboard::KeyCode::Enter
                            }
                            winit::keyboard::KeyCode::SuperLeft => {
                                magma_input::keyboard::KeyCode::SuperLeft
                            }
                            winit::keyboard::KeyCode::SuperRight => {
                                magma_input::keyboard::KeyCode::SuperRight
                            }
                            winit::keyboard::KeyCode::ShiftLeft => {
                                magma_input::keyboard::KeyCode::ShiftLeft
                            }
                            winit::keyboard::KeyCode::ShiftRight => {
                                magma_input::keyboard::KeyCode::ShiftRight
                            }
                            winit::keyboard::KeyCode::Space => {
                                magma_input::keyboard::KeyCode::Space
                            }
                            winit::keyboard::KeyCode::Tab => magma_input::keyboard::KeyCode::Tab,
                            winit::keyboard::KeyCode::Convert => {
                                magma_input::keyboard::KeyCode::Convert
                            }
                            winit::keyboard::KeyCode::KanaMode => {
                                magma_input::keyboard::KeyCode::KanaMode
                            }
                            winit::keyboard::KeyCode::Lang1 => {
                                magma_input::keyboard::KeyCode::Lang1
                            }
                            winit::keyboard::KeyCode::Lang2 => {
                                magma_input::keyboard::KeyCode::Lang2
                            }
                            winit::keyboard::KeyCode::Lang3 => {
                                magma_input::keyboard::KeyCode::Lang3
                            }
                            winit::keyboard::KeyCode::Lang4 => {
                                magma_input::keyboard::KeyCode::Lang4
                            }
                            winit::keyboard::KeyCode::Lang5 => {
                                magma_input::keyboard::KeyCode::Lang5
                            }
                            winit::keyboard::KeyCode::NonConvert => {
                                magma_input::keyboard::KeyCode::NonConvert
                            }
                            winit::keyboard::KeyCode::Delete => {
                                magma_input::keyboard::KeyCode::Delete
                            }
                            winit::keyboard::KeyCode::End => magma_input::keyboard::KeyCode::End,
                            winit::keyboard::KeyCode::Help => magma_input::keyboard::KeyCode::Help,
                            winit::keyboard::KeyCode::Home => magma_input::keyboard::KeyCode::Home,
                            winit::keyboard::KeyCode::Insert => {
                                magma_input::keyboard::KeyCode::Insert
                            }
                            winit::keyboard::KeyCode::PageDown => {
                                magma_input::keyboard::KeyCode::PageDown
                            }
                            winit::keyboard::KeyCode::PageUp => {
                                magma_input::keyboard::KeyCode::PageUp
                            }
                            winit::keyboard::KeyCode::ArrowDown => {
                                magma_input::keyboard::KeyCode::ArrowDown
                            }
                            winit::keyboard::KeyCode::ArrowLeft => {
                                magma_input::keyboard::KeyCode::ArrowLeft
                            }
                            winit::keyboard::KeyCode::ArrowRight => {
                                magma_input::keyboard::KeyCode::ArrowRight
                            }
                            winit::keyboard::KeyCode::ArrowUp => {
                                magma_input::keyboard::KeyCode::ArrowUp
                            }
                            winit::keyboard::KeyCode::NumLock => {
                                magma_input::keyboard::KeyCode::NumLock
                            }
                            winit::keyboard::KeyCode::Numpad0 => {
                                magma_input::keyboard::KeyCode::Numpad0
                            }
                            winit::keyboard::KeyCode::Numpad1 => {
                                magma_input::keyboard::KeyCode::Numpad1
                            }
                            winit::keyboard::KeyCode::Numpad2 => {
                                magma_input::keyboard::KeyCode::Numpad2
                            }
                            winit::keyboard::KeyCode::Numpad3 => {
                                magma_input::keyboard::KeyCode::Numpad3
                            }
                            winit::keyboard::KeyCode::Numpad4 => {
                                magma_input::keyboard::KeyCode::Numpad4
                            }
                            winit::keyboard::KeyCode::Numpad5 => {
                                magma_input::keyboard::KeyCode::Numpad5
                            }
                            winit::keyboard::KeyCode::Numpad6 => {
                                magma_input::keyboard::KeyCode::Numpad6
                            }
                            winit::keyboard::KeyCode::Numpad7 => {
                                magma_input::keyboard::KeyCode::Numpad7
                            }
                            winit::keyboard::KeyCode::Numpad8 => {
                                magma_input::keyboard::KeyCode::Numpad8
                            }
                            winit::keyboard::KeyCode::Numpad9 => {
                                magma_input::keyboard::KeyCode::Numpad9
                            }
                            winit::keyboard::KeyCode::NumpadAdd => {
                                magma_input::keyboard::KeyCode::NumpadAdd
                            }
                            winit::keyboard::KeyCode::NumpadBackspace => {
                                magma_input::keyboard::KeyCode::NumpadBackspace
                            }
                            winit::keyboard::KeyCode::NumpadClear => {
                                magma_input::keyboard::KeyCode::NumpadClear
                            }
                            winit::keyboard::KeyCode::NumpadClearEntry => {
                                magma_input::keyboard::KeyCode::NumpadClearEntry
                            }
                            winit::keyboard::KeyCode::NumpadComma => {
                                magma_input::keyboard::KeyCode::NumpadComma
                            }
                            winit::keyboard::KeyCode::NumpadDecimal => {
                                magma_input::keyboard::KeyCode::NumpadDecimal
                            }
                            winit::keyboard::KeyCode::NumpadDivide => {
                                magma_input::keyboard::KeyCode::NumpadDivide
                            }
                            winit::keyboard::KeyCode::NumpadEnter => {
                                magma_input::keyboard::KeyCode::NumpadEnter
                            }
                            winit::keyboard::KeyCode::NumpadEqual => {
                                magma_input::keyboard::KeyCode::NumpadEqual
                            }
                            winit::keyboard::KeyCode::NumpadHash => {
                                magma_input::keyboard::KeyCode::NumpadHash
                            }
                            winit::keyboard::KeyCode::NumpadMemoryAdd => {
                                magma_input::keyboard::KeyCode::NumpadMemoryAdd
                            }
                            winit::keyboard::KeyCode::NumpadMemoryClear => {
                                magma_input::keyboard::KeyCode::NumpadMemoryClear
                            }
                            winit::keyboard::KeyCode::NumpadMemoryRecall => {
                                magma_input::keyboard::KeyCode::NumpadMemoryRecall
                            }
                            winit::keyboard::KeyCode::NumpadMemoryStore => {
                                magma_input::keyboard::KeyCode::NumpadMemoryStore
                            }
                            winit::keyboard::KeyCode::NumpadMemorySubtract => {
                                magma_input::keyboard::KeyCode::NumpadMemorySubtract
                            }
                            winit::keyboard::KeyCode::NumpadMultiply => {
                                magma_input::keyboard::KeyCode::NumpadMultiply
                            }
                            winit::keyboard::KeyCode::NumpadParenLeft => {
                                magma_input::keyboard::KeyCode::NumpadParenLeft
                            }
                            winit::keyboard::KeyCode::NumpadParenRight => {
                                magma_input::keyboard::KeyCode::NumpadParenRight
                            }
                            winit::keyboard::KeyCode::NumpadStar => {
                                magma_input::keyboard::KeyCode::NumpadStar
                            }
                            winit::keyboard::KeyCode::NumpadSubtract => {
                                magma_input::keyboard::KeyCode::NumpadSubtract
                            }
                            winit::keyboard::KeyCode::Escape => {
                                magma_input::keyboard::KeyCode::Escape
                            }
                            winit::keyboard::KeyCode::Fn => magma_input::keyboard::KeyCode::Fn,
                            winit::keyboard::KeyCode::FnLock => {
                                magma_input::keyboard::KeyCode::FnLock
                            }
                            winit::keyboard::KeyCode::PrintScreen => {
                                magma_input::keyboard::KeyCode::PrintScreen
                            }
                            winit::keyboard::KeyCode::ScrollLock => {
                                magma_input::keyboard::KeyCode::ScrollLock
                            }
                            winit::keyboard::KeyCode::Pause => {
                                magma_input::keyboard::KeyCode::Pause
                            }
                            winit::keyboard::KeyCode::BrowserBack => {
                                magma_input::keyboard::KeyCode::BrowserBack
                            }
                            winit::keyboard::KeyCode::BrowserFavorites => {
                                magma_input::keyboard::KeyCode::BrowserFavorites
                            }
                            winit::keyboard::KeyCode::BrowserForward => {
                                magma_input::keyboard::KeyCode::BrowserForward
                            }
                            winit::keyboard::KeyCode::BrowserHome => {
                                magma_input::keyboard::KeyCode::BrowserHome
                            }
                            winit::keyboard::KeyCode::BrowserRefresh => {
                                magma_input::keyboard::KeyCode::BrowserRefresh
                            }
                            winit::keyboard::KeyCode::BrowserSearch => {
                                magma_input::keyboard::KeyCode::BrowserSearch
                            }
                            winit::keyboard::KeyCode::BrowserStop => {
                                magma_input::keyboard::KeyCode::BrowserStop
                            }
                            winit::keyboard::KeyCode::Eject => {
                                magma_input::keyboard::KeyCode::Eject
                            }
                            winit::keyboard::KeyCode::LaunchApp1 => {
                                magma_input::keyboard::KeyCode::LaunchApp1
                            }
                            winit::keyboard::KeyCode::LaunchApp2 => {
                                magma_input::keyboard::KeyCode::LaunchApp2
                            }
                            winit::keyboard::KeyCode::LaunchMail => {
                                magma_input::keyboard::KeyCode::LaunchMail
                            }
                            winit::keyboard::KeyCode::MediaPlayPause => {
                                magma_input::keyboard::KeyCode::MediaPlayPause
                            }
                            winit::keyboard::KeyCode::MediaSelect => {
                                magma_input::keyboard::KeyCode::MediaSelect
                            }
                            winit::keyboard::KeyCode::MediaStop => {
                                magma_input::keyboard::KeyCode::MediaStop
                            }
                            winit::keyboard::KeyCode::MediaTrackNext => {
                                magma_input::keyboard::KeyCode::MediaTrackNext
                            }
                            winit::keyboard::KeyCode::MediaTrackPrevious => {
                                magma_input::keyboard::KeyCode::MediaTrackPrevious
                            }
                            winit::keyboard::KeyCode::Power => {
                                magma_input::keyboard::KeyCode::Power
                            }
                            winit::keyboard::KeyCode::Sleep => {
                                magma_input::keyboard::KeyCode::Sleep
                            }
                            winit::keyboard::KeyCode::AudioVolumeDown => {
                                magma_input::keyboard::KeyCode::AudioVolumeDown
                            }
                            winit::keyboard::KeyCode::AudioVolumeMute => {
                                magma_input::keyboard::KeyCode::AudioVolumeMute
                            }
                            winit::keyboard::KeyCode::AudioVolumeUp => {
                                magma_input::keyboard::KeyCode::AudioVolumeUp
                            }
                            winit::keyboard::KeyCode::WakeUp => {
                                magma_input::keyboard::KeyCode::WakeUp
                            }
                            winit::keyboard::KeyCode::Meta => magma_input::keyboard::KeyCode::Meta,
                            winit::keyboard::KeyCode::Hyper => {
                                magma_input::keyboard::KeyCode::Hyper
                            }
                            winit::keyboard::KeyCode::Turbo => {
                                magma_input::keyboard::KeyCode::Turbo
                            }
                            winit::keyboard::KeyCode::Abort => {
                                magma_input::keyboard::KeyCode::Abort
                            }
                            winit::keyboard::KeyCode::Resume => {
                                magma_input::keyboard::KeyCode::Resume
                            }
                            winit::keyboard::KeyCode::Suspend => {
                                magma_input::keyboard::KeyCode::Suspend
                            }
                            winit::keyboard::KeyCode::Again => {
                                magma_input::keyboard::KeyCode::Again
                            }
                            winit::keyboard::KeyCode::Copy => magma_input::keyboard::KeyCode::Copy,
                            winit::keyboard::KeyCode::Cut => magma_input::keyboard::KeyCode::Cut,
                            winit::keyboard::KeyCode::Find => magma_input::keyboard::KeyCode::Find,
                            winit::keyboard::KeyCode::Open => magma_input::keyboard::KeyCode::Open,
                            winit::keyboard::KeyCode::Paste => {
                                magma_input::keyboard::KeyCode::Paste
                            }
                            winit::keyboard::KeyCode::Props => {
                                magma_input::keyboard::KeyCode::Props
                            }
                            winit::keyboard::KeyCode::Select => {
                                magma_input::keyboard::KeyCode::Select
                            }
                            winit::keyboard::KeyCode::Undo => magma_input::keyboard::KeyCode::Undo,
                            winit::keyboard::KeyCode::Hiragana => {
                                magma_input::keyboard::KeyCode::Hiragana
                            }
                            winit::keyboard::KeyCode::Katakana => {
                                magma_input::keyboard::KeyCode::Katakana
                            }
                            winit::keyboard::KeyCode::F1 => magma_input::keyboard::KeyCode::F1,
                            winit::keyboard::KeyCode::F2 => magma_input::keyboard::KeyCode::F2,
                            winit::keyboard::KeyCode::F3 => magma_input::keyboard::KeyCode::F3,
                            winit::keyboard::KeyCode::F4 => magma_input::keyboard::KeyCode::F4,
                            winit::keyboard::KeyCode::F5 => magma_input::keyboard::KeyCode::F5,
                            winit::keyboard::KeyCode::F6 => magma_input::keyboard::KeyCode::F6,
                            winit::keyboard::KeyCode::F7 => magma_input::keyboard::KeyCode::F7,
                            winit::keyboard::KeyCode::F8 => magma_input::keyboard::KeyCode::F8,
                            winit::keyboard::KeyCode::F9 => magma_input::keyboard::KeyCode::F9,
                            winit::keyboard::KeyCode::F10 => magma_input::keyboard::KeyCode::F10,
                            winit::keyboard::KeyCode::F11 => magma_input::keyboard::KeyCode::F11,
                            winit::keyboard::KeyCode::F12 => magma_input::keyboard::KeyCode::F12,
                            winit::keyboard::KeyCode::F13 => magma_input::keyboard::KeyCode::F13,
                            winit::keyboard::KeyCode::F14 => magma_input::keyboard::KeyCode::F14,
                            winit::keyboard::KeyCode::F15 => magma_input::keyboard::KeyCode::F15,
                            winit::keyboard::KeyCode::F16 => magma_input::keyboard::KeyCode::F16,
                            winit::keyboard::KeyCode::F17 => magma_input::keyboard::KeyCode::F17,
                            winit::keyboard::KeyCode::F18 => magma_input::keyboard::KeyCode::F18,
                            winit::keyboard::KeyCode::F19 => magma_input::keyboard::KeyCode::F19,
                            winit::keyboard::KeyCode::F20 => magma_input::keyboard::KeyCode::F20,
                            winit::keyboard::KeyCode::F21 => magma_input::keyboard::KeyCode::F21,
                            winit::keyboard::KeyCode::F22 => magma_input::keyboard::KeyCode::F22,
                            winit::keyboard::KeyCode::F23 => magma_input::keyboard::KeyCode::F23,
                            winit::keyboard::KeyCode::F24 => magma_input::keyboard::KeyCode::F24,
                            winit::keyboard::KeyCode::F25 => magma_input::keyboard::KeyCode::F25,
                            winit::keyboard::KeyCode::F26 => magma_input::keyboard::KeyCode::F26,
                            winit::keyboard::KeyCode::F27 => magma_input::keyboard::KeyCode::F27,
                            winit::keyboard::KeyCode::F28 => magma_input::keyboard::KeyCode::F28,
                            winit::keyboard::KeyCode::F29 => magma_input::keyboard::KeyCode::F29,
                            winit::keyboard::KeyCode::F30 => magma_input::keyboard::KeyCode::F30,
                            winit::keyboard::KeyCode::F31 => magma_input::keyboard::KeyCode::F31,
                            winit::keyboard::KeyCode::F32 => magma_input::keyboard::KeyCode::F32,
                            winit::keyboard::KeyCode::F33 => magma_input::keyboard::KeyCode::F33,
                            winit::keyboard::KeyCode::F34 => magma_input::keyboard::KeyCode::F34,
                            winit::keyboard::KeyCode::F35 => magma_input::keyboard::KeyCode::F35,
                            _ => magma_input::keyboard::KeyCode::Unidentified,
                        },
                        winit::keyboard::PhysicalKey::Unidentified(_) => {
                            magma_input::keyboard::KeyCode::Unidentified
                        }
                    },
                    state: match event.state {
                        winit::event::ElementState::Pressed => magma_input::ButtonState::Pressed,
                        winit::event::ElementState::Released => magma_input::ButtonState::Released,
                    },
                    repeat: event.repeat,
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::MouseInput { state, button, .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(MouseButtonInput {
                    button: match button {
                        winit::event::MouseButton::Left => magma_input::mouse::MouseButton::Left,
                        winit::event::MouseButton::Right => magma_input::mouse::MouseButton::Right,
                        winit::event::MouseButton::Middle => {
                            magma_input::mouse::MouseButton::Middle
                        }
                        winit::event::MouseButton::Back => magma_input::mouse::MouseButton::Back,
                        winit::event::MouseButton::Forward => {
                            magma_input::mouse::MouseButton::Forward
                        }
                        winit::event::MouseButton::Other(id) => {
                            magma_input::mouse::MouseButton::Other(id)
                        }
                    },
                    state: match state {
                        winit::event::ElementState::Pressed => magma_input::ButtonState::Pressed,
                        winit::event::ElementState::Released => magma_input::ButtonState::Pressed,
                    },
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::MouseWheel { delta, .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(MouseScrollInput {
                    unit: match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, _) => {
                            magma_input::mouse::MouseScrollUnit::Line
                        }
                        winit::event::MouseScrollDelta::PixelDelta(_) => {
                            magma_input::mouse::MouseScrollUnit::Pixel
                        }
                    },
                    x: match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, _) => x,
                        winit::event::MouseScrollDelta::PixelDelta(delta) => delta.x as f32,
                    },
                    y: match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                        winit::event::MouseScrollDelta::PixelDelta(delta) => delta.y as f32,
                    },
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(MouseMotionInput {
                    delta: Vec2::new(delta.0 as f32, delta.1 as f32),
                })
                .unwrap(),
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // create winit windows for new window components
        self.app
            .world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                let mut window_component = window_entity.get_component_mut::<Window>().unwrap();
                if !window_component.has_window {
                    self.windows.create_winit_window(
                        &self.app.world,
                        event_loop,
                        &mut window_component,
                        window_entity.into(),
                    );
                } else if window_component.changed_attr {
                    self.windows.update_winit_window(
                        &mut window_component,
                        window_entity.into(),
                        &self.app.world,
                    );
                }
                window_component.changed_attr = false;
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowCreated {
                        window: window_entity.into(),
                    })
                    .unwrap();
            });

        let windows = self.app.world.query::<(Window,)>().unwrap();

        // exit if no windows are present
        if windows.is_empty() {
            event_loop.exit();
        } else if windows.len() < self.windows.winit_windows.len() {
            // drop winit windows without an entity
            let windows_to_drop = self
                .windows
                .window_to_entity
                .iter()
                .filter_map(|(_, entity)| {
                    if windows
                        .iter()
                        .find(|query_entity| entity.id() == query_entity.id())
                        .is_none()
                    {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for window in windows_to_drop {
                self.windows.delete_window(window);
            }
        }

        // Delete window entities which have a pending close request.
        // Their winit windows will be destroyd before the next update.
        // TODO: this could be refactored as a system, when scheduling systems at "start" or "end" is supported
        self.app
            .world
            .query::<(ClosingWindow, Window)>()
            .unwrap()
            .iter()
            .for_each(|closing_window| {
                closing_window.delete();
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowClosed {
                        window: closing_window.into(),
                    })
                    .unwrap();
            });

        // update the app
        self.app.update();
    }
}

fn winit_event_loop(app: App) {
    // create primary window
    app.world.create_entity((Window::new(),)).unwrap();
    // set up winit event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp {
        app,
        windows: Windows::new(),
    };
    event_loop.run_app(&mut app).unwrap();
}
