//! Gravekeeper's Supernaturalist
use super::prelude::*;

pub const ID: u32 = 79206925;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NECROVALLEY],
    listed_series: &[SET_GRAVEKEEPERS, SET_NECROVALLEY],
    material_setcode: &[SET_GRAVEKEEPERS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GRAVEKEEPERS),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GRAVEKEEPERS as u16], Some(fc), st, Some(tp)))),
            2,
        )],
    );
    // Increase ATK/DEF
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e1, effect_card_value(matcheck));
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Search 1 "Gravekeeper's" monster or "Necrovalley" card during the End Phase
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(regtg)));
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local lv=c:GetMaterial():GetSum(Card.GetOriginalLevel)
    let mat = card::get_material(d, c);
    let lv = group::get_sum(d, &mat, &|d, x| card::get_original_level(d, x) as i64);
    // if lv==0 then return end
    if lv == 0 {
        return 0;
    }
    // Increase ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value(d, e1, lv * 100);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    0
}
fn indcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_NECROVALLEY)
    duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler() or c:IsLocation(LOCATION_FZONE)
    effect::get_handler(d, e) == Some(c) || card::is_location(d, c, LOCATION_FZONE)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_GRAVEKEEPERS) and c:IsMonster()) or c:IsSetCard(SET_NECROVALLEY)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::is_monster(d, c)) || card::is_set_card(d, c, &[SET_NECROVALLEY as u16]))
        && card::is_able_to_hand(d, c, None)
}
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
