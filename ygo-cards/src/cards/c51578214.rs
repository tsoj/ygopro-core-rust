//! Beetrooper Assault Roller
use super::prelude::*;

pub const ID: u32 = 51578214;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_BEETROOPER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search 1 "Beetroper" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.adtg)
    effect::set_target(d, e3, Some(cost_target(adtg)));
    // e3:SetOperation(s.adop)
    effect::set_operation(d, e3, Some(operation(adop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spfilter(c,tp)
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //     and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_INSECT)
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,1,1,nil,0)
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,1,nil,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // rg:KeepAlive()
        // e:SetLabelObject(rg)
        effect::set_label_object(d, e, Value::Group(rg));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local rg=e:GetLabelObject()
    let rg = effect::get_label_object(d, e).as_group().cloned();
    // if not rg then return end
    let Some(rg) = rg else {
        return;
    };
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // rg:DeleteGroup()
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_INSECT),c:GetControler(),LOCATION_MZONE,0,c)*200
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_INSECT))), tp, LOCATION_MZONE, 0, c.into()) as i64 * 200
}
// function s.adfilter(c)
fn adfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BEETROOPER) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BEETROOPER as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.adtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn adtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.adfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&adfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.adop(e,tp,eg,ep,ev,re,r,rp)
fn adop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.adfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&adfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
