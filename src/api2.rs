use ::RenderContext;
use std::cell::RefCell;
use std::rc::Rc;


// fn draw(&self, x: f32, y: f32, w: Option<f32>, h: Option<f32>) -> [f32; 2];

pub trait WidgetExt {
    fn draw(&self, ctx: &mut RenderContext, x: f32, y: f32, w: Option<f32>, h: Option<f32>) -> [f32; 2];
}

pub struct Widget<T: ?Sized> {
    child: Rc<RefCell<T>>,
}

impl<T> Widget<T> {
    pub fn new(inner: T) -> Widget<T> {
        Widget {
            child: Rc::new(RefCell::new(inner))
        }
    }
}

impl<T: ?Sized> Clone for Widget<T> {
    fn clone(&self) -> Self {
        Self {
            child: self.child.clone()
        }
    }
}

impl<T: WidgetExt + 'static> Widget<T> {
    pub fn as_widget(&self) -> Widget<WidgetExt> {
        let child = self.child.clone() as Rc<RefCell<WidgetExt>>;
        Widget {
            child
        }
    }
}

impl<T: WidgetExt + ?Sized> Widget<T> {
    fn draw(&self, ctx: &mut RenderContext, x: f32, y: f32, w: Option<f32>, h: Option<f32>) -> [f32; 2] {
        self.child.borrow().draw(ctx)
    }
}

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: String) -> Widget<Label> {
        Widget::new(Label {
            text,
        })
    }
}

impl WidgetExt for Label {
    fn draw(&self, ctx: &mut RenderContext, x: f32, y: f32, w: Option<f32>, h: Option<f32>) -> [f32; 2] {
        ctx.text(self.pos[0] + 5.0, self.pos[1] + 15.0, &self.text);
    }
}

widget!{
    impl GridLayout : GridLayoutExt {
        fn add(&mut self, widget: Widget) {
            self.children.push(widget);
        }
    }
}

impl GridLayout {
    fn add(&mut self, widget: Widget) {
        self.children.push(widget);
    }
}

trait GridLayoutExt {
    fn add(&self, widget: Widget);
}

impl WidgetBox<GridLayout> for GridLayoutExt {
    fn add(&self, widget: Widget) {
        self.child.borrow_mut().add(widget)
    }
}



pub struct Rectangle {
    pos: [f32; 2],
    size: [f32; 2],
    child: Option<Widget<WidgetExt>>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Widget<Rectangle> {
        Widget::new(Rectangle {
            pos: [0.0; 2],
            size: [width, height],
            child: None,
        })
    }

    pub fn new_with_child(child: Widget<WidgetExt>) -> Widget<Rectangle> {
        child.reposition(0.0, 0.0);
        Widget::new(Rectangle {
            pos: [0.0; 2],
            size: child.size(),
            child: Some(child),
        })
    }
}


impl WidgetExt for Rectangle {
    fn draw(&self, ctx: &mut RenderContext) {
        ctx.rectangle(self.pos[0], self.pos[1], self.size[0], self.size[1]);

        //self.child.map(|c| c.draw(ctx));A
        if let Some(ref child) = self.child {
            child.draw(ctx);
        }
    }

    fn reposition(&mut self, x: f32, y: f32) {
        self.pos = [x, y];

        if let Some(ref child) = self.child {
            child.reposition(x, y);
        }
    }

    fn size(&self) -> [f32; 2] {
        self.size
    }

    fn resize(&mut self, width: f32, height: f32) -> bool {
        match self.child {
            Some(ref c) => {
                if c.resize(width, height) {
                    self.size = c.size();
                    true
                }
                else {
                    false
                }
            }
            None => {
                self.size = [width, height];
                true
            }
        }
    }
}

pub struct ListLayout {
    pos: [f32; 2],
    size: [f32; 2],
    children: Vec<Widget<WidgetExt>>,
}

impl ListLayout {
    pub fn new() -> Widget<ListLayout> {
        Widget::new(ListLayout {
            pos: [0.0; 2],
            size: [0.0, 0.0],
            children: Vec::new(),
        })
    }

    fn add_child(&mut self, child: Widget<WidgetExt>) {
        let size = child.size();
        self.size[1] += size[1];
        if size[0] > self.size[0] {
            self.size[0] = size[0];
        }
        child.reposition()
        self.children.push(child);
    }
}


impl WidgetExt for ListLayout {
    fn draw(&self, ctx: &mut RenderContext) {
        for child
        //self.child.map(|c| c.draw(ctx));A
        if let Some(ref child) = self.child {
            child.draw(ctx);
        }
    }

    fn reposition(&mut self, x: f32, y: f32) {
        self.pos = [x, y];

        if let Some(ref child) = self.child {
            child.reposition(x, y);
        }
    }

    fn size(&self) -> [f32; 2] {
        self.size
    }

    fn resize(&mut self, width: f32, height: f32) -> bool {
        match self.child {
            Some(ref c) => {
                if c.resize(width, height) {
                    self.size = c.size();
                    true
                }
                else {
                    false
                }
            }
            None => {
                self.size = [width, height];
                true
            }
        }
    }
}
