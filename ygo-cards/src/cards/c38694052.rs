//! Voloferniges, the Darkest Dragon Doomrider
use super::prelude::*;

pub const ID: u32 = 38694052;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // Destroy 1 face-up card on the field, and if you destroyed a monster, you can make 1 face-up monster you control gain ATK equal to the destroyed monster's original Level/Rank x 300 until the end of the next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetCondition(function(e) return not e:GetHandler():GetOverlayGroup():IsExists(Card.IsRace,1,nil,RACE_DRAGON) end)
    effect::set_condition(d, e1, Some(condition(e1con)));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(2),s.descost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(2), None, None), cost_target(descost)])));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This is a Quick Effect if this card has a Dragon monster as material
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsRace,1,nil,RACE_DRAGON) end)
    effect::set_condition(d, e2, Some(condition(e2con)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return not e:GetHandler():GetOverlayGroup():IsExists(Card.IsRace,1,nil,RACE_DRAGON) end`
fn e1con(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    !group::is_exists(d, &og, &|d, x| card::is_race(d, x, RACE_DRAGON), 1, NO_EX)
}
/// `function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsRace,1,nil,RACE_DRAGON) end`
fn e2con(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_race(d, x, RACE_DRAGON), 1, NO_EX)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // This card cannot attack the turn you activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and tc:IsPreviousLocation(LOCATION_MZONE)
    //     and (tc:HasLevel() or tc:HasRank()) and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    let tc = tc.nil();
    if card::is_relate_to_effect(d, tc, e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_previous_location(d, tc, LOCATION_MZONE)
        && (card::has_level(d, tc) || card::has_rank(d, tc))
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local sc=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // local lvrk=math.max(tc:GetOriginalLevel(),tc:GetOriginalRank())
        let lvrk = card::get_original_level(d, tc).max(card::get_original_rank(d, tc)) as i64;
        // It gains ATK equal to the destroyed monster's original Level/Rank x 300 until the end of the next turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(lvrk*300)
        effect::set_value(d, e1, lvrk * 300);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
