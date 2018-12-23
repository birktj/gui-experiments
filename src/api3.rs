use ::RenderContext;
use std::cell::RefCell;
use std::rc::Rc;

pub trait View {
    fn view<L: Layout>(&self, layout: &mut L, ctx: &mut RenderContext);
}

pub trait Layout {
    fn fill(&mut self, w: f32, h: f32);
}

struct ColumnLayout<'a, L> {
    layout: &'a mut L,
    ctx: &'a mut RenderContext,
    y: f32,
    w: f32,
    h: f32,
}

impl<'a, L: Layout> ColumnLayout<'a, L> {
    fn new(layout: &'a mut L, ctx: &'a mut RenderContext) -> Self {
        Self {
            layout,
            ctx,
        }
    }
}

pub trait Drawing {
    fn layout(&self, ctx: &mut LayoutContext);

    fn draw(&self, area: &Area, ctx: &mut RenderContext);
}

pub struct Rectangle {
    width: SizeConstraint,
    height: SizeConstraint,
}

impl Drawing for Rectangle {
    fn layout(&self, ctx: &mut LayoutContext) {
        ctx.layout_rect(self.width, self.height);
    }

    fn draw(&self, area: &Area, ctx: &mut RenderContext) {
        ctx.rectangle(area.x(), area.y(), area.width(), area.height());
    }
}

pub struct Label {
    text: String,
}

impl Drawing for Label {
    fn layout(&self, ctx: &mut LayoutContext) -> Layout {
        let [w, h] = ctx.messure_text(&self.text);
        ctx.layout_rect(SizeConstraint::exact(w), SizeConstraint::exact(h))
    }

    fn draw(&self, area: &Area, ctx: &mut RenderContext) {
        ctx.text(area.x(), area.y() + 15.0, &self.text);
    }
}

pub struct ColumnLayout {
    elements: Vec<Box<Drawing>>,
}

impl Drawing for ColumnLayout {
    fn layout(&self, ctx: &mut LayoutContext) -> Layout {
        
    }
}

pub trait View {
    fn view(&self) -> impl Drawing;
}

impl<V: View> Drawing for V {
    fn draw(&self, ctx: &mut RenderContext) {
        self.view().draw(ctx);
    }

    fn layout(&self, ctx: &mut LayoutContext) {
        self.view().layout(ctx)
    }
}

struct State {
    path: String,
    active: usize,
    elements: Vec<String>,
};

impl View for State {
    fn view(&self) -> impl Drawing {
        let mut layout = ColumnLayout::new();
        layout.add(Label::new(self.path.clone()));
        layout.spacing(50.0);

        for (i, element) in elements.enumerate() {
            if i == self.active {
                layout.add(Label::new(element).color([1.0,0.0,0.0]));
            }
            else {
                layout.add(Label::new(element));
            }
        }

        let mut row = RowLayout::new();
        row.add(Label::new("Left side".to_string()));
        //row.expand();
        row.add(Label::new("Right side".to_string()));

        layout.add(row);

        layout
    }
}


impl View for State {
    fn view<L: Layout>(&self, layout: &mut L, ctx: &mut RenderContext) {
        let mut layout = ColumnLayout::new(layout, ctx);
        layout.add(Label::new(self.path));

        layout.spacing(10.0);
        
        {
            let mut row = RowLayout::new(layout, ctx)
        }
    }
}
