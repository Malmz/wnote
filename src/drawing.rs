use gtk4::cairo;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;
use gtk4::gdk::prelude::*

mod imp {

    use gtk4::{cairo::ImageSurface, gdk::{DeviceTool, DeviceToolType}, graphene::Rect};

    use super::*;

    #[derive(Debug, Default)]
    pub struct Drawing {
        cr: RefCell<Option<cairo::Context>>,
        surface: RefCell<Option<ImageSurface>>,
    }

    impl Drawing {
        fn ensure_surface(&self, width: i32, height: i32) {
            let recreate = self
                .surface
                .borrow()
                .as_ref()
                .map_or(false, |s| s.width() != width || s.height() != height);

            if recreate {
                let surface = ImageSurface::create(cairo::Format::ARgb32, width, height).unwrap();
                if let Some(ref s) = *self.surface.borrow() {
                    let cr = cairo::Context::new(&surface).unwrap();
                    cr.set_source_surface(s, 0.0, 0.0).unwrap();
                    cr.paint().unwrap();
                }
                *self.cr.borrow_mut() = Some(cairo::Context::new(&surface).unwrap());
                *self.surface.borrow_mut() = Some(surface);
            }
        }

        fn apply_stroke(&self, tool: DeviceTool, x: f64, y: f64) {
            let cr = self.cr.borrow().as_ref().unwrap();
            match tool.tool_type() {
                DeviceToolType::Eraser => {
                    cr.set_line_width(arg)
                },
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Drawing {
        const NAME: &'static str = "Drawing";
        type Type = super::Drawing;
        type ParentType = gtk4::Widget;
    }

    impl ObjectImpl for Drawing {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            let gesture = gtk4::GestureStylus::new();
            
            let cr = self.cr.borrow().clone().unwrap();
            gesture.connect_down(move |_g, _x, _y| {
                cr.new_path();
            });

            
            let cr = self.cr.borrow().clone().unwrap();
            gesture.connect_motion(|g, _x, _y| {
                let tool = g.device_tool().unwrap();
                if let Some(b) = g.backlog() {
                    for e in b {
                        
                    }
                }

            });

            obj.add_controller(&gesture);
        }

        fn dispose(&self, _obj: &Self::Type) {}
    }

    impl WidgetImpl for Drawing {
        fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
            self.ensure_surface(width, height);
            self.parent_size_allocate(widget, width, height, baseline);
        }

        fn map(&self, widget: &Self::Type) {
            self.parent_map(widget);
            let allo = widget.allocation();
            self.ensure_surface(allo.width, allo.height);
        }

        fn unmap(&self, widget: &Self::Type) {
            self.cr.borrow_mut().take();
            self.surface.borrow_mut().take();
            self.parent_unmap(widget);
        }

        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk4::Snapshot) {
            let allo = widget.allocation();
            let cr = snapshot
                .append_cairo(&Rect::new(0.0, 0.0, allo.width as f32, allo.height as f32))
                .unwrap();

            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint().unwrap();
            cr.set_source_surface(self.surface.borrow().as_ref().unwrap(), 0.0, 0.0)
                .unwrap();
            cr.paint().unwrap();
        }
    }
}

glib::wrapper! {
    pub struct Drawing(ObjectSubclass<imp::Drawing>)
        @extends gtk4::Widget;
}

impl Default for Drawing {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawing {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
}
