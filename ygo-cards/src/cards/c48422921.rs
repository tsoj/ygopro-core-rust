//! Super Rush Headlong
use super::prelude::*;

pub const ID: u32 = 48422921;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP() or Duel.IsBattlePhase()
    duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local val=ATTRIBUTE_ALL
    let mut val = ATTRIBUTE_ALL;
    // local reg=g:GetFirst():GetFlagEffectLabel(id)
    let first = group::get_first(d, &g).nil();
    let reg = card::get_flag_effect_label(d, first, ID).first().copied();
    // if reg then val=val&(~reg) end
    if let Some(reg) = reg {
        val &= !(reg as u32);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // local att=Duel.AnnounceAttribute(tp,1,val)
    let att = duel::announce_attribute(d, tp, 1, val);
    // Duel.SetTargetParam(att)
    duel::set_target_param(d, att);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
            // local att=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
            let att = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_BATTLE_START)
            effect::set_code(d, e1, EVENT_BATTLE_START);
            // e1:SetLabel(att)
            effect::set_label(d, e1, &[att]);
            // e1:SetOwnerPlayer(tp)
            effect::set_owner_player(d, e1, tp);
            // e1:SetCondition(s.descon)
            effect::set_condition(d, e1, Some(condition(descon)));
            // e1:SetOperation(s.desop)
            effect::set_operation(d, e1, Some(operation(desop)));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
            // local reg=tc:GetFlagEffectLabel(id)
            let reg = card::get_flag_effect_label(d, tc, ID).first().copied();
            if let Some(reg) = reg {
                // reg=(reg|att)
                let reg = reg | att;
                // tc:SetFlagEffectLabel(id,reg)
                card::set_flag_effect_label(d, tc, ID, reg as u32);
            } else {
                // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,att)
                card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, att, 0);
            }
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // return tp==e:GetOwnerPlayer() and tc and tc:IsControler(1-tp) and tc:IsAttribute(e:GetLabel())
    if a.tp != effect::get_owner_player(d, a.e) {
        return false;
    }
    match tc {
        Some(tc) => card::is_controler(d, tc, 1 - a.tp) && card::is_attribute(d, tc, effect::get_label(d, a.e) as u32),
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc.map(CardSet::from_card).unwrap_or_default(), REASON_EFFECT, None, None);
}
