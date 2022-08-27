mod l_sys;

use std::collections::HashSet;

use turtle::Turtle;

use l_sys::{System, TurtleSystem};

fn main() {
    let alph: HashSet<char> = vec!['a', 'b'].into_iter().collect();
    let start = "a".to_owned();

    let system = System::new(alph, start, |c| match c {
        'a' => "ab".to_owned(),
        'b' => "a".to_owned(),
        n => n.to_string(),
    });

    let mut state = system.begin();
    println!("Start state is {state}");

    for i in 1..10 {
        state = system.step(&state);
        println!("State {i} is {state}");
    }

    let t_alph: HashSet<char> = vec!['f', 'g', '+', '-'].into_iter().collect();
    let t_start = "f-g-g".to_owned();
    let t_system = TurtleSystem::new(
        t_alph,
        t_start,
        |c| match c {
            'f' => "f-g+f+g-f".to_owned(),
            'g' => "gg".to_owned(),
            n => n.to_string(),
        },
        |c, t| match c {
            'f' => t.forward(1.0),
            'g' => t.forward(1.0),
            '+' => t.left(120.0),
            '-' => t.right(120.0),
            _ => (),
        },
    );

    let mut t_state = t_system.begin();
    let mut turtle = Turtle::new();

    for _ in 0..10 {
        turtle.reset();
        turtle.use_degrees();
        turtle.set_speed("instant");
        t_state = t_system.step(&t_state);
        t_system.draw(&t_state, &mut turtle);
    }
}
