//! Rokket Recharger
use super::prelude::*;

pub const ID: u32 = 5969957;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // cannot be battle target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_condition(d, e2, Some(condition(imcon)));
    effect::set_value_fn(d, e2, aux::imval1());
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:GetPreviousAttributeOnField()&ATTRIBUTE_DARK == ATTRIBUTE_DARK and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsSummonLocation(LOCATION_EXTRA) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_controler(d, c, tp)
        && card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_DARK == ATTRIBUTE_DARK
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_summon_location(d, c, LOCATION_EXTRA)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.cfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    let g = group::filter(d, &eg, &|d, c| cfilter(d, c, tp), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, a.e, Value::Group(g));
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.spfilter(c,oc,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, oc: CardId, e: EffectId, tp: u8) -> bool {
    // return not c:IsOriginalCodeRule(oc:GetOriginalCodeRule()) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let ocodes = card::get_original_code_rule(d, oc);
    !card::is_original_code_rule(d, c, &ocodes)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tgfilter(c,eg,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, eg: &CardSet, e: EffectId, tp: u8) -> bool {
    // return eg:IsContains(c) and s.cfilter(c,tp) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,c,e,tp)
    eg.contains(c) && cfilter(d, c, tp) && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.tgfilter(chkc,g,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && tgfilter(d, chkc, &g, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,1,nil,g,e,tp) end
    if a.chk == Some(0) {
        let (ft, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        return ft > 0
            && duel::is_existing_target(
                d,
                Some(&|d, c| tgfilter(d, c, &g, a.e, a.tp)),
                a.tp,
                LOCATION_GRAVE | LOCATION_REMOVED,
                LOCATION_GRAVE | LOCATION_REMOVED,
                1,
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tg=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,1,1,nil,g,e,tp)
    duel::select_target(
        d,
        a.tp,
        Some(&|d, c| tgfilter(d, c, &g, a.e, a.tp)),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        LOCATION_GRAVE | LOCATION_REMOVED,
        1,
        1,
        false,
        NO_EX,
    );
    // g:DeleteGroup()
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let (ft, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
            if ft > 0 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,tc,e,tp)
                let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, tc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
                // if #g>0 then
                if !g.is_empty() {
                    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
}
/// `s.imfilter(c)`
fn imfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.imcon(e)`
fn imcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.imfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&imfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
