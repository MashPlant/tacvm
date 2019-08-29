use nom::{branch::alt, bytes::complete::{escaped, tag, take_while1}, character::complete::{none_of, digit1, space0, one_of, space1, multispace0, newline}, combinator::{cut, map, map_res, flat_map}, sequence::{preceded, terminated, tuple}, multi::{separated_list, fold_many0}};
use nom_locate::LocatedSpan;
use unescape::unescape;
use crate::{program::*, VTBL, EMPTY, FUNCTION, CALL, PARAM, RETURN, REG_PREFIX, LABEL_PREFIX, BRANCH};

pub type Span<'a> = LocatedSpan<&'a str>;

type IResult<'a, O> = nom::IResult<Span<'a>, O>;

pub fn bin_op(i: Span) -> IResult<BinOp> {
  use BinOp::*;
  alt((
    map(tag("+"), |_| Add),
    map(tag("-"), |_| Sub),
    map(tag("*"), |_| Mul),
    map(tag("/"), |_| Div),
    map(tag("%"), |_| Mod),
    map(tag("&&"), |_| And),
    map(tag("||"), |_| Or),
    map(tag("=="), |_| Eq),
    map(tag("!="), |_| Ne),
    map(tag("<="), |_| Le),
    map(tag("<"), |_| Lt),
    map(tag(">="), |_| Ge),
    map(tag(">"), |_| Gt),
  ))(i)
}

pub fn un_op(i: Span) -> IResult<UnOp> {
  alt((
    map(tag("-"), |_| UnOp::Neg),
    map(tag("!"), |_| UnOp::Not),
  ))(i)
}

pub fn int(i: Span) -> IResult<i32> {
  alt((
    map_res(digit1, |s: Span| s.fragment.parse::<i32>()),
    map_res(preceded(tag("-"), cut(digit1)), |s: Span| s.fragment.parse::<i32>().map(|i| -i)),
  ))(i)
}

pub fn str(i: Span) -> IResult<&str> {
  preceded(tag("\""), cut(terminated(
    escaped(none_of(r#""\"#), '\\', one_of(r#""nrt\"#)), tag("\""))))(i)
    .map(|r| (r.0, r.1.fragment))
}

pub fn reg(i: Span) -> IResult<u32> {
  map_res(preceded(tag(REG_PREFIX), cut(digit1)), |s: Span| s.fragment.parse::<u32>())(i)
}

pub fn id(i: Span) -> IResult<&str> {
  take_while1(|ch: char| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.')(i)
    .map(|r| (r.0, r.1.fragment))
}

pub fn label(i: Span) -> IResult<u32> {
  map_res(preceded(tag(LABEL_PREFIX), cut(digit1)), |s: Span| s.fragment.parse::<u32>())(i)
}

pub fn branch(i: Span) -> IResult<u32> {
  preceded(tuple((tag(BRANCH), space1)), cut(label))(i)
}

pub fn operand(i: Span) -> IResult<Operand> {
  alt((
    map(int, Operand::Const),
    map(reg, Operand::Reg),
  ))(i)
}

pub fn mem(i: Span) -> IResult<(u32, i32)> {
  map(preceded(tuple((tag("*"), space0)),
               cut(tuple((tag("("), space0, reg, space0, alt((tag("+"), tag("-"))), space0, int, space0, tag(")"))))),
      |(_, _, base, _, op, _, off, _, _, )| (base, if op.fragment == "+" { off } else { -off }))(i)
}

pub fn call(i: Span) -> IResult<CallKind> {
  preceded(tuple((tag(CALL), space0)), cut(alt((
    map(reg, CallKind::Reg),
    map(id, CallKind::Named),
  ))))(i)
}

pub fn inst<'a>(i: Span<'a>) -> IResult<'a, RawInst> {
  use RawInstKind::*;
  let new = |kind: RawInstKind<'a>| RawInst { line: i.line, kind };
  alt((
    flat_map(tuple((reg, space0, tag("="), space0)), move |(d, _, _, _)| cut(alt((
      map(tuple((tag("("), space0, operand, space0, bin_op, space0, operand, space0, tag(")"))), move |(_, _, l, _, op, _, r, _, _)| new(Bin(op, d, l, r))),
      map(tuple((un_op, space0, operand)), move |(op, _, r)| new(Un(op, d, r))),
      map(operand, move |r| new(Mv(d, r))),
      map(call, move |c| new(Call(Some(d), c))),
      map(str, move |r| new(LStr(d, unescape(r).unwrap().into()))), // must be valid here
      map(tuple((tag(VTBL), space0, tag("<"), id, tag(">"))), move |(_, _, _, name, _)| new(LVTbl(d, name))),
      map(mem, move |(base, off)| new(Load(d, base, off))),
    )))),
    map(preceded(tuple((tag(PARAM), space1)), cut(operand)), move |r| new(Param(r))),
    map(call, move |c| new(Call(None, c))),
    map(preceded(tuple((tag(RETURN), space0)), cut(alt((
      map(tag(EMPTY), |_| None),
      map(operand, |r| Some(r)),
    )))), move |r| new(Ret(r))),
    map(branch, move |l| new(J(l))),
    map(preceded(tuple((tag("if"), space0)),
                 cut(tuple((tag("("), space0, operand, space0, alt((tag("!="), tag("=="))), space0, tag("0"), space0, tag(")"),
                            space0, branch)))), move |(_, _, r, _, z, _, _, _, _, _, l)| new(B(r, z.fragment == "==", l))),
    map(tuple((label, space0, tag(":"))), move |(l, _, _)| new(Label(l))),
    map(tuple((mem, space0, tag("="), space0, operand)), move |((base, off), _, _, _, r)| new(Store(r, base, off))),
  ))(i)
}

// their are too many traits for I to implement... just use a specific type here for convenience
fn curly_braced<'a, O>(parser: impl Fn(Span<'a>) -> nom::IResult<Span<'a>, O>) -> impl Fn(Span<'a>) -> nom::IResult<Span<'a>, Vec<O>> {
  preceded(tuple((tag("{"), multispace0)), terminated(
    separated_list(newline, preceded(space0, terminated(parser, space0))),
    tuple((multispace0, tag("}")))))
}

pub fn func(i: Span) -> IResult<RawFunc> {
  let (i, name) = preceded(tuple((tag(FUNCTION), space0, tag("("), space0)),
                           terminated(id, tuple((space0, tag(")"), multispace0))))(i)?;
  let (i, code) = cut(curly_braced(inst))(i)?;
  Ok((i, RawFunc { name, line: i.line, code }))
}

pub fn vtbl_slot<'a>(i: Span<'a>) -> IResult<RawVTblSlot<'a>> {
  use RawVTblSlotKind::*;
  let new = |kind: RawVTblSlotKind<'a>| RawVTblSlot { line: i.line, kind };
  alt((
    map(str, move |s| new(String(unescape(s).unwrap().into()))),
    map(tag(EMPTY), move |_| new(Empty)),
    map(terminated(id, tuple((space0, tag(";")))), move |name| new(FuncRef(name))),
    map(id, move |name| new(VTblRef(name))),
  ))(i)
}

pub fn vtbl(i: Span) -> IResult<RawVTbl> {
  let (i, name) = preceded(tuple((tag(VTBL), space0, tag("("), space0)),
                           terminated(id, tuple((space0, tag(")"), multispace0))))(i)?;
  let (i, data) = cut(curly_braced(vtbl_slot))(i)?;
  Ok((i, RawVTbl { name, line: i.line, data }))
}

pub fn program(i: Span) -> IResult<RawProgram> {
  enum ProgramItem<'a> {
    Func(RawFunc<'a>),
    VTbl(RawVTbl<'a>),
  }
  fold_many0(preceded(multispace0, alt((
    map(func, ProgramItem::Func),
    map(vtbl, ProgramItem::VTbl),
  ))), RawProgram { vtbl: vec![], func: vec![] }, |mut p, r| {
    match r {
      ProgramItem::Func(f) => p.func.push(f),
      ProgramItem::VTbl(v) => p.vtbl.push(v),
    }
    p
  })(i)
}