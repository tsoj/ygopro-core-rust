//! Holactie the Creator of Light
use super::prelude::*;

pub const ID: u32 = 10000040;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SLIFER, CARD_OBELISK, CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by Tributing 3 monsters ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card's Special Summon cannot be negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // The player that Special Summons this card wins the Duel
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetOperation(function(e) Duel.Win(e:GetHandler():GetSummonPlayer(),WIN_REASON_CREATORGOD) end)
    effect::set_operation(d, e3, Some(lua_fn(win_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:GetClassCount(Card.GetOriginalCodeRule)==3
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::get_class_count(d, sg, &|d, c| card::get_original_code_rule(d, c)[0] as i64) == 3;
    (res, false)
}
/// The tribute group: monsters whose original names are Slifer, Obelisk and Ra
fn tribute_group(d: &mut Duel, tp: u8) -> CardSet {
    // Duel.GetReleaseGroup(tp):Match(Card.IsOriginalCodeRule,nil,CARD_SLIFER,CARD_OBELISK,CARD_RA)
    let mut g = duel::get_release_group(d, tp, false, false, None);
    group::match_(d, &mut g, &|d, c| card::is_original_code_rule(d, c, &[CARD_SLIFER, CARD_OBELISK, CARD_RA]), NO_EX);
    g
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=...
    let g = tribute_group(d, tp);
    // return #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
    if g.len() < 3 {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.selfsptg(e,tp,...,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=...
    let g = tribute_group(d, tp);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #sg==3 then
    if sg.len() == 3 {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,...,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if g then Duel.Release(g,REASON_COST) end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        duel::release(d, g, REASON_COST, None);
    }
}
/// `function(e) Duel.Win(e:GetHandler():GetSummonPlayer(),WIN_REASON_CREATORGOD) end`
fn win_op(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    let p = card::get_summon_player(d, h);
    duel::win(d, p, WIN_REASON_CREATORGOD);
}
