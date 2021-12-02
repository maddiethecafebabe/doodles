use super::Command;
use core::mem;

#[derive(Debug)]
pub struct VmContext {
    pub pos_horizontal: usize,
    pub depth: usize,
    pub aim: usize,
}

impl VmContext {
    fn new() -> Self {
        Self {
            pos_horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn reset(&mut self) -> Self {
        mem::replace(self, Self::new())
    }
}

pub struct HooksBuilder {
    forward: Option<Box<dyn FnMut(&mut VmContext, usize)>>,
    down: Option<Box<dyn FnMut(&mut VmContext, usize)>>,
    up: Option<Box<dyn FnMut(&mut VmContext, usize)>>,
}

impl HooksBuilder {
    pub fn new() -> Self {
        Self {
            forward: None,
            down: None,
            up: None,
        }
    }

    pub fn forward(mut self, f: impl FnMut(&mut VmContext, usize) + 'static) -> Self {
        self.forward = Some(Box::new(f));
        self
    }

     pub fn down(mut self, f: impl FnMut(&mut VmContext, usize) + 'static) -> Self {
        self.down = Some(Box::new(f));
        self
    }

      pub fn up(mut self, f: impl FnMut(&mut VmContext, usize) + 'static) -> Self {
        self.up = Some(Box::new(f));
        self
    }

    pub fn build(self) -> VmHooks {
        VmHooks {
            forward: self.forward.unwrap(),
            down: self.down.unwrap(),
            up: self.up.unwrap(),
        }
    }
}

pub struct VmHooks {
    forward: Box<dyn FnMut(&mut VmContext, usize)>,
    down: Box<dyn FnMut(&mut VmContext, usize)>,
    up: Box<dyn FnMut(&mut VmContext, usize)>,
}

pub struct Vm {
    ctx: VmContext,
    hooks: VmHooks,
}

impl Vm {
    pub fn new(hooks: VmHooks) -> Self {
        Self {
            ctx: VmContext::new(),
            hooks
        }
    }

    pub fn run_command_stream<'a>(&mut self, commands: impl Iterator<Item = &'a Command>) -> &mut Self {
        use Command::*;
        
        for cmd in commands {
            match cmd {
                Forward(x) => (self.hooks.forward)(&mut self.ctx, *x),
                Down(x) => (self.hooks.down)(&mut self.ctx, *x),
                Up(x) => (self.hooks.up)(&mut self.ctx, *x),
            }
        }

        self
    }

    pub fn reset(&mut self) -> VmContext {
        self.ctx.reset()
    }

    pub fn hooks_mut(&mut self) -> &mut VmHooks {
        &mut self.hooks
    }

    pub fn pos_horizontal(&self) -> usize {
        self.ctx.pos_horizontal
    }

    pub fn depth(&self) -> usize {
        self.ctx.depth
    }

    pub fn aim(&self) -> usize {
        self.ctx.aim
    }

    pub fn inspect(&self, f: impl FnOnce(&VmContext)) -> &Self {
        f(&self.ctx);
        self
    }
}
