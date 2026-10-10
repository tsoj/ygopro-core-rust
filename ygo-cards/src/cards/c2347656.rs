//! Lovely Labrynth of the Silver Castle
use super::prelude::*;

pub const ID: u32 = 2347656;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Prevent activations in response to your Normal Traps
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.chainop)
    effect::set_operation(d, e1, Some(operation(chainop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 Trap from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.fgtg)
    effect::set_target(d, e2, Some(cost_target(fgtg)));
    // e2:SetOperation(s.fgop)
    effect::set_operation(d, e2, Some(operation(fgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card in your opponent's hand or field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.chainop(e,tp,eg,ep,ev,re,r,rp)
fn chainop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rp==tp and rc:IsNormalTrap() and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if a.rp == a.tp && card::is_normal_trap(d, rc) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
// function s.chainlm(e,rp,tp)
fn chainlm(d: &mut Duel, a: Args) -> bool {
    // return rp==tp or not e:IsMonsterEffect()
    a.p(1) == a.p(2) || !effect::is_monster_effect(d, a.e(0))
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable()
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.fgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn fgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
// function s.fgop(e,tp,eg,ep,ev,re,r,rp)
fn fgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SSet(tp,tc)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::s_set(d, a.tp, CardSet::from_card(tc), None, true) > 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetCondition(s.triggercon)
        effect::set_condition(d, e1, Some(lua_fn(triggercon)));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.triggercon(e)
fn triggercon(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND),tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_EFFECT)
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re and re:IsTrapEffect() and re:GetHandler():GetOriginalType()==TYPE_TRAP
    //     and eg:IsExists(s.cfilter,1,nil)
    if a.rp != a.tp {
        return false;
    }
    let Some(re) = a.re else { return false };
    if !effect::is_trap_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::get_original_type(d, rc) == TYPE_TRAP && group::is_exists(d, &a.eg.clone().nil(), &cfilter, 1, NO_EX)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_ONFIELD|LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, (LOCATION_ONFIELD | LOCATION_HAND) as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g1 = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local g2=Duel.GetMatchingGroup(nil,tp,0,LOCATION_HAND,nil)
    let g2 = duel::get_matching_group(d, None, a.tp, 0, LOCATION_HAND, NO_EX);
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0
    let b2 = !g2.is_empty();
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // local sg=Group.CreateGroup()
    let sg: CardSet = if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // sg=g1:Select(tp,1,1,nil)
        let sg = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        sg
    } else if op == Some(2) {
        // sg=g2:RandomSelect(tp,1)
        group::random_select(d, &g2, a.tp, 1)
    } else {
        CardSet::new()
    };
    // if #sg>0 then Duel.Destroy(sg,REASON_EFFECT) end
    if !sg.is_empty() {
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
