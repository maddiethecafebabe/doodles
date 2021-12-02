use shared::{Input, Reporter, computer::{commands, vm::{Vm, HooksBuilder, VmHooks}}};

fn part1_hooks() -> VmHooks {
    HooksBuilder::new()
        .forward(|ctx, x| ctx.pos_horizontal += x)
        .down(|ctx, x| ctx.depth += x)
        .up(|ctx, x| ctx.depth -= x)
        .build()
}

fn part2_hooks() -> VmHooks {
    HooksBuilder::new()
        .forward(|ctx, x| {
            ctx.pos_horizontal += x;
            ctx.depth += ctx.aim * x;
        })
        .down(|ctx, x| ctx.aim += x)
        .up(|ctx, x| ctx.aim -= x)
        .build()
}

fn main() {
    let inp = Input::from_env_args().unwrap();
    let cmds = commands::map_to_commands(inp.lines());
    let mut vm = Vm::new(part1_hooks());

    Reporter::day(2)
      .part1(|| {
            vm.run_command_stream(cmds.iter());           
            vm.depth() * vm.pos_horizontal()
        })
        .part2(|| {
            vm.reset();
            *vm.hooks_mut() = part2_hooks();

            vm.run_command_stream(cmds.iter());
            vm.depth() * vm.pos_horizontal()
        })
        .print()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn day02_part1_test() {
        let cmds = commands::map_to_commands(Input::from_str(TEST_INPUT).lines());
        let mut vm = Vm::new(part1_hooks());
        vm.run_command_stream(cmds.iter());

        assert_eq!(vm.pos_horizontal() * vm.depth(), 150);
    }

    #[test]
    fn day02_part2_test() {
        let cmds = commands::map_to_commands(Input::from_str(TEST_INPUT).lines());
        let mut vm = Vm::new(part2_hooks());
        vm.run_command_stream(cmds.iter());

        assert_eq!(vm.pos_horizontal() * vm.depth(), 900)
    }
}
