//! Stairway to a Fabled Realm
//! 魔轟神界の階
//! Scripted by the Razgriz
use super::prelude::*;

pub const ID: u32 = 22555834;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
    // Add Fabled from GY to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // ATK Increase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(lua_fn(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(effect_card_fn(atktg)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, lua_fn(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_FABLED) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_FABLED as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_FABLED) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_FABLED as u16]) && card::is_able_to_hand(d, c, None)
}
/// `Card.IsDiscardable` as a filter (no arguments: reason REASON_COST)
fn is_discardable_cost(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&is_discardable_cost), a.tp, LOCATION_HAND, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.DiscardHand(tp,Card.IsDiscardable,2,2,REASON_EFFECT|REASON_DISCARD,nil)~=0 then
    if duel::discard_hand(d, a.tp, Some(&is_discardable_cost), 2, 2, REASON_EFFECT | REASON_DISCARD, NO_EX) != 0 {
        // if tc and tc:IsRelateToEffect(e) then
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.SendtoHand(tc,tp,REASON_EFFECT)
                duel::send_to_hand(d, tc, Some(a.tp), REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - a.tp, tc);
            }
        }
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: Args) -> bool {
    // s[0]=false
    set_s_var(d, ID, 0, Value::Bool(false));
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c==Duel.GetAttacker() and c:IsSetCard(SET_FABLED)
    duel::get_attacker(d) == Some(c) && card::is_set_card(d, c, &[SET_FABLED as u16])
}
/// `s.atkval(e)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // local ch=Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_HAND,0)
    let p = effect::get_handler_player(d, e);
    let ch = duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64;
    // local oh=Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_HAND)
    let oh = duel::get_field_group_count(d, p, 0, LOCATION_HAND) as i64;
    // if s[0] or (d and ch<oh) then
    if s_var(d, ID, 0).truthy() || (at.is_some() && ch < oh) {
        // s[0]=true
        set_s_var(d, ID, 0, Value::Bool(true));
        // return (oh-ch)*200
        return (oh - ch) * 200;
    }
    // else return 0 end
    0
}
