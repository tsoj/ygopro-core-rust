//! Riryoku
use super::prelude::*;

pub const ID: u32 = 34016756;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Halve the ATK of a monster and increase the ATK of another by the same amount
    // e1:SetDescription(aux.Stringid(id,0))
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,2,2,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g), n, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    let mut g = group::filter(d, &tg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local hc=g:GetFirst()
    let mut hc = group::get_first(d, &g);
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // hc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        hc = group::get_first(d, &sel);
    }
    let hc = hc.nil();
    // g:RemoveCard(hc)
    group::remove_card(&mut g, &CardSet::from_card(hc));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=hc:GetAttack()
    let atk = card::get_attack(d, hc);
    // Halve the ATK of one monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(atk/2)
    effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
    // hc:RegisterEffect(e1)
    card::register_effect(d, hc, e1, false);
    // if not hc:IsImmuneToEffect(e1) and #g>0 then
    if !card::is_immune_to_effect(d, hc, e1) && !g.is_empty() {
        // Add that lost ATK to the other monster
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetValue(atk/2)
        effect::set_value(d, e2, lua_int(atk as f64 / 2.0));
        // g:GetFirst():RegisterEffect(e2)
        let other = group::get_first(d, &g).nil();
        card::register_effect(d, other, e2, false);
    }
}
