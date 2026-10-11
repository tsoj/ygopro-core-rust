//! Cyber Energy Shock
use super::prelude::*;

pub const ID: u32 = 38265153;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The condition of `e1` (`function(e,tp)`).
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_JINZO]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local istrap=tc:IsTrap()
    let istrap = card::is_trap(d, tc);
    // if not (tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and istrap) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 && istrap) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // local b1=Duel.IsExistingMatchingCard(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,exc)
    let b1 = duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_negatable(d, c)),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        exc.map_or(NO_EX, |x| x.into()),
    );
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_JINZO]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    );
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // if not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,2)},
    //   {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if op==1 then
    if op == Some(1) {
        // Negate the effects of 1 face-up card on the field until the end of this turn
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,exc):GetFirst()
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, c| card::is_negatable(d, c)),
            a.tp,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            1,
            false,
            exc.map_or(NO_EX, |x| x.into()),
        )
        .nil();
        let tc = group::get_first(d, &g);
        // if not tc then return end
        let Some(tc) = tc else { return };
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // tc:NegateEffects(c,RESET_PHASE|PHASE_END,true)
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), true, None);
    } else if op == Some(2) {
        // All "Jinzo" you control gain 800 ATK
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_JINZO]))),
            a.tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // Gains 800 ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(800)
            effect::set_value(d, e1, 800);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
