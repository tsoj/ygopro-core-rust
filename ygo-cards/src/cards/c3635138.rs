//! Emissaries from Darkness
use super::prelude::*;

pub const ID: u32 = 3635138;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[3635138], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you take battle or effect damage: You can Special Summon this card from your hand, and if you do, gain 2000 LP, then you can apply 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RECOVER+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_RECOVER + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the End Phase, if this card was Special Summoned this turn: You can target 1 Trap in your GY; Set it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and r&(REASON_BATTLE|REASON_EFFECT)>0 end`
fn e1_condition(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp && a.r & (REASON_BATTLE | REASON_EFFECT) > 0
}
/// `function(e,tp,...) return e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_status(d, h, STATUS_SPSUMMON_TURN)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,2000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 2000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.Recover(tp,2000,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::recover(d, tp, 2000, REASON_EFFECT, false, None) > 0
    {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&aux::necro_valley_filter(|d, cc| spfilter(d, cc, e, tp))),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
        // local b2=Duel.IsExistingMatchingCard(Card.IsAttackPos,tp,0,LOCATION_MZONE,1,nil)
        let b2 = duel::is_existing_matching_card(d, Some(&|d, cc| card::is_attack_pos(d, cc)), tp, 0, LOCATION_MZONE, 1, NO_EX);
        // if not ((b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then return end
        if !((b1 || b2) && duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
            return;
        }
        // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,3)}, {b2,aux.Stringid(id,4)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
        // if op==1 then
        if op == Some(1) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&aux::necro_valley_filter(|d, cc| spfilter(d, cc, e, tp))),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        } else if op == Some(2) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAttackPos,tp,0,LOCATION_MZONE,1,2,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_attack_pos(d, cc)), tp, 0, LOCATION_MZONE, 1, 2, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.settg(e,tp,...,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsTrap() and chkc:IsSSetable() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_trap(d, chkc) && card::is_s_setable(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsTrap,Card.IsSSetable),tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&settable_trap), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsTrap,Card.IsSSetable),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&settable_trap), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SET,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SET, g, 1, tp, 0);
    true
}
/// `aux.AND(Card.IsTrap,Card.IsSSetable)`
fn settable_trap(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.setop(e,tp,...)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SSet(tp,tc) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::s_set(d, a.tp, CardSet::from_card(tc), None, true);
    }
}
