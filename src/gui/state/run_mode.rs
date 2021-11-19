use rltk::Rltk;

#[derive(Clone, Copy, Debug)]
pub enum RunMode {
    Initial,
}

impl RunMode {

    pub fn tick(self, ctx: &mut Rltk) -> Option<RunMode> {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
        Some(RunMode::Initial)
    }

}


