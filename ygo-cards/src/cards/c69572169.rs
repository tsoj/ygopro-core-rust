//! E・HERO フラッシュ (Elemental HERO Flash)
use super::prelude::*;

pub const ID: u32 = 69572169;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&rfilter), tp, LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0) and rg:GetClassCount(Card.GetCode)>=3 end
    if a.chk == Some(0) {
        let eg = Value::Group(a.eg());
        let re = match a.re {
            Some(re) => Value::Effect(re),
            None => Value::Nil,
        };
        let bfg = aux::bfgcost()
            .call(
                d,
                &[
                    Value::Effect(a.e),
                    Value::Int(tp as i64),
                    eg,
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    re,
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                    Value::Int(0),
                ],
            )
            .truthy();
        return bfg && group::get_class_count(d, &rg, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // local sg=aux.SelectUnselectGroup(rg,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_REMOVE)
    let mut sg = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(3), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(sg+e:GetHandler(),POS_FACEUP,REASON_COST)
    group::add_card(&mut sg, &CardSet::from_card(h));
    duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalSpell() and c:IsAbleToHand()
    card::is_normal_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
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
