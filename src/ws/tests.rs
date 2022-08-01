// Copyright (C) 2022 Andrew Archibald
//
// Nebula 2 is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with Nebula 2. If not, see http://www.gnu.org/licenses/.

use bitvec::prelude::*;

use crate::syntax::PrefixParser;
use crate::text::EncodingError;
use crate::ws::inst::{Inst, RawInst};
use crate::ws::parse::TABLE;
use crate::ws::token::{
    bit_pack_padded, bit_unpack_padded, Lexer, Mapping, MappingLexer, Token, Token::*,
};

const TUTORIAL_STL: &[u8] = br"
S S S T L                    push 1
L S S S T S S S S T T L  label_C:
S L S                        dup
T L S T                      printi
S S S T S T S L              push 10
T L S S                      printc
S S S T L                    push 1
T S S S                      add
S L S                        dup
S S S T S T T L              push 11
T S S T                      sub
L T S S T S S S T S T L      jz label_E
L S L S T S S S S T T L      jmp label_C
L S S S T S S S T S T L  label_E:
S L L                        drop
L L L                        end
";

const TUTORIAL_TOKENS: &[Token] = &[
    S, S, S, T, L, L, S, S, S, T, S, S, S, S, T, T, L, S, L, S, T, L, S, T, S, S, S, T, S, T, S, L,
    T, L, S, S, S, S, S, T, L, T, S, S, S, S, L, S, S, S, S, T, S, T, T, L, T, S, S, T, L, T, S, S,
    T, S, S, S, T, S, T, L, L, S, L, S, T, S, S, S, S, T, T, L, L, S, S, S, T, S, S, S, T, S, T, L,
    S, L, L, L, L, L,
];

const TUTORIAL_BITS: &[u8] = &[
    0b00010111, 0b10001000, 0b00101011, 0b01101011, 0b01000010, 0b01001110, 0b11000001, 0b01110000,
    0b01100001, 0b00101011, 0b10001011, 0b10001000, 0b01001011, 0b11011010, 0b00001010, 0b11110001,
    0b00001001, 0b01101111, 0b11111100,
];

fn get_tutorial_insts() -> Vec<RawInst> {
    vec![
        Inst::Push(bitvec![0, 1]),
        Inst::Label(bitvec![0, 1, 0, 0, 0, 0, 1, 1]),
        Inst::Dup,
        Inst::Printi,
        Inst::Push(bitvec![0, 1, 0, 1, 0]),
        Inst::Printc,
        Inst::Push(bitvec![0, 1]),
        Inst::Add,
        Inst::Dup,
        Inst::Push(bitvec![0, 1, 0, 1, 1]),
        Inst::Sub,
        Inst::Jz(bitvec![0, 1, 0, 0, 0, 1, 0, 1]),
        Inst::Jmp(bitvec![0, 1, 0, 0, 0, 0, 1, 1]),
        Inst::Label(bitvec![0, 1, 0, 0, 0, 1, 0, 1]),
        Inst::Drop,
        Inst::End,
    ]
}

#[test]
fn lex() -> Result<(), EncodingError> {
    let lex = MappingLexer::new_utf8(TUTORIAL_STL, Mapping::<char>::STL, true);
    let toks = lex.collect::<Result<Vec<_>, EncodingError>>()?;
    assert_eq!(TUTORIAL_TOKENS, toks);
    Ok(())
}

#[test]
fn byte_lex() -> Result<(), EncodingError> {
    let lex = MappingLexer::new_bytes(TUTORIAL_STL, Mapping::<u8>::STL);
    let toks = lex.collect::<Result<Vec<_>, EncodingError>>()?;
    assert_eq!(TUTORIAL_TOKENS, toks);
    Ok(())
}

#[test]
fn bit_pack() -> Result<(), EncodingError> {
    let bits = bit_pack_padded::<u8, Msb0>(TUTORIAL_TOKENS);
    assert_eq!(TUTORIAL_BITS, bits);
    Ok(())
}

#[test]
fn bit_unpack() -> Result<(), EncodingError> {
    let toks = bit_unpack_padded::<u8, Msb0>(TUTORIAL_BITS);
    assert_eq!(TUTORIAL_TOKENS, toks);
    Ok(())
}

#[test]
fn parse() {
    let lex = MappingLexer::new_utf8(TUTORIAL_STL, Mapping::<char>::STL, true);
    let parser = PrefixParser::new(&*TABLE, lex);
    let insts = parser.collect::<Vec<_>>();
    assert_eq!(get_tutorial_insts(), insts);
}

#[test]
fn parse_dyn() {
    let lexers: [Box<dyn Lexer>; 3] = [
        box MappingLexer::new_utf8(TUTORIAL_STL, Mapping::<char>::STL, true),
        box MappingLexer::new_bytes(TUTORIAL_STL, Mapping::<u8>::STL),
        box bit_unpack_padded::<u8, Msb0>(TUTORIAL_BITS)
            .into_iter()
            .map(Ok),
    ];
    for lex in lexers {
        let parser = PrefixParser::new(&*TABLE, lex);
        let insts = parser.collect::<Vec<_>>();
        assert_eq!(get_tutorial_insts(), insts);
    }
}
