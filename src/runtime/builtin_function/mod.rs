
use crate::common::{executable::{BuiltinFunction, ExecutableLine}, errors::ChapError};

use crate::common::errors::Result;

mod assign;

mod control_flow;
mod math;
mod utils;
mod std;
mod bools;
mod strings;
mod random;
mod type_of;
mod type_conversion;
mod date_time;
mod delay;
mod debug;
mod error_handling;

pub fn closure_gen(executable: &ExecutableLine) -> Result<BuiltinFunction>{

    let function_name = executable.function_name
        .clone()
        .to_lowercase()
        .replace([' ', '_'], "");

    Ok(match function_name.as_str() {
        "assign" => assign::assign,
        "jump" => control_flow::jump::jump,
        "jumpif" => control_flow::jump_if::jump_if,
        "jumpifnot" => control_flow::jump_if_not::jump_if_not,
        "jumpifequal" | "jumpeq" => control_flow::jump_if_equal::jump_if_equal,
        "jumpifnotequal" | "jumpneq"=> control_flow::jump_if_not_equal::jump_if_not_equal,
        "newtag" => control_flow::new_tag::new_tag,

        "add" => math::add::add,
        "addmany" | "addall" => math::add_many::add_many,
        "minus" => math::minus::minus,
        "multiply" => math::multiply::multiply,
        "divide" => math::divide::divide,
        "modulus" | "mod" => math::modulus::modulus,
        "power" | "pow" => math::power::power,
        "sqrt" | "squareroot" => math::sqrt::sqrt,
        "increase" | "inc" => math::increase::increase,
        "decrease" | "dec" => math::decrease::decrease,

        "equal" | "eq" => bools::equal::equal,
        "not_equal" | "neq" => bools::not_equal::not_equal,

        "print" | "println" | "printline" | "stdout" => std::println::println,
        "input" | "stdin" => std::input::input,
        "exit" | "quit" | "kill" | "terminate" | "close" | "end" => std::exit::exit,
        _ => return Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    })
}

