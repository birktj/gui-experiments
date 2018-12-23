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

pub enum Drawing {
    Rectangle(Size, Size),
    Text(String),
    // StackStart,
    // StackEnd,
    Stack(Vec<Drawing>),
}

pub struct Drawing<D, L> {
    draw: Box<Fn(frame: [f32; 4], ctx: &mut RenderContext)>,
    layout: Box<Fn()>
}

impl<D, L> Drawing<D, L> {
    pub fn rectangle(width: f32, height: f32) -> Self {
        
    }
}

impl Drawing {
    
}
