//! `Chain.*` (`chain.lua`): data kept per chain link, and named chain information.

use crate::constants::*;
use crate::lua::duel;
use crate::prelude::*;

const NS_EFFECT_DATA: u32 = crate::ns::CHAIN_DATA;

/// The fields scripts store in `e:GetChainData()`.
pub mod field {
    pub const COST_LP_PAID: u8 = 1;
    pub const COST_DISCARDED_CARDS: u8 = 2;
    pub const COST_DETACHED_MATERIALS: u8 = 3;
    pub const COST_CHOICE: u8 = 4;
    /// The first key a card script may use for its own fields (`e:GetChainData().level_of_the_tribute`):
    /// `CARD + 0`, `CARD + 1`, ... (the data belongs to the effect, so cards cannot collide)
    pub const CARD: u8 = 32;
}

/// `Duel.GetChainInfo(ch, info)` as one value (`Value::Nil` when the chain link does not exist)
fn chain_info(d: &Duel, ch: u8, info: u32) -> Value {
    duel::get_chain_info(d, ch, &[info]).first().cloned().unwrap_or(Value::Nil)
}
/// `Chain.GetTargetCards(ch)` (a group, or nil)
pub fn get_target_cards(d: &Duel, ch: u8) -> Value {
    chain_info(d, ch, CHAININFO_TARGET_CARDS)
}
/// `Chain.GetTargetPlayer(ch)`
pub fn get_target_player(d: &Duel, ch: u8) -> Value {
    chain_info(d, ch, CHAININFO_TARGET_PLAYER)
}
/// `Chain.GetTargetParam(ch)`
pub fn get_target_param(d: &Duel, ch: u8) -> Value {
    chain_info(d, ch, CHAININFO_TARGET_PARAM)
}
/// `Chain.GetDisableReason(ch)`
pub fn get_disable_reason(d: &Duel, ch: u8) -> Value {
    chain_info(d, ch, CHAININFO_DISABLE_REASON)
}
/// `Chain.GetDisablePlayer(ch)`
pub fn get_disable_player(d: &Duel, ch: u8) -> Value {
    chain_info(d, ch, CHAININFO_DISABLE_PLAYER)
}
/// `Chain.GetID(ch)`
pub fn get_id(d: &Duel, ch: u8) -> i64 {
    duel::get_chain_info(d, ch, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int())
}

/// The chain link `Chain.Data(ch)` refers to (errors as the script does).
fn data_link(d: &Duel, ch: Option<u8>) -> u8 {
    let current = duel::get_current_chain(d, false);
    if current == 0 {
        crate::lua_error("Attempted to access Chain Data table while there is currently no Chain");
    }
    match ch {
        Some(c) if c as i32 > current => crate::lua_error("Attempted to access Chain Link higher than current Chain"),
        Some(c) if c != 0 => c,
        _ => current as u8,
    }
}

fn effect_key(d: &Duel, e: EffectId, ch: Option<u8>, f: u8) -> (u32, u64) {
    let cid = get_id(d, data_link(d, ch)) as u64;
    (NS_EFFECT_DATA, (cid << 40) | ((e.0 as u64) << 8) | f as u64)
}

/// `e:GetChainData(ch).<field>`
pub fn effect_data(d: &Duel, e: EffectId, ch: Option<u8>, f: u8) -> Value {
    d.script_var(effect_key(d, e, ch, f))
}
/// `e:GetChainData(ch).<field> = v`
pub fn set_effect_data(d: &mut Duel, e: EffectId, ch: Option<u8>, f: u8, v: Value) {
    let k = effect_key(d, e, ch, f);
    prune_finished_chains(d);
    d.set_script_var(k, v);
}
/// Drops the data of chain links that are no longer on the chain: scripts can only reach the
/// links of the current chain, and chain ids are not reused, so this is not observable (it
/// keeps the copied script state small).
fn prune_finished_chains(d: &mut Duel) {
    let n = duel::get_current_chain(d, false).max(0) as u8;
    let live: Vec<u64> = (1..=n).map(|ch| get_id(d, ch) as u64).collect();
    let dead = |((ns, k), _): &((u32, u64), Value)| *ns == NS_EFFECT_DATA && !live.contains(&(k >> 40));
    if d.script_vars.iter().any(dead) {
        std::sync::Arc::make_mut(&mut d.script_vars).retain(|x| !dead(x));
    }
}

/// The card properties of `chain.lua` (`CARD_PROPERTIES`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Prop {
    Code,
    Controler,
    Location,
    Sequence,
    Position,
    Type,
    Level,
    Rank,
    Scale,
    Link,
    Attribute,
    Race,
    Attack,
    Defense,
    SummonLocation,
    SummonType,
    SetCard,
}
pub const PROPS: [Prop; 17] = [
    Prop::Code,
    Prop::Controler,
    Prop::Location,
    Prop::Sequence,
    Prop::Position,
    Prop::Type,
    Prop::Level,
    Prop::Rank,
    Prop::Scale,
    Prop::Link,
    Prop::Attribute,
    Prop::Race,
    Prop::Attack,
    Prop::Defense,
    Prop::SummonLocation,
    Prop::SummonType,
    Prop::SetCard,
];
impl Prop {
    /// `Code` and `SetCard` have several values (`{fn(c)}` in a properties table).
    fn multi(self) -> bool {
        matches!(self, Prop::Code | Prop::SetCard)
    }
    /// How `Is<prop>` compares (`return_equals`, `return_equals_any`, `return_has_common_bits`).
    fn cmp(self) -> Cmp {
        match self {
            Prop::Controler => Cmp::Equals,
            Prop::Sequence | Prop::Level | Prop::Rank | Prop::Scale | Prop::Link | Prop::Attack | Prop::Defense => Cmp::EqualsAny,
            Prop::Position | Prop::Type | Prop::Attribute | Prop::Race | Prop::SummonLocation | Prop::SummonType | Prop::Location => Cmp::CommonBits,
            Prop::Code => Cmp::Code,
            Prop::SetCard => Cmp::SetCode,
        }
    }
}
#[derive(Clone, Copy)]
enum Cmp {
    Equals,
    EqualsAny,
    CommonBits,
    Code,
    SetCode,
}
/// A properties table: the values by `Prop` (`Code`/`SetCard` as `Value::Multi`).
pub type Props = std::sync::Arc<[Value]>;

fn props_value(p: &Props) -> Value {
    Value::Multi(p.to_vec())
}
/// The values of a property as a function returns them (`table.unpack` for `Code`/`SetCard`).
fn unpack(v: &Value) -> Value {
    v.clone()
}
fn first(v: Value) -> Value {
    match v {
        Value::Multi(m) => m.into_iter().next().unwrap_or(Value::Nil),
        v => v,
    }
}
fn ints(v: &Value) -> Vec<i64> {
    match v {
        Value::Multi(m) => m.iter().map(|x| x.int()).collect(),
        Value::Nil => Vec::new(),
        v => vec![v.int()],
    }
}
fn code_check(codes: &[i64], args: &[i64]) -> bool {
    args.iter().any(|a| codes.first() == Some(a) || codes.get(1) == Some(a))
}
fn setcode_check(to_match: &[i64], setcodes: &[i64]) -> bool {
    to_match.iter().any(|&sa| setcodes.iter().any(|&sb| sa & 0xfff == sb & 0xfff && sa & sb == sb))
}
fn arg(args: &[i64], i: usize) -> i64 {
    *args.get(i).unwrap_or_else(|| crate::lua_error("bad argument (number expected, got nil)"))
}
/// `Is<prop>` of a property getter's result (`nil` stays `nil`).
fn compare(p: Prop, res: &Value, args: &[i64]) -> Value {
    if matches!(res, Value::Nil) && !p.multi() {
        return Value::Nil;
    }
    Value::Bool(match p.cmp() {
        Cmp::Equals => res.int() == arg(args, 0),
        Cmp::EqualsAny => args.contains(&res.int()),
        Cmp::CommonBits => res.int() & arg(args, 0) > 0,
        Cmp::Code => code_check(&ints(res), args),
        Cmp::SetCode => setcode_check(&ints(res), args),
    })
}

/// `get_all_current_properties(c)`
pub fn current_props(d: &mut Duel, c: CardId) -> Props {
    PROPS.iter().map(|&p| current_get(d, c, p)).collect::<Vec<_>>().into()
}
/// `Card.Get<prop>(c)`
fn current_get(d: &mut Duel, c: CardId, p: Prop) -> Value {
    use crate::lua::card as k;
    let i = |v: i64| Value::Int(v);
    match p {
        Prop::Code => Value::Multi(k::get_code(d, c).into_iter().map(|x| i(x as i64)).collect()),
        Prop::Controler => i(k::get_controler(d, c) as i64),
        Prop::Location => i(k::get_location(d, c) as i64),
        Prop::Sequence => i(k::get_sequence(d, c) as i64),
        Prop::Position => i(k::get_position(d, c) as i64),
        Prop::Type => i(k::get_type(d, c) as i64),
        Prop::Level => i(k::get_level(d, c) as i64),
        Prop::Rank => i(k::get_rank(d, c) as i64),
        Prop::Scale => i(k::get_scale(d, c) as i64),
        Prop::Link => i(k::get_link(d, c) as i64),
        Prop::Attribute => i(k::get_attribute(d, c) as i64),
        Prop::Race => i(k::get_race(d, c) as i64),
        Prop::Attack => i(k::get_attack(d, c) as i64),
        Prop::Defense => i(k::get_defense(d, c) as i64),
        Prop::SummonLocation => i(k::get_summon_location(d, c) as i64),
        Prop::SummonType => i(k::get_summon_type(d, c) as i64),
        Prop::SetCard => Value::Multi(k::get_set_card(d, c).into_iter().map(|x| i(x as i64)).collect()),
    }
}
/// `Card.Is<prop>(c, ...)`
fn current_is(d: &mut Duel, c: CardId, p: Prop, args: &[i64]) -> Value {
    use crate::lua::card as k;
    let u32s = || args.iter().map(|&x| x as u32).collect::<Vec<_>>();
    Value::Bool(match p {
        Prop::Code => k::is_code(d, c, &u32s()),
        Prop::Controler => k::is_controler(d, c, arg(args, 0) as u8),
        Prop::Location => k::is_location(d, c, arg(args, 0) as u32),
        Prop::Sequence => k::is_sequence(d, c, &u32s()),
        Prop::Position => k::is_position(d, c, arg(args, 0) as u32),
        Prop::Type => k::is_type(d, c, arg(args, 0) as u32),
        Prop::Level => k::is_level(d, c, &u32s()),
        Prop::Rank => k::is_rank(d, c, &u32s()),
        Prop::Scale => k::is_scale(d, c, &u32s()),
        Prop::Link => k::is_link(d, c, &u32s()),
        Prop::Attribute => k::is_attribute(d, c, arg(args, 0) as u32),
        Prop::Race => k::is_race(d, c, arg(args, 0) as u64),
        Prop::Attack => k::is_attack(d, c, &u32s()),
        Prop::Defense => k::is_defense(d, c, &u32s()),
        Prop::SummonLocation => k::is_summon_location(d, c, arg(args, 0) as u32),
        Prop::SummonType => k::is_summon_type(d, c, &u32s()),
        Prop::SetCard => k::is_set_card(d, c, &args.iter().map(|&x| x as u16).collect::<Vec<_>>()),
    })
}

/// `Chain.GetTriggering<prop>(ch)`
pub fn get_triggering(d: &Duel, ch: u8, p: Prop) -> Value {
    let one = |info: u32| duel::get_chain_info(d, ch, &[info]).into_iter().next().unwrap_or(Value::Nil);
    match p {
        Prop::Code => Value::Multi(duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2])),
        Prop::Controler => one(CHAININFO_TRIGGERING_CONTROLER),
        Prop::Location => one(CHAININFO_TRIGGERING_LOCATION),
        Prop::Sequence => one(CHAININFO_TRIGGERING_SEQUENCE),
        Prop::Position => one(CHAININFO_TRIGGERING_POSITION),
        Prop::Type => one(CHAININFO_TRIGGERING_TYPE),
        Prop::Level => one(CHAININFO_TRIGGERING_LEVEL),
        Prop::Rank => one(CHAININFO_TRIGGERING_RANK),
        // (assumes the left and right scales are the same)
        Prop::Scale => one(CHAININFO_TRIGGERING_LSCALE),
        Prop::Link => one(CHAININFO_TRIGGERING_LINK),
        Prop::Attribute => one(CHAININFO_TRIGGERING_ATTRIBUTE),
        Prop::Race => one(CHAININFO_TRIGGERING_RACE),
        Prop::Attack => one(CHAININFO_TRIGGERING_ATTACK),
        Prop::Defense => one(CHAININFO_TRIGGERING_DEFENSE),
        Prop::SummonLocation => one(CHAININFO_TRIGGERING_SUMMON_LOCATION),
        Prop::SummonType => one(CHAININFO_TRIGGERING_SUMMON_TYPE),
        Prop::SetCard => match one(CHAININFO_TRIGGERING_SETCODES) {
            Value::Nil => Value::Nil,
            v => Value::Multi(ints(&v).into_iter().map(Value::Int).collect()),
        },
    }
}
/// `get_all_triggering_properties(ch)`
pub fn triggering_props(d: &Duel, ch: u8) -> Props {
    PROPS.iter().map(|&p| get_triggering(d, ch, p)).collect::<Vec<_>>().into()
}
/// `Chain.IsTriggering<prop>(ch, ...)`
pub fn is_triggering(d: &Duel, ch: u8, p: Prop, args: &[i64]) -> Value {
    match p {
        Prop::Code => match duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]) {
            v if v.is_empty() || matches!(v[0], Value::Nil) => Value::Nil,
            v => Value::Bool(code_check(&v.iter().map(|x| x.int()).collect::<Vec<_>>(), args)),
        },
        Prop::SetCard => match duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_SETCODES]).into_iter().next() {
            None | Some(Value::Nil) => Value::Nil,
            Some(v) => Value::Bool(setcode_check(&ints(&v), args)),
        },
        // (`aux.OR` of the location and the symbolic location)
        Prop::Location => {
            let sym = duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_LOCATION_SYMBOLIC]).into_iter().next().unwrap_or(Value::Nil);
            Value::Bool(compare(p, &get_triggering(d, ch, p), args).truthy() || compare(p, &sym, args).truthy())
        }
        _ => compare(p, &get_triggering(d, ch, p), args),
    }
}
/// `Chain.IsTriggeringRaceExcept(ch, race)`
pub fn is_triggering_race_except(d: &Duel, ch: u8, race: u64) -> Value {
    is_triggering(d, ch, Prop::Race, &[(RACE_ALL & !race) as i64])
}
pub fn is_triggering_attribute_except(d: &Duel, ch: u8, attr: u32) -> Value {
    is_triggering(d, ch, Prop::Attribute, &[(ATTRIBUTE_ALL & !attr) as i64])
}
/// `Chain.IsTriggeringCompositeType(ch, typ)`
pub fn is_triggering_composite_type(d: &Duel, ch: u8, typ: u32) -> Value {
    match get_triggering(d, ch, Prop::Type) {
        Value::Nil => Value::Nil,
        v => Value::Bool(v.int() as u32 & typ == typ),
    }
}
/// `Chain.IsTriggeringExactType(ch, typ)`
pub fn is_triggering_exact_type(d: &Duel, ch: u8, typ: u32) -> Value {
    compare(Prop::Controler, &get_triggering(d, ch, Prop::Type), &[typ as i64])
}

/// The key of the shared data of a chain link (`Chain.Data(ch)` without a key).
const SHARED: u32 = u32::MAX >> 8;
/// The field of the shared chain data with the resolving card's properties.
const RESOLVING_PROPERTIES: u8 = 0;

/// `Chain.Data(ch).resolving_properties`
fn resolving_props(d: &Duel, ch: u8) -> Option<Props> {
    let (ns, k) = effect_key(d, EffectId(SHARED), Some(ch), RESOLVING_PROPERTIES);
    match d.script_var((ns, k)) {
        Value::Multi(m) => Some(m.into()),
        _ => None,
    }
}
/// `resolving_fns.Get<prop>(ch)`
fn resolving_get(d: &Duel, ch: u8, p: Prop) -> Value {
    match resolving_props(d, ch) {
        Some(r) => unpack(&r[p as usize]),
        None => Value::Nil,
    }
}
/// `resolving_fns.Is<prop>(ch, ...)` (there is none for `Location`: it is
/// `get_all_resolving_properties`)
fn resolving_is(d: &Duel, ch: u8, p: Prop, args: &[i64]) -> Value {
    match (p, resolving_props(d, ch)) {
        (Prop::Location, r) => r.map_or(Value::Nil, |r| props_value(&r)),
        (_, None) => Value::Nil,
        (_, Some(r)) => compare(p, &r[p as usize], args),
    }
}
/// The store of the resolving properties (`register_resolving_props_effect`; `chain.lua` only
/// registers it in `Debug.ReloadFieldBegin`).
pub fn store_resolving_props() -> LuaFn {
    operation(|d, a| {
        let re = a.re.unwrap();
        let rc = crate::lua::effect::get_handler(d, re).unwrap();
        if crate::lua::card::is_relate_to_effect(d, rc, re) {
            let props = current_props(d, rc);
            let k = effect_key(d, EffectId(SHARED), None, RESOLVING_PROPERTIES);
            d.set_script_var(k, props_value(&props));
        }
    })
}

/// `registering_properties[e]`: the properties of the effect that registered `e` (recorded by
/// `Duel.RegisterEffect`).
fn registering_props(d: &Duel, e: EffectId) -> Option<Props> {
    d.effect(e).script_data.clone()
}
/// `Duel.RegisterEffect` of `chain.lua`: records the registering effect's properties.
pub(crate) fn record_registering_props(d: &mut Duel, e: EffectId) {
    if let Some(re) = duel::get_reason_effect(d) {
        if !crate::lua::effect::is_global_effect(d, e) {
            let props = effect_card_props(d, re);
            d.effect_mut(e).script_data = props;
        }
    }
}
/// `registering_fns.Get<prop>(e)`
fn registering_get(d: &Duel, e: EffectId, p: Prop) -> Value {
    match registering_props(d, e) {
        Some(r) => unpack(&r[p as usize]),
        None => crate::lua_error("attempt to index a nil value (local 'reg_props')"),
    }
}
/// `registering_fns.Is<prop>(e, ...)` (none for `Location`: `get_all_registering_properties`)
fn registering_is(d: &Duel, e: EffectId, p: Prop, args: &[i64]) -> Value {
    match (p, registering_props(d, e)) {
        (Prop::Location, r) => r.map_or(Value::Nil, |r| props_value(&r)),
        (Prop::Code | Prop::SetCard, None) => Value::Nil,
        (Prop::Code | Prop::SetCard, Some(r)) => compare(p, &r[p as usize], args),
        (_, _) => compare(p, &first(registering_get(d, e, p)), args),
    }
}

/// What the dynamic chain properties come from (`chain_prop`).
enum Source {
    Registering(EffectId),
    Current(CardId),
    Chain(u8),
}
fn chain_source(d: &mut Duel, ch: u8) -> Option<Source> {
    if duel::get_current_chain(d, false) == 0 {
        return None;
    }
    let Some(eff) = duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_EFFECT]).into_iter().next().and_then(|v| v.as_effect()) else {
        crate::lua_error("attempt to index a nil value (local 'eff')");
    };
    if crate::lua::effect::is_has_property(d, eff, EFFECT_FLAG_FIELD_ONLY, 0) {
        return Some(Source::Registering(eff));
    }
    let ec = crate::lua::effect::get_handler(d, eff).unwrap();
    if crate::lua::card::is_relate_to_effect(d, ec, eff) && crate::lua::card::is_faceup(d, ec) {
        return Some(Source::Current(ec));
    }
    Some(Source::Chain(ch))
}
/// `Chain.Get<prop>(ch)`: the current properties if the card is still there face-up, else the
/// resolving or triggering ones (the registering ones for a Duel effect).
pub fn get(d: &mut Duel, ch: u8, p: Prop) -> Value {
    match chain_source(d, ch) {
        None => Value::Nil,
        Some(Source::Registering(e)) => registering_get(d, e, p),
        Some(Source::Current(c)) => current_get(d, c, p),
        Some(Source::Chain(ch)) => match first(resolving_get(d, ch, p)) {
            Value::Nil => get_triggering(d, ch, p),
            // (only the first value of `Code`/`SetCard`)
            res => res,
        },
    }
}
/// `get_all_chain_properties(ch)`
pub fn all_props(d: &mut Duel, ch: u8) -> Option<Props> {
    match chain_source(d, ch)? {
        Source::Registering(e) => registering_props(d, e),
        Source::Current(c) => Some(current_props(d, c)),
        Source::Chain(ch) => resolving_props(d, ch).or_else(|| Some(triggering_props(d, ch))),
    }
}
/// `Chain.Is<prop>(ch, ...)`
pub fn is(d: &mut Duel, ch: u8, p: Prop, args: &[i64]) -> Value {
    match chain_source(d, ch) {
        None => Value::Nil,
        Some(Source::Registering(e)) => registering_is(d, e, p, args),
        Some(Source::Current(c)) => current_is(d, c, p, args),
        Some(Source::Chain(ch)) => match resolving_is(d, ch, p, args) {
            Value::Nil => is_triggering(d, ch, p, args),
            res => res,
        },
    }
}
pub fn is_race_except(d: &mut Duel, ch: u8, race: u64) -> Value {
    is(d, ch, Prop::Race, &[(RACE_ALL & !race) as i64])
}
pub fn is_attribute_except(d: &mut Duel, ch: u8, attr: u32) -> Value {
    is(d, ch, Prop::Attribute, &[(ATTRIBUTE_ALL & !attr) as i64])
}
/// `Chain.IsCompositeType(ch, typ)`
pub fn is_composite_type(d: &mut Duel, ch: u8, typ: u32) -> Value {
    match first(get(d, ch, Prop::Type)) {
        Value::Nil => Value::Nil,
        v => Value::Bool(v.int() as u32 & typ == typ),
    }
}
/// `Chain.IsExactType(ch, typ)`
pub fn is_exact_type(d: &mut Duel, ch: u8, typ: u32) -> Value {
    compare(Prop::Controler, &first(get(d, ch, Prop::Type)), &[typ as i64])
}

/// What the dynamic effect properties come from (`effect_card_prop`); `None` for a global
/// Duel effect.
fn effect_source(d: &mut Duel, e: EffectId) -> Option<Source> {
    if crate::lua::effect::is_has_property(d, e, EFFECT_FLAG_FIELD_ONLY, 0) {
        if crate::lua::effect::is_global_effect(d, e) {
            return None;
        }
        return Some(Source::Registering(e));
    }
    let owner = crate::lua::effect::get_owner(d, e).unwrap();
    if !crate::lua::effect::is_activated(d, e) {
        return Some(Source::Current(owner));
    }
    let te = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_EFFECT]).into_iter().next().and_then(|v| v.as_effect());
    if te == Some(e) {
        return Some(Source::Chain(0));
    }
    Some(Source::Current(owner))
}
/// `effect_get_all_card_props(e)`
pub fn effect_card_props(d: &mut Duel, e: EffectId) -> Option<Props> {
    match effect_source(d, e)? {
        Source::Registering(e) => registering_props(d, e),
        Source::Current(c) => Some(current_props(d, c)),
        Source::Chain(ch) => all_props(d, ch),
    }
}
/// `e:GetCard<prop>()`: the properties of the effect's card (as it was when it registered the
/// effect, for a Duel effect; while the effect is on the chain, the dynamic chain properties).
pub fn effect_get_card(d: &mut Duel, e: EffectId, p: Prop) -> Value {
    match effect_source(d, e) {
        None => Value::Nil,
        Some(Source::Registering(e)) => registering_get(d, e, p),
        Some(Source::Current(c)) => current_get(d, c, p),
        Some(Source::Chain(ch)) => get(d, ch, p),
    }
}
/// `e:IsCard<prop>(...)`
pub fn effect_is_card(d: &mut Duel, e: EffectId, p: Prop, args: &[i64]) -> Value {
    match effect_source(d, e) {
        // (`(mode=="Is") and false or nil` is nil)
        None => Value::Nil,
        Some(Source::Registering(e)) => registering_is(d, e, p, args),
        Some(Source::Current(c)) => current_is(d, c, p, args),
        Some(Source::Chain(ch)) => is(d, ch, p, args),
    }
}
pub fn effect_is_card_race_except(d: &mut Duel, e: EffectId, race: u64) -> Value {
    effect_is_card(d, e, Prop::Race, &[(RACE_ALL & !race) as i64])
}
pub fn effect_is_card_attribute_except(d: &mut Duel, e: EffectId, attr: u32) -> Value {
    effect_is_card(d, e, Prop::Attribute, &[(ATTRIBUTE_ALL & !attr) as i64])
}
/// `e:IsCardCompositeType(typ)` (errors on `nil`, as the Lua does)
pub fn effect_is_card_composite_type(d: &mut Duel, e: EffectId, typ: u32) -> bool {
    match first(effect_get_card(d, e, Prop::Type)) {
        Value::Nil => crate::lua_error("attempt to perform bitwise operation on a nil value"),
        v => v.int() as u32 & typ == typ,
    }
}
pub fn effect_is_card_exact_type(d: &mut Duel, e: EffectId, typ: u32) -> bool {
    matches!(first(effect_get_card(d, e, Prop::Type)), Value::Int(t) if t == typ as i64)
}

/// The global effects `chain.lua` registers in `Debug.ReloadFieldBegin` (only there: normal duels
/// have neither): `register_data_reset_effect` and `register_resolving_props_effect`.
pub fn register_reload_field_effects(d: &mut Duel) {
    use crate::lua::effect;
    let data_reset = effect::global_effect(d);
    effect::set_type(d, data_reset, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, data_reset, EVENT_CHAIN_END);
    effect::set_count_limit(d, data_reset, 1, 0, 0, 0);
    // (`data_table={}`: the data of finished chains is pruned anyway, see `set_effect_data`)
    effect::set_operation(d, data_reset, Some(operation(|_, _| {})));
    duel::register_effect(d, data_reset, 0);
    let store = effect::global_effect(d);
    effect::set_type(d, store, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, store, EVENT_CHAIN_SOLVING);
    effect::set_operation(d, store, Some(store_resolving_props()));
    duel::register_effect(d, store, 0);
}
