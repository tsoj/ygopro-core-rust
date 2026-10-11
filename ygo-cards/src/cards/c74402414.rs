//! Spell Chronicle
use super::prelude::*;

pub const ID: u32 = 74402414;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x25], ..ScriptInfo::DEFAULT };

/// The group that `e0` holds as its label object. The other effects of this script point to `e0`
/// (their label object is `Value::Effect(e0)`), so the group has one place in the duel.
fn shared_group(d: &Duel, e: EffectId) -> Group {
    match effect::get_label_object(d, e) {
        Value::Group(g) => g,
        Value::Effect(e0) => match effect::get_label_object(d, e0) {
            Value::Group(g) => g,
            _ => CardSet::new(),
        },
        _ => CardSet::new(),
    }
}
/// `e:GetLabelObject():X(...)` where the group changes: store it back in `e0`
fn store_shared_group(d: &mut Duel, e: EffectId, g: Group) {
    let e0 = match effect::get_label_object(d, e) {
        Value::Effect(e0) => e0,
        _ => e,
    };
    effect::set_label_object(d, e0, Value::Group(g));
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x25)
    card::enable_counter_permit(d, c, 0x25, None, None);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCost(s.cost)
    effect::set_cost(d, e0, Some(cost_target(cost_fn)));
    // e0:SetTarget(s.target)
    effect::set_target(d, e0, Some(cost_target(target)));
    // e0:SetOperation(s.activate)
    effect::set_operation(d, e0, Some(operation(activate)));
    // e0:SetLabelObject(g)
    effect::set_label_object(d, e0, Value::Group(CardSet::new()));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add counters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetOperation(aux.chainreg)
    effect::set_operation(d, e1, Some(aux::chainreg()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add cards back to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCost(s.thcost)
    effect::set_cost(d, e3, Some(cost_target(thcost)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Effect(e0));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Register before leaving
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetOperation(s.checkop)
    effect::set_operation(d, e4, Some(operation(checkop)));
    // e4:SetLabelObject(g)
    effect::set_label_object(d, e4, Value::Effect(e0));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Inflict damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetOperation(s.damop)
    effect::set_operation(d, e5, Some(operation(damop)));
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0):Filter(aux.TRUE,e:GetHandler())
    let fg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let h = effect::get_handler(d, a.e).nil();
    let hg = group::filter(d, &fg, &|_d, _c| true, h.into());
    // if chk==0 then return #hg>0 and hg:FilterCount(Card.IsAbleToGraveAsCost,nil)==#hg end
    if a.chk == Some(0) {
        let n = group::get_count(&hg);
        return n > 0 && group::filter_count(d, &hg, &|d, c| card::is_able_to_grave_as_cost(d, c), NO_EX) == n;
    }
    // Duel.SendtoGrave(hg,REASON_COST)
    duel::send_to_grave(d, hg, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToRemove()
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,5,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 5, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,5,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 5, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<5 then return end
    if group::get_count(&g) < 5 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=g:Select(tp,5,5,nil)
    let rg = group::select(d, &g, tp, 5, 5, false, NO_EX).nil();
    // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    // for tc in aux.Next(rg) do
    for tc in group::iter(d, &rg) {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
    // e:GetLabelObject():Clear()
    // e:GetLabelObject():Merge(rg)
    store_shared_group(d, a.e, rg);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if rp==1-tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    if a.rp == 1 - a.tp {
        let re = a.re.nil();
        let h = effect::get_handler(d, a.e).nil();
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():AddCounter(0x25,1)
            card::add_counter(d, h, 0x25, 1, false);
        }
    }
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x25,2,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, a.tp, 0x25, 2, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,0x25,2,REASON_COST)
    card::remove_counter(d, h, a.tp, 0x25, 2, REASON_COST);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsAbleToHand()
    card::get_flag_effect(d, c, ID) != 0 && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return e:GetLabelObject():IsContains(chkc) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&shared_group(d, a.e), chkc) && thfilter(d, chkc);
    }
    // if chk==0 then return e:GetLabelObject():IsExists(s.thfilter,1,nil) end
    if a.chk == Some(0) {
        let sg = shared_group(d, a.e);
        return group::is_exists(d, &sg, &thfilter, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
    // local g=e:GetLabelObject():FilterSelect(1-tp,s.thfilter,1,1,nil)
    let mut sg = shared_group(d, a.e);
    let g = group::filter_select(d, &sg, 1 - tp, &thfilter, 1, 1, false, NO_EX).nil();
    // e:GetLabelObject():Sub(g)
    group::remove_card(&mut sg, &g);
    store_shared_group(d, a.e, sg);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - a.tp, tc);
        }
    }
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabelObject():FilterCount(s.dfilter,nil)
    let sg = shared_group(d, a.e);
    let ct = group::filter_count(d, &sg, &dfilter, NO_EX) as i64;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsDisabled() or not c:IsStatus(STATUS_EFFECT_ENABLED) or ct==0 then
    if card::is_disabled(d, c) || !card::is_status(d, c, STATUS_EFFECT_ENABLED) || ct == 0 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    } else {
        // e:SetLabel(ct)
        effect::set_label(d, a.e, &[ct]);
    }
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabelObject():GetLabel()
    let ct = match effect::get_label_object(d, a.e) {
        Value::Effect(e4) => effect::get_label(d, e4),
        _ => crate::lua_error("attempt to index a nil value"),
    };
    // if ct>0 and c:GetPreviousControler()==tp then
    if ct > 0 && card::get_previous_controler(d, c) == a.tp {
        // Duel.Damage(tp,ct*500,REASON_EFFECT)
        duel::damage(d, a.tp, ct * 500, REASON_EFFECT, false, None);
    }
}
