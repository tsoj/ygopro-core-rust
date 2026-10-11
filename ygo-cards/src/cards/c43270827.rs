//! Therion Cross
//! Scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 43270827;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ARGYRO_SYSTEM], listed_series: &[SET_THERION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(condition(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect()
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_THERION),tp,LOCATION_MZONE,0,1,nil)
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_THERION as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local b1=Duel.IsChainDisablable(ev) and not rc:IsDisabled()
    let b1 = duel::is_chain_disablable(d, a.ev as u8) && !card::is_disabled(d, rc);
    // local b2=rc:IsAbleToRemove() and rc:IsRelateToEffect(re) and not rc:IsLocation(LOCATION_REMOVED)
    let b2 = card::is_able_to_remove(d, rc, None, None, None) && card::is_relate_to_effect(d, rc, a.re.nil()) && !card::is_location(d, rc, LOCATION_REMOVED);
    // local b3=b1 and b2 and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_ARGYRO_SYSTEM)
    let b3 = b1 && b2 && duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &[CARD_ARGYRO_SYSTEM])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)}, {b3,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1)), (b3, aux::stringid(ID, 2))]);
    let op = op.map_or(0, |v| v as i64);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    } else if op == 2 {
        // Duel.SetTargetCard(rc)
        duel::set_target_card(d, rc);
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, e, CATEGORY_REMOVE);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,1-tp,rc:GetLocation())
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, 1 - tp, card::get_location(d, rc) as i32);
    } else if op == 3 {
        // Duel.SetTargetCard(rc)
        duel::set_target_card(d, rc);
        // e:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
        effect::set_category(d, e, CATEGORY_DISABLE + CATEGORY_REMOVE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,1-tp,rc:GetLocation())
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, 1 - tp, card::get_location(d, rc) as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Negate that effect
    if op == 1 {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    // Banish that monster
    } else if op == 2 {
        // if rc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, rc, a.e) {
            // Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    // Activate both, in sequence
    } else if op == 3 {
        // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(e) then
        if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
