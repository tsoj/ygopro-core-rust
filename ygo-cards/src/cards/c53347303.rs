//! Blue-Eyes Shining Dragon
use super::prelude::*;

pub const ID: u32 = 53347303;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[23995346], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, effect_card_value(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // negate
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e4, CATEGORY_DISABLE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Double Snare
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(3682106)
    effect::set_code(d, e5, 3682106);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsCode,1,false,1,true,c,c:GetControler(),nil,false,nil,23995346)
    let tp = card::get_controler(d, c);
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| card::is_code(d, x, &[23995346])),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsCode,1,1,false,true,true,c,nil,nil,false,nil,23995346)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| card::is_code(d, x, &[23995346])),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: Some(c), ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // g:DeleteGroup()
    }
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsRace,c:GetControler(),LOCATION_GRAVE,0,nil,RACE_DRAGON)*300
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_race(d, x, RACE_DRAGON)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local loc,tg=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TARGET_CARDS]);
    let loc = info[0].int() as u32;
    let tg = info.get(1).and_then(|v| v.as_group().cloned());
    // if not tg or not tg:IsContains(c) then return false end
    match tg {
        Some(tg) if group::is_contains(&tg, c) => {}
        _ => return false,
    }
    // return Duel.IsChainDisablable(ev) and loc~=LOCATION_DECK
    duel::is_chain_disablable(d, a.ev as u8) && loc != LOCATION_DECK
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
