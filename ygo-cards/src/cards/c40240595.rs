//! Cocoon of Evolution
use super::prelude::*;

pub const ID: u32 = 40240595;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[58192742, ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip this card to 1 "Petit Moth" you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // e2
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_condition(d, e2, Some(condition(checkcon)));
    effect::set_operation(d, e2, Some(operation(checkop)));
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    card::register_effect(d, c, e2, false);
    // e3
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_ADJUST);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_condition(d, e3, Some(condition(checkcon2)));
    effect::set_operation(d, e3, Some(operation(checkop2)));
    card::register_effect(d, c, e3, false);
}
/// `s.checkcon(e,tp,...)`
fn checkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_card_target(d, h);
    // return Duel.IsTurnPlayer(tp) and g and g:IsExists(Card.IsCode,1,nil,58192742)
    duel::is_turn_player(d, a.tp) && group::is_exists(d, &g, &|d, c| card::is_code(d, c, &[58192742]), 1, NO_EX)
}
/// `s.checkop(e,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()+1
    let ct = card::get_turn_counter(d, c) + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
}
/// `s.checkcon2(e,...)`
fn checkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_card_target(d, h);
    // local g2=e:GetLabelObject() or Group.CreateGroup()
    let g2 = effect::get_label_object(d, a.e).as_group().cloned().unwrap_or_else(CardSet::new);
    // return #g>0 and not g:Equal(g2)
    !g.is_empty() && !group::equal(&g, &g2)
}
/// `s.checkop2(e,...)`
fn checkop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetCardTarget()
    let g = card::get_card_target(d, c);
    // local g2=e:GetLabelObject()
    let g2 = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g:IsExists(Card.IsCode,1,g:Filter(aux.TRUE,g2),58192742) then
    let filtered = match &g2 {
        Some(g2) => group::filter(d, &g, &|_, _| true, Exclude::Group(g2)),
        None => group::filter(d, &g, &|_, _| true, NO_EX),
    };
    let exists = group::is_exists(d, &g, &|d, cc| card::is_code(d, cc, &[58192742]), 1, Exclude::Group(&filtered));
    if !exists {
        // c:SetTurnCounter(0)
        card::set_turn_counter(d, c, 0);
    }
    // g:KeepAlive()
    // e:SetLabelObject(g)
    effect::set_label_object(d, a.e, Value::Group(g));
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(58192742)
    card::is_faceup(d, c) && card::is_code(d, c, &[58192742])
}
/// `s.eqtg(e,...,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsControler(1-tp) or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    let bad = match tc {
        None => true,
        Some(tc) => {
            duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
                || card::is_controler(d, tc, 1 - a.tp)
                || card::is_facedown(d, tc)
                || !card::is_relate_to_effect(d, tc, a.e)
        }
    };
    if bad {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    let tc = tc.nil();
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, a.tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    effect::set_label_object(d, e1, Value::Card(tc));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    // equip effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    effect::set_value(d, e2, 0);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_SET_BASE_DEFENSE);
    effect::set_value(d, e3, 2000);
    card::register_effect(d, c, e3, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
