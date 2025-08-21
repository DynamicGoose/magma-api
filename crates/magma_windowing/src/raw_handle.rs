use std::{any::Any, marker::PhantomData, ops::Deref, sync::Arc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

/// A Wrapper for windows
#[derive(Clone, Debug)]
pub struct WindowWrapper<W> {
    reference: Arc<dyn Any + Send + Sync>,
    window_type: PhantomData<W>,
}

impl<W: Send + Sync + 'static> WindowWrapper<W> {
    pub fn new(window: W) -> Self {
        Self {
            reference: Arc::new(window),
            window_type: PhantomData,
        }
    }
}

impl<W: 'static> Deref for WindowWrapper<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.reference.downcast_ref::<W>().unwrap()
    }
}

/// A thread safe wrapper over [`RawWindowHandle`] and [`RawDisplayHandle`].
#[derive(Clone, Debug)]
pub struct RawHandleWrapper {
    _window: Arc<dyn Any + Send + Sync>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl RawHandleWrapper {
    pub fn new<W: HasWindowHandle + HasDisplayHandle + 'static>(window: &WindowWrapper<W>) -> Self {
        Self {
            _window: window.reference.clone(),
            window_handle: window.window_handle().unwrap().as_raw(),
            display_handle: window.display_handle().unwrap().as_raw(),
        }
    }

    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }

    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }
}

unsafe impl Send for RawHandleWrapper {}
unsafe impl Sync for RawHandleWrapper {}
