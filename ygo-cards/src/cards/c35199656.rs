//! Trickstar Lycoris
use super::prelude::*;

pub const ID: u32 = 35199656;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(lycost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict 200 damage per card added to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e2, EVENT_TO_HAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.chainsolvedop)
    effect::set_operation(d, e3, Some(operation(chainsolvedop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lycost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and c:IsFaceup() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c.nil(), a.e, 0, tp, false, false, None, None, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, c.map(CardSet::from_card), 1, tp, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsControler,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX)
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // local ct=eg:FilterCount(Card.IsControler,nil,1-tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| card::is_controler(d, c, 1 - a.tp), NO_EX);
    // if Duel.IsChainSolving() then
    if duel::is_chain_solving(d) {
        // for i=1,ct do e:GetHandler():RegisterFlagEffect(id+1,RESET_CHAIN,0,1) end
        let h = effect::get_handler(d, a.e).nil();
        for _ in 0..ct {
            card::register_flag_effect(d, h, ID + 1, RESET_CHAIN, 0, 1, 0, 0);
        }
    } else {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.Damage(1-tp,ct*200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, (ct * 200) as i64, REASON_EFFECT, false, None);
    }
}
// function s.chainsolvedop(e,tp,eg,ep,ev,re,r,rp)
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetHandler():GetFlagEffect(id+1)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_flag_effect(d, h, ID + 1);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.Damage(1-tp,ct*200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, (ct * 200) as i64, REASON_EFFECT, false, None);
        // e:GetHandler():ResetFlagEffect(id+1)
        card::reset_flag_effect(d, h, ID + 1);
    }
}
