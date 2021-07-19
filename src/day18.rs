#![allow(clippy::redundant_closure_call)]
use peg::parser;

parser! {
    grammar part_one() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() };
        pub rule expression() -> u64 = precedence!{
            x:(@) "+" y:@ {x + y}
            x:(@) "*" y:@ {x * y}
            --
            n:number() { n }
            "(" e:expression() ")" { e }
        }
    }
}

parser! {
    grammar part_two() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() };
        pub rule expression() -> u64 = precedence!{
            x:(@) "*" y:@ {x * y}
            --
            x:(@) "+" y:@ {x + y}
            --
            n:number() { n }
            "(" e:expression() ")" { e }
        }
    }
}

pub fn generator(input: &str) -> String {
    input.replace(' ', "")
}

pub fn part_one(data: &str) -> u64 {
    data.lines()
        .map(|x| {
            part_one::expression(x).unwrap_or_else(|_| panic!("Couldn't parse expression {}", x))
        })
        .sum()
}

pub fn part_two(data: &str) -> u64 {
    data.lines()
        .map(|x| {
            part_two::expression(x).unwrap_or_else(|_| panic!("Couldn't parse expression {}", x))
        })
        .sum()
}
