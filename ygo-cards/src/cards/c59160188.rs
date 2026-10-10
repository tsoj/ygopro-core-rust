//! Lair of Darkness
use super::prelude::*;

pub const ID: u32 = 59160188;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[59160189], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
    // e2:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e2, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // tribute substitute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_RELEASE_NONSUM)
    effect::set_code(d, e3, EFFECT_EXTRA_RELEASE_NONSUM);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_DARK))));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetValue(s.relval)
    effect::set_value_fn(d, e3, lua_fn(relval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // token
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_RELEASE)
    effect::set_code(d, e6, EVENT_RELEASE);
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetOperation(s.regop)
    effect::set_operation(d, e6, Some(operation(regop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EVENT_PHASE_START|PHASE_DRAW)
    effect::set_code(d, e7, EVENT_PHASE_START | PHASE_DRAW);
    // e7:SetOperation(s.clearop)
    effect::set_operation(d, e7, Some(operation(clearop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=e6:Clone()
    let e8 = effect::clone(d, e6);
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e8:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e8, EVENT_LEAVE_FIELD_P);
    // e8:SetOperation(s.clearop)
    effect::set_operation(d, e8, Some(operation(clearop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.relval(e,re,r,rp)`: `return re and re:IsActivated() and (r&REASON_COST)~=0`
fn relval(d: &mut Duel, a: Args) -> bool {
    match a.v(1).as_effect() {
        Some(re) => effect::is_activated(d, re) && (a.int(2) as u32 & REASON_COST) != 0,
        None => false,
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local ec=eg:FilterCount(Card.IsMonster,nil)
    let ec = group::filter_count(d, &a.eg.clone().nil(), &card::is_monster, NO_EX);
    // if ec>0 then
    if ec > 0 {
        // local val=e:GetLabelObject():GetLabel()
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        let val = effect::get_label(d, lo);
        // e:GetLabelObject():SetLabel(val+ec)
        effect::set_label(d, lo, &[val + ec as i64]);
    }
}
/// `s.clearop(e,tp,eg,ep,ev,re,r,rp)`
fn clearop(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(0)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[0]);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
/// The token data of `s.spop` (`TYPES_TOKEN`, 1000/1000, level 3, Fiend, DARK).
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(3),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // if ct==0 or not e:GetHandler():IsRelateToEffect(e)
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,1000,1000,3,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP_DEFENSE,p) then return end
    // (`ct` is a global here, never set by this script: `ct==0` is false in Lua.)
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e)
        || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(p), 0)
    {
        return;
    }
    // local ct=math.min(Duel.GetLocationCount(p,LOCATION_MZONE),e:GetLabel())
    let loc = duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let mut ct = loc.min(effect::get_label(d, a.e));
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=math.min(1,ct) end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1.min(ct);
    }
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // for i=1,ct do
    for _ in 1..=ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,p,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, a.tp, p, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
