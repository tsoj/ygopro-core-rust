//! Trap Holic
use super::prelude::*;

pub const ID: u32 = 22377092;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Spell/Trap you control, and if you do, Set 1 Normal Trap from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c,tp,ft)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32) -> bool {
    // if not c:IsSpellTrap() then return false end
    if !card::is_spell_trap(d, c) {
        return false;
    }
    // if c:IsLocation(LOCATION_STZONE) then ft=ft+1 end
    let ft = if card::is_location(d, c, LOCATION_STZONE) { ft + 1 } else { ft };
    // return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,ft>0)
    let haszone = ft > 0;
    duel::is_existing_matching_card(d, Some(&|d, x| setfilter(d, x, haszone)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.setfilter(c,haszone)`
fn setfilter(d: &mut Duel, c: CardId, haszone: bool) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable(haszone) and not c:IsCode(id)
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, haszone, None) && !card::is_code(d, c, &[ID])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if not c:IsLocation(LOCATION_SZONE) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then ft=ft-1 end
    if !card::is_location(d, c, LOCATION_SZONE) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        ft -= 1;
    }
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc,tp,ft) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc, a.tp, ft) && chkc != c;
    }
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,c,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, tp, ft)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,c,tp,ft)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, tp, ft)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,false):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| setfilter(d, x, false)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SSet(tp,sc)>0 then
        if let Some(sc) = sc {
            if duel::s_set(d, tp, CardSet::from_card(sc), None, true) > 0 {
                // It can be activated this turn while you have 3 or more Traps in your GY
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(aux.Stringid(id,1))
                effect::set_description(d, e1, aux::stringid(ID, 1));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
                effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
                // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                // e1:SetCondition(function(e) return Duel.GetMatchingGroupCount(Card.IsTrap,tp,LOCATION_GRAVE,0,nil)>=3 end)
                effect::set_condition(
                    d,
                    e1,
                    Some(condition(move |d, _a| duel::get_matching_group_count(d, Some(&card::is_trap), tp, LOCATION_GRAVE, 0, NO_EX) >= 3)),
                );
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    }
}
