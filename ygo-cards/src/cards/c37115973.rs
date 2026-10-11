//! Numeral Hunter
use super::prelude::*;

pub const ID: u32 = 37115973;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // todeck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // splimit
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetTarget(s.splimit)
    effect::set_target(d, e4, Some(effect_card_fn(splimit)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // indes
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_BATTLE);
    // e5:SetValue(s.indval)
    effect::set_value_fn(d, e5, effect_card_fn(indval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // immune
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e6, EFFECT_IMMUNE_EFFECT);
    // e6:SetValue(s.efilter)
    effect::set_value_fn(d, e6, lua_fn(efilter));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ) and c:IsAbleToExtra()
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_able_to_extra(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if not c:IsSetCard(SET_NUMBER) then return false end
    if !card::is_set_card(d, c, &[SET_NUMBER as u16]) {
        return false;
    }
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return c:IsType(TYPE_XYZ)
        card::is_type(d, c, TYPE_XYZ)
    } else {
        // return c:IsOriginalType(TYPE_XYZ)
        card::is_original_type(d, c, TYPE_XYZ)
    }
}
/// `s.indval(e,c)`
fn indval(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ)
    card::is_type(d, c, TYPE_XYZ)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsActiveType(TYPE_XYZ)
    effect::is_active_type(d, a.e(1), TYPE_XYZ)
}
