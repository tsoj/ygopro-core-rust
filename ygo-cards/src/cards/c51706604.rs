//! Blizzard
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 51706604;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsNegatableSpellTrap()
    card::is_spell(d, c) && card::is_negatable_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_ONFIELD) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // continous negation
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_CHAIN_SOLVING)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
            // e1:SetCondition(s.discon)
            effect::set_condition(d, e1, Some(condition(discon)));
            // e1:SetOperation(s.disop)
            effect::set_operation(d, e1, Some(operation(disop)));
            // e1:SetLabel(tc:GetOriginalCodeRule())
            let codes: Vec<i64> = card::get_original_code_rule(d, tc).iter().map(|&x| x as i64).collect();
            effect::set_label(d, e1, &codes);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
            // redirect
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
            effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE|RESET_PHASE|PHASE_END)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE | RESET_PHASE | PHASE_END, 1);
            // e2:SetCondition(s.recon)
            effect::set_condition(d, e2, Some(condition(recon)));
            // e2:SetValue(LOCATION_HAND)
            effect::set_value(d, e2, LOCATION_HAND as i64);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
        }
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    let loc = loc[0].int();
    // local code1,code2=re:GetHandler():GetOriginalCodeRule()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    let codes = card::get_original_code_rule(d, rc);
    let code1 = codes.first().copied();
    let code2 = codes.get(1).copied();
    // return re:IsSpellEffect() and loc&LOCATION_ONFIELD~=0 and (code1==code or code2==code)
    effect::is_spell_effect(d, a.re.nil())
        && loc & LOCATION_ONFIELD as i64 != 0
        && (code1 == Some(code as u32) || code2 == Some(code as u32))
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
fn recon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:GetDestination()==LOCATION_GRAVE
    card::is_faceup(d, c) && card::get_destination(d, c) == LOCATION_GRAVE
}
