#![feature(plugin_registrar, box_syntax)]
#![feature(rustc_private, core, collections)]
#![feature(num_bits_bytes)]
#![allow(unknown_lints)]

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;
#[macro_use]
extern crate rustc_front;

// Only for the compile time checking of paths
extern crate core;
extern crate collections;

// for unicode nfc normalization
extern crate unicode_normalization;

use rustc::plugin::Registry;

#[macro_use]
pub mod utils;
pub mod consts;
pub mod types;
pub mod misc;
pub mod eq_op;
pub mod bit_mask;
pub mod ptr_arg;
pub mod needless_bool;
pub mod approx_const;
pub mod eta_reduction;
pub mod identity_op;
pub mod minmax;
pub mod mut_mut;
pub mod len_zero;
pub mod attrs;
pub mod collapsible_if;
pub mod unicode;
pub mod shadow;
pub mod strings;
pub mod methods;
pub mod returns;
pub mod lifetimes;
pub mod loops;
pub mod ranges;
pub mod matches;
pub mod precedence;

mod reexport {
    pub use syntax::ast::{Name, Ident, NodeId};
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box types::TypePass);
    reg.register_late_lint_pass(box misc::TopLevelRefPass);
    reg.register_late_lint_pass(box misc::CmpNan);
    reg.register_late_lint_pass(box eq_op::EqOp);
    reg.register_late_lint_pass(box bit_mask::BitMask);
    reg.register_late_lint_pass(box ptr_arg::PtrArg);
    reg.register_late_lint_pass(box needless_bool::NeedlessBool);
    reg.register_late_lint_pass(box approx_const::ApproxConstant);
    reg.register_late_lint_pass(box misc::FloatCmp);
    reg.register_early_lint_pass(box precedence::Precedence);
    reg.register_late_lint_pass(box eta_reduction::EtaPass);
    reg.register_late_lint_pass(box identity_op::IdentityOp);
    reg.register_late_lint_pass(box mut_mut::MutMut);
    reg.register_late_lint_pass(box len_zero::LenZero);
    reg.register_late_lint_pass(box misc::CmpOwned);
    reg.register_late_lint_pass(box attrs::AttrPass);
    reg.register_late_lint_pass(box collapsible_if::CollapsibleIf);
    reg.register_late_lint_pass(box misc::ModuloOne);
    reg.register_late_lint_pass(box unicode::Unicode);
    reg.register_late_lint_pass(box strings::StringAdd);
    reg.register_late_lint_pass(box returns::ReturnPass);
    reg.register_late_lint_pass(box methods::MethodsPass);
    reg.register_late_lint_pass(box shadow::ShadowPass);
    reg.register_late_lint_pass(box types::LetPass);
    reg.register_late_lint_pass(box types::UnitCmp);
    reg.register_late_lint_pass(box loops::LoopsPass);
    reg.register_late_lint_pass(box lifetimes::LifetimePass);
    reg.register_late_lint_pass(box ranges::StepByZero);
    reg.register_late_lint_pass(box types::CastPass);
    reg.register_late_lint_pass(box types::TypeComplexityPass);
    reg.register_late_lint_pass(box matches::MatchPass);
    reg.register_late_lint_pass(box misc::PatternPass);
    reg.register_late_lint_pass(box minmax::MinMaxPass);

    reg.register_lint_group("clippy_pedantic", vec![
        methods::OPTION_UNWRAP_USED,
        methods::RESULT_UNWRAP_USED,
        methods::WRONG_PUB_SELF_CONVENTION,
        mut_mut::MUT_MUT,
        ptr_arg::PTR_ARG,
        shadow::SHADOW_REUSE,
        shadow::SHADOW_SAME,
        shadow::SHADOW_UNRELATED,
        strings::STRING_ADD,
        strings::STRING_ADD_ASSIGN,
        types::CAST_POSSIBLE_TRUNCATION,
        types::CAST_POSSIBLE_WRAP,
        types::CAST_PRECISION_LOSS,
        types::CAST_SIGN_LOSS,
        unicode::NON_ASCII_LITERAL,
        unicode::UNICODE_NOT_NFC,
    ]);

    reg.register_lint_group("clippy", vec![
        approx_const::APPROX_CONSTANT,
        attrs::INLINE_ALWAYS,
        bit_mask::BAD_BIT_MASK,
        bit_mask::INEFFECTIVE_BIT_MASK,
        collapsible_if::COLLAPSIBLE_IF,
        eq_op::EQ_OP,
        eta_reduction::REDUNDANT_CLOSURE,
        identity_op::IDENTITY_OP,
        len_zero::LEN_WITHOUT_IS_EMPTY,
        len_zero::LEN_ZERO,
        lifetimes::NEEDLESS_LIFETIMES,
        loops::EXPLICIT_COUNTER_LOOP,
        loops::EXPLICIT_ITER_LOOP,
        loops::ITER_NEXT_LOOP,
        loops::NEEDLESS_RANGE_LOOP,
        loops::REVERSE_RANGE_LOOP,
        loops::UNUSED_COLLECT,
        loops::WHILE_LET_LOOP,
        matches::MATCH_REF_PATS,
        matches::SINGLE_MATCH,
        methods::SHOULD_IMPLEMENT_TRAIT,
        methods::STR_TO_STRING,
        methods::STRING_TO_STRING,
        methods::WRONG_SELF_CONVENTION,
        minmax::MIN_MAX,
        misc::CMP_NAN,
        misc::CMP_OWNED,
        misc::FLOAT_CMP,
        misc::MODULO_ONE,
        misc::REDUNDANT_PATTERN,
        misc::TOPLEVEL_REF_ARG,
        needless_bool::NEEDLESS_BOOL,
        precedence::PRECEDENCE,
        ranges::RANGE_STEP_BY_ZERO,
        returns::LET_AND_RETURN,
        returns::NEEDLESS_RETURN,
        types::BOX_VEC,
        types::LET_UNIT_VALUE,
        types::LINKEDLIST,
        types::TYPE_COMPLEXITY,
        types::UNIT_CMP,
        unicode::ZERO_WIDTH_SPACE,
    ]);
}
