//! Shiranui Squiresaga
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 52711246;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // You can only Special Summon "Shiranui Squiresaga(s)" once per turn.
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Once per turn: You can banish 1 face-up monster you control or in your GY, then apply these effects...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and c:IsMonster() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    card::is_able_to_remove(d, c, None, None, None)
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,tp,300)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, 300);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
            let h = effect::get_handler(d, a.e).nil();
            // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
            let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
            // local g2=Duel.GetMatchingGroup(nil,tp,LOCATION_SZONE,LOCATION_SZONE,nil)
            let g2 = duel::get_matching_group(d, None, a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
            // local g3=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
            let g3 = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
            // --● Zombie: You can have all monsters you control gain 300 ATK
            // if tc:IsRace(RACE_ZOMBIE) and #g1>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            if card::is_race(d, tc, RACE_ZOMBIE) && !g1.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
                // for t1 in g1:Iter() do
                for t1 in group::iter(d, &g1) {
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let e1 = effect::create_effect(d, h);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_UPDATE_ATTACK)
                    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // e1:SetValue(300)
                    effect::set_value(d, e1, 300);
                    // t1:RegisterEffect(e1)
                    card::register_effect(d, t1, e1, false);
                }
            }
            // --● FIRE: You can destroy 1 Spell/Trap on the field
            // if tc:IsAttribute(ATTRIBUTE_FIRE) and #g2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if card::is_attribute(d, tc, ATTRIBUTE_FIRE) && !g2.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
                // local t2=g2:Select(tp,1,1,nil)
                let t2 = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(t2)
                duel::hint_selection(d, t2.clone(), true);
                // Duel.Destroy(t2,REASON_EFFECT)
                duel::destroy(d, t2, REASON_EFFECT, None, None);
            }
            // --● Synchro: You can destroy 1 monster on the field
            // if tc:IsType(TYPE_SYNCHRO) and #g3>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            if card::is_type(d, tc, TYPE_SYNCHRO) && !g3.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
                // local t3=g3:Select(tp,1,1,nil)
                let t3 = group::select(d, &g3, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(t3)
                duel::hint_selection(d, t3.clone(), true);
                // Duel.Destroy(t3,REASON_EFFECT)
                duel::destroy(d, t3, REASON_EFFECT, None, None);
            }
        }
    }
}
