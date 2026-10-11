//! Zexal Alliance
use super::prelude::*;

pub const ID: u32 = 31712840;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Utopia" monster from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`: pay LP until you have 10 left
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLP(tp)>10 end
    if a.chk == Some(0) {
        return duel::get_lp(d, a.tp) > 10;
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)-10)
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, (lp - 10) as u32);
    true
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp))
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsPreviousPosition(POS_FACEUP)
    card::is_type(d, c, TYPE_XYZ)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != tp))
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: if a face-up Xyz monster was destroyed by battle or opponent's card effect
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_UTOPIA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_set_card(d, c, &[SET_UTOPIA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, None, tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, None, tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: Special Summon 1 "Utopia" monster from GY, place 1 card from deck on top of deck
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local sg=g:GetFirst()
        let sg = group::get_first(d, &g);
        // if sg then
        if let Some(sg) = sg {
            // Cannot be destroyed by battle, except with a "Number" monster
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sg:RegisterEffect(e1)
            card::register_effect(d, sg, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
            // e2:SetValue(s.indval)
            effect::set_value_fn(d, e2, effect_card_fn(indval));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // sg:RegisterEffect(e2)
            card::register_effect(d, sg, e2, false);
            // Double its ATK
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e3, EFFECT_SET_ATTACK);
            // e3:SetValue(sg:GetTextAttack()*2)
            let v = card::get_text_attack(d, sg) * 2;
            effect::set_value(d, e3, v as i64);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // sg:RegisterEffect(e3)
            card::register_effect(d, sg, e3, false);
        }
        // Choose a card to place on top of deck
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, None, tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveSequence(sc,0)
            duel::move_sequence(d, sc, 0, None);
            // Duel.ConfirmDecktop(tp,1)
            duel::confirm_decktop(d, tp, 1);
        }
    }
}
/// `s.indval(e,c)`
fn indval(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_NUMBER)
    !card::is_set_card(d, c, &[SET_NUMBER as u16])
}
