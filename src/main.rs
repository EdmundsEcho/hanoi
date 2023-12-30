use anyhow::Context;

#[derive(Debug, Copy, Clone)]
enum Pin {
    A,
    B,
    C,
}
// simple solution
fn move_tower_untyped(height: u32, start: Pin, finish: Pin, temp: Pin) {
    println!("building the stack for tower with height: {:?}", &height);
    if height == 1 {
        println!("✅ {:?} to {:?}", &start, &finish);
    } else {
        move_tower_untyped(height - 1, start, temp, finish);
        println!("👉 {:?} to {:?}", &start, &finish);
        move_tower_untyped(height - 1, temp, finish, start);
    }
    println!("...🏁");
}

type Height = u32;
type Start = Pin;
type Finish = Pin;
type Temp = Pin;
struct Tower(Height);

#[derive(Clone)]
struct Step((Start, Finish));
impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start, finish) = self.0;
        write!(f, "{:?} to {:?}", &start, &finish)
    }
}

/// the collection of steps that describe the solution
#[derive(Clone)]
struct Solution(Vec<Step>);
impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut solution = String::new();
        for (i, step) in self.0.iter().enumerate() {
            solution.push_str(&format!("{}: {}\n", i, step));
        }
        write!(f, "{}", solution)
    }
}
impl Solution {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(&mut self, step: Step) {
        self.0.push(step);
    }
    fn concat(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        self
    }
    fn iter(&self) -> std::slice::Iter<'_, Step> {
        self.0.iter()
    }
}
// builder-like pattern
impl Tower {
    fn new(height: Height) -> Self {
        Self(height)
    }
    fn solution(&self) -> Solution {
        Move::start(self.0)
    }
}
struct Move {
    height: Height,
    start: Start,
    finish: Finish,
    temp: Temp,
}
impl Move {
    fn start(height: Height) -> Solution {
        Self {
            height,
            start: Pin::A,
            finish: Pin::B,
            temp: Pin::C,
        }
        .move_tower()
    }
    fn move_tower(self) -> Solution {
        // base case (recursive stack terminates here)
        if self.height == 1 {
            let mut s = Solution::new();
            s.push(Step((self.start, self.finish)));
            s
        } else {
            // create a stack of moves from start to temp
            let mut s = Move {
                height: self.height - 1, // recurse to base case
                start: self.start,       // unchanged
                finish: self.temp,
                temp: self.finish,
            }
            .move_tower();

            // push onto the completed stack, the starting move where height > 1
            s.push(Step((self.start, self.finish)));

            // finish with a stack of moves from temp to finish
            s.concat(
                Move {
                    height: self.height - 1, // recurse to base case
                    start: self.temp,
                    finish: self.finish, // unchanged
                    temp: self.start,
                }
                .move_tower(),
            )
        }
    }
}

// ------------------------------------------------------------------------------------------------
/// testing the solution
#[derive(Debug)]
struct Hanoi {
    a: Vec<u32>, // start
    b: Vec<u32>, // finish
    c: Vec<u32>, // temp
}
impl std::fmt::Display for Hanoi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hanoi = String::new();
        hanoi.push_str(&format!("a: {:?}\n", &self.a));
        hanoi.push_str(&format!("b: {:?}\n", &self.b));
        hanoi.push_str(&format!("c: {:?}\n", &self.c));
        write!(f, "{}", hanoi)
    }
}
impl Hanoi {
    fn new(height: Height) -> Self {
        Hanoi {
            a: (1..=height).rev().collect(),
            b: Vec::new(),
            c: Vec::new(),
        }
    }
}
fn apply_step(step: &Step, mut hanoi: Hanoi) -> anyhow::Result<Hanoi> {
    match step {
        Step((Pin::A, Pin::B)) => {
            let v = hanoi
                .a
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.b.push(v);
        }
        Step((Pin::A, Pin::C)) => {
            let v = hanoi
                .a
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.c.push(v);
        }
        Step((Pin::B, Pin::A)) => {
            let v = hanoi
                .b
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.a.push(v);
        }
        Step((Pin::B, Pin::C)) => {
            let v = hanoi
                .b
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.c.push(v);
        }
        Step((Pin::C, Pin::A)) => {
            let v = hanoi
                .c
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.a.push(v);
        }
        Step((Pin::C, Pin::B)) => {
            let v = hanoi
                .c
                .pop()
                .with_context(|| format!("Failed step {}\r\n{}", &step, &hanoi))?;
            hanoi.b.push(v);
        }
        _ => panic!("invalid move"),
    }
    Ok(hanoi)
}
// ------------------------------------------------------------------------------------------------
//
fn main() {
    let height = 5;
    println!("The tower with {} stories is being moved", &height);
    move_tower_untyped(height, Pin::A, Pin::B, Pin::C);
    let solution = Tower::new(height).solution();
    println!("solution:\n{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hanoi() {
        let height = 7;
        let solution = Tower::new(height).solution();
        let hanoi = solution.iter().fold(Ok(Hanoi::new(height)), |hanoi, step| {
            apply_step(step, hanoi?)
        });
        assert_eq!(hanoi.unwrap().b, (1..=height).rev().collect::<Vec<_>>());
    }
}
