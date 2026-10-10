//! Tri-Brigade Arms Bucephalus II
use super::prelude::*;

pub const ID: u32 = 10019086;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRI_BRIGADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure: 3+ Beast, Beast-Warrior, and/or Winged Beast monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACES_BEAST_BWARRIOR_WINGB),3,5,nil,nil,s.splimit)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACES_BEAST_BWARRIOR_WINGB as u64, sc, st, Some(tp)))),
        3,
        Some(5),
        None,
        None,
        Some(lua_fn(splimit)),
    );
    // Special Summon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Opponent cannot activate cards or effects on Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.sucop)
    effect::set_operation(d, e1, Some(operation(sucop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 Beast, Beast-Warrior, or Winged Beast monster to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcostfilter(c)
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsSpellTrap()
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_spell_trap(d, c)
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or Duel.IsExistingMatchingCard(s.spcostfilter,sp,LOCATION_GRAVE,0,3,nil)
    let c = effect::get_handler(d, a.e(0)).nil();
    let sp = a.p(2);
    !card::is_location(d, c, LOCATION_EXTRA) || duel::is_existing_matching_card(d, Some(&spcostfilter), sp, LOCATION_GRAVE, 0, 3, NO_EX)
}
// function s.sucop(e,tp,eg,ep,ev,re,r,rp)
fn sucop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(Card.IsSummonPlayer,1,nil,tp) then
    let tp = a.tp;
    if group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(function(_,rp,tp) return rp==tp end)
        duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, x: Args| x.p(1) == x.p(2)));
    }
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,PLAYER_ALL,LOCATION_ONFIELD)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsAbleToRemove() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_remove(d, c, None, None, None) {
        // Duel.Remove(c,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACES_BEAST_BWARRIOR_WINGB) and c:IsAbleToGrave()
    card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64) && card::is_able_to_grave(d, c, None)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
