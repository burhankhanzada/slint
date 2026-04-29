// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use winit::dpi::PhysicalSize;

use euclid::{Scale, Size2D};

use slint::{ComponentHandle, language::ColorScheme};

use servo::{DevicePixel, DevicePoint, DeviceVector2D, Scroll, Theme};

use crate::{MyApp, WebviewLogic, webview::SlintServoAdapter};

use super::adapter::upgrade_adapter;
use super::events_utils::{
    convert_input_string_to_servo_url, convert_slint_key_event_to_servo_input_event,
    convert_slint_pointer_event_to_servo_input_event,
    convert_slint_text_input_to_servo_input_events,
};

pub struct WebViewEvents<'a> {
    app: &'a MyApp,
    adapter: Rc<SlintServoAdapter>,
}

impl<'a> WebViewEvents<'a> {
    pub fn new(app: &'a MyApp, adapter: Rc<SlintServoAdapter>) {
        let instance = Self { app, adapter };
        instance.on_url();
        instance.on_theme();
        instance.on_resize();
        instance.on_scroll();
        instance.on_buttons();
        instance.on_pointer();
        instance.on_key_event();
        instance.on_text_input();
    }

    fn on_url(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_loadUrl(move |url| {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            let url = convert_input_string_to_servo_url(&url);
            webview.load(url.into_url());
        });
    }

    fn on_theme(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_theme(move |color_scheme| {
            let theme = if color_scheme == ColorScheme::Dark { Theme::Dark } else { Theme::Light };
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            // Theme not updating until mouse move over it
            // https://github.com/servo/servo/issues/40268
            webview.notify_theme_change(theme);
        });
    }

    fn on_resize(&self) {
        let app_weak = self.app.as_weak();
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_resize(move |width, height| {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();

            let scale_factor =
                app_weak.upgrade().expect("Failed to upgrade app").window().scale_factor();
            let scale = Scale::new(scale_factor);

            webview.set_hidpi_scale_factor(scale);

            let size: Size2D<f32, DevicePixel> = Size2D::new(width, height);
            let physical_size = PhysicalSize::new(size.width as u32, size.height as u32);

            webview.resize(physical_size);
        });
    }

    fn on_scroll(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_scroll(
            move |initial_x, initial_y, delta_x, delta_y| {
                let adapter = upgrade_adapter(&adapter_weak);
                let webview = adapter.webview();

                let point = DevicePoint::new(initial_x, initial_y);
                let moved_by = DeviceVector2D::new(delta_x, delta_y);
                // Invert delta to match Servo's coordinate system
                let servo_delta = -moved_by;

                webview.notify_scroll_event(Scroll::Delta(servo_delta.into()), point.into());
            },
        );
    }

    fn on_buttons(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_back(move || {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            webview.go_back(1);
        });

        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_forward(move || {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            webview.go_forward(1);
        });

        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_reload(move || {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            webview.reload();
        });
    }

    fn on_pointer(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_pointer(move |pointer_event, x, y| {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            let point = DevicePoint::new(x, y);
            let input_event =
                convert_slint_pointer_event_to_servo_input_event(&pointer_event, point.into());
            webview.notify_input_event(input_event);
        });
    }

    fn on_key_event(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        self.app.global::<WebviewLogic>().on_key_event(move |event, is_pressed| {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();
            let input_event = convert_slint_key_event_to_servo_input_event(&event, is_pressed);
            webview.notify_input_event(input_event);
        });
    }

    fn on_text_input(&self) {
        let adapter_weak = Rc::downgrade(&self.adapter);
        let last_text = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        self.app.global::<WebviewLogic>().on_text_input(move |text, start, end| {
            let adapter = upgrade_adapter(&adapter_weak);
            let webview = adapter.webview();

            let mut last = last_text.borrow_mut();
            let text_str = text.as_str();

            // Android often sends the full text with a replacement range of all.
            // To avoid duplicates in the webview (which likely appends characters),
            // we only send the delta if the new text extends the previous one.
            if start == i32::MIN && end == i32::MAX && text_str.starts_with(&*last) {
                let delta = &text_str[last.len()..];
                if !delta.is_empty() {
                    let input_events = convert_slint_text_input_to_servo_input_events(delta);
                    for event in input_events {
                        webview.notify_input_event(event);
                    }
                }
            } else {
                // For other cases, send the full text. 
                // Note: This might still cause some duplication if the webview has state,
                // but it's the best we can do without a full IME implementation in the bridge.
                let input_events = convert_slint_text_input_to_servo_input_events(text_str);
                for event in input_events {
                    webview.notify_input_event(event);
                }
            }
            *last = text_str.to_string();
        });
    }
}
