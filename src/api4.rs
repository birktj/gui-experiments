pub enum Size {
    Pixels(f32),
    Precent(f32),
    Fill(u32),
}

fn solve_size(size: f32, constraints: &[Size]) -> Vec<f32> {
    let mut tot_fill = 0;
    let mut tot_size = 0.0;

    let mut res = vec![0; constraints.size()];

    for i in 0..constraints.len() {
        match constraints[i] {
            Size::Pixels(x) => {
                res[i] = x;
                tot_size += res[i];
            }
            Size::Precent(x) => {
                res[i] = x * size;
                tot_size += res[i];
            }
            Size::Fill(x) => {
                tot_fill += x;
            }
        }
    }
    for i in 0..constraints.len() {
        match constraints[i] {
            SizeConstraint::Fill(x) => if size <= tot_size {
                res[i] = x as f32 / tot_fill as f32 * (size - tot_size);
            }
            _ => (),
        }
    }

    res
}

pub trait Drawing {
    fn size(&self) -> (Size, Size);

    fn draw(&self, area: &Area, ctx: &mut RenderContext);
}

pub struct Rectangle {
    width: Size,
    height: Size,
}

impl Drawing for Rectangle {
    fn size(&self, layout_ctx: &LayoutContext) -> (Size, Size) {
        (self.width, self.height)
    }

    fn draw(&self, area: &Area, ctx: &mut RenderContext) {
        ctx.rectangle(area.x(), area.y(), area.width(), area.height());
    }
}

pub struct ColumnLayout {
    elements: Vec<Box<Drawing>>,
    width: Size,
    height: Size,
}

impl Drawing for ColumnLayout {
    fn size(&self) -> (Size, Size) {
        (self.width, self.height)
    }

    fn draw(&self, area: &Area, ctx: &mut RenderContext) {
        
    }
}
