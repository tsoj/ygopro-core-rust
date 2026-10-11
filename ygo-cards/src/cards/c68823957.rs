//! Subterror Nemesis Defender
use super::prelude::*;

pub const ID: u32 = 68823957;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.cbtg)
    effect::set_target(d, e1, Some(cost_target(cbtg)));
    // e1:SetOperation(s.cbop)
    effect::set_operation(d, e1, Some(operation(cbop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_MZONE);
    // e2:SetCondition(s.cecon)
    effect::set_condition(d, e2, Some(condition(cecon)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.cetg)
    effect::set_target(d, e2, Some(cost_target(cetg)));
    // e2:SetOperation(s.ceop)
    effect::set_operation(d, e2, Some(operation(ceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetCondition(s.indcon)
    effect::set_condition(d, e3, Some(condition(indcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cbfilter(c,e)`
fn cbfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e)
    card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.cbfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && cbfilter(d, chkc, a.e);
    }
    // local ag=Duel.GetAttacker():GetAttackableTarget()
    let attacker = duel::get_attacker(d).nil();
    let mut ag = card::get_attackable_target(d, attacker).0;
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // ag:RemoveCard(at)
    group::remove_card(&mut ag, &CardSet::from_card(at));
    // if chk==0 then return Duel.GetAttacker():IsControler(1-tp) and at:IsControler(tp) and at:IsFacedown()
    //     and ag:IsExists(s.cbfilter,1,e:GetHandler(),e) end
    if a.chk == Some(0) {
        let atk = duel::get_attacker(d).nil();
        return card::is_controler(d, atk, 1 - a.tp)
            && card::is_controler(d, at, a.tp)
            && card::is_facedown(d, at)
            && group::is_exists(d, &ag, &|d, cc| cbfilter(d, cc, a.e), 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=ag:FilterSelect(tp,s.cbfilter,1,1,e:GetHandler(),e)
    let g = group::filter_select(d, &ag, a.tp, &|d, cc| cbfilter(d, cc, a.e), 1, 1, false, h.into()).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let atk = duel::get_attacker(d).nil();
            if !card::is_immune_to_effect(d, atk, a.e) {
                // Duel.ChangeAttackTarget(tc)
                duel::change_attack_target(d, Some(tc), false);
            }
        }
    }
}
/// `s.cecon(e,tp,eg,ep,ev,re,r,rp)`
fn cecon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e==re or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.re == Some(a.e) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not g or #g~=1 then return false end
    let Some(g) = g else { return false };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsLocation(LOCATION_MZONE) and tc:IsFacedown()
    card::is_location(d, tc, LOCATION_MZONE) && card::is_facedown(d, tc)
}
/// `s.cefilter(c,ct,oc)`
fn cefilter(d: &mut Duel, c: CardId, ct: u32, oc: CardId) -> bool {
    // return oc~=c and Duel.CheckChainTarget(ct,c)
    oc != c && duel::check_chain_target(d, ct as u8, c)
}
/// `s.cetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cetg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.cefilter(chkc,ev,e:GetHandler()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && cefilter(d, chkc, a.ev, h);
    }
    // e:GetLabelObject()
    let label = effect::get_label_object(d, a.e).as_card();
    let ex = label.map_or(NO_EX, |x| x.into());
    // if chk==0 then return Duel.IsExistingTarget(s.cefilter,tp,LOCATION_MZONE,0,1,e:GetLabelObject(),ev,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| cefilter(d, cc, a.ev, h)), a.tp, LOCATION_MZONE, 0, 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.cefilter,tp,LOCATION_MZONE,0,1,1,e:GetLabelObject(),ev,e:GetHandler())
    duel::select_target(d, a.tp, Some(&|d, cc| cefilter(d, cc, a.ev, h)), a.tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    true
}
/// `s.ceop(e,tp,eg,ep,ev,re,r,rp)`
fn ceop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangeTargetCard(ev,Group.FromCards(tc))
            duel::change_target_card(d, a.ev as u8, &CardSet::from_card(tc));
        }
    }
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFacedown,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
