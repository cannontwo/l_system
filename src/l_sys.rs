use std::collections::HashSet;

use turtle::Turtle;

/// L-System definition struct
pub struct System<F>
where
    F: Fn(char) -> String,
{
    /// The system alphabet defines the set of characters valid in this system.
    pub alphabet: HashSet<char>,
    /// The system start (or axiom) defines the initial state of string in this system.
    pub start: String,
    /// The production function defines how characters in the system evolve. This production rule
    /// should evaluate to the identity for all characters not in the system alphabet.
    pub production: F,
}

/// Definition struct for L-Systems that can be drawn by a Turtle program.
pub struct TurtleSystem<F, G>
where
    F: Fn(char) -> String,
    G: Fn(char, &mut Turtle) -> (),
{
    /// The system alphabet defines the set of characters valid in this system.
    pub alphabet: HashSet<char>,
    /// The system start (or axiom) defines the initial state of string in this system.
    pub start: String,
    /// The production function defines how characters in the system evolve. This production rule
    /// should evaluate to the identity for all characters not in the system alphabet.
    pub production: F,
    /// Drawing function defining how the alphabet of this system is drawn.
    pub draw: G,
}

impl<F> System<F>
where
    F: Fn(char) -> String,
{
    pub fn new(alphabet: HashSet<char>, start: String, production: F) -> System<F> {
        System {
            alphabet,
            start,
            production,
        }
    }

    pub fn begin(&self) -> String {
        self.start.clone()
    }

    pub fn step(&self, state: &str) -> String {
        let new_state = state.chars().map(|c| (self.production)(c));
        new_state.reduce(|x, y| format!("{}{}", x, y)).unwrap()
    }
}

impl<F, G> TurtleSystem<F, G>
where
    F: Fn(char) -> String,
    G: Fn(char, &mut Turtle) -> (),
{
    pub fn new(
        alphabet: HashSet<char>,
        start: String,
        production: F,
        draw: G,
    ) -> TurtleSystem<F, G> {
        TurtleSystem {
            alphabet,
            start,
            production,
            draw,
        }
    }

    pub fn begin(&self) -> String {
        self.start.clone()
    }

    pub fn step(&self, state: &str) -> String {
        let new_state = state.chars().map(|c| (self.production)(c));
        new_state.reduce(|x, y| format!("{}{}", x, y)).unwrap()
    }

    pub fn draw(&self, state: &str, turtle: &mut Turtle) {
        state.chars().for_each(|c| (self.draw)(c, turtle));
    }
}
