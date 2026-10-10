//! Shift
use super::prelude::*;

pub const ID: u32 = 59560625;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change battle target
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e1, Some(condition(condition1)));
    effect::set_target(d, e1, Some(cost_target(target1)));
    effect::set_operation(d, e1, Some(operation(activate1)));
    card::register_effect(d, c, e1, false);
    // Change effect target
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_condition(d, e2, Some(condition(condition2)));
    effect::set_target(d, e2, Some(cost_target(target2)));
    effect::set_operation(d, e2, Some(operation(activate2)));
    card::register_effect(d, c, e2, false);
}
/// `s.condition1(e,tp,...)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,d=Duel.GetAttacker(),Duel.GetAttackTarget()
    let att = duel::get_attacker(d);
    let dt = duel::get_attack_target(d);
    // return a and d and a:IsControler(1-tp) and d:IsControler(tp)
    match (att, dt) {
        (Some(att), Some(dt)) => card::is_controler(d, att, 1 - a.tp) && card::is_controler(d, dt, a.tp),
        _ => false,
    }
}
/// `s.target1(e,tp,...,chk,chkc)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ag=Duel.GetAttacker():GetAttackableTarget()
    let att = duel::get_attacker(d).nil();
    let (ag, _) = card::get_attackable_target(d, att);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    let ex = at.map_or(NO_EX, |x| x.into());
    // if chk==0 then return ag:IsExists(Card.IsCanBeEffectTarget,1,at,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &ag, &|d, c| card::is_can_be_effect_target(d, c, Some(a.e)), 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tg=ag:FilterSelect(tp,Card.IsCanBeEffectTarget,1,1,at,e)
    let tg = group::filter_select(d, &ag, a.tp, &|d, c| card::is_can_be_effect_target(d, c, Some(a.e)), 1, 1, false, ex).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    true
}
/// `s.activate1(e,tp,...)`
fn activate1(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        let att = duel::get_attacker(d).nil();
        if !card::is_immune_to_effect(d, att, a.e) {
            // Duel.ChangeAttackTarget(tc)
            duel::change_attack_target(d, Some(tc), false);
        }
    }
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = match info.into_iter().next() {
        Some(Value::Group(g)) => Some(g),
        _ => None,
    };
    // if not g or #g~=1 then return false end
    let Some(g) = g else {
        return false;
    };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and tc:IsLocation(LOCATION_MZONE)
    card::is_controler(d, tc, a.tp) && card::is_location(d, tc, LOCATION_MZONE)
}
/// `s.filter(c,ct)`
fn filter(d: &mut Duel, c: CardId, ct: u32) -> bool {
    // return Duel.CheckChainTarget(ct,c)
    duel::check_chain_target(d, ct as u8, c)
}
/// `s.target2(e,tp,...,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if chkc then return chkc~=tc and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,ev) end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) != tc && card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.ev as u32);
    }
    let ex = tc.map_or(NO_EX, |x| x.into());
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,tc,ev) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.ev as u32)), a.tp, LOCATION_MZONE, 0, 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,tc,ev)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.ev as u32)), a.tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    true
}
/// `s.activate2(e,tp,...)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.ChangeTargetCard(ev,Group.FromCards(tc))
        duel::change_target_card(d, a.ev as u8, &CardSet::from_card(tc));
    }
}
