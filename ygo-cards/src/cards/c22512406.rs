//! Masked Ninja Ebisu
use super::prelude::*;

pub const ID: u32 = 22512406;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 10236520], listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return Spells/Traps your opponent controls to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.confilter(c)
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NINJA) and c:IsFaceup() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_NINJA as u16]) && card::is_faceup(d, c) && !card::is_code(d, c, &[ID])
}
// aux.FaceupFilter(Card.IsSetCard,SET_NINJA)
fn faceup_ninja(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NINJA as u16])
}
// aux.AND(Card.IsSpellTrap,Card.IsAbleToHand)
fn spelltrap_to_hand(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.confilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&confilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_NINJA),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&faceup_ninja), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,ct,nil) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::is_existing_matching_card(d, Some(&spelltrap_to_hand), a.tp, 0, LOCATION_ONFIELD, ct as u32, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,ct,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, ct as u8, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Each of your "Goe Goe the Gallant Ninja" can attack directly this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsCode,10236520))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[10236520]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_NINJA),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&faceup_ninja), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&spelltrap_to_hand), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if ct>#g then return end
    if ct > g.len() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local sg=g:Select(tp,ct,ct,nil)
    let sg = group::select(d, &g, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg,true)
    duel::hint_selection(d, sg.clone(), true);
    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
    duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
}
