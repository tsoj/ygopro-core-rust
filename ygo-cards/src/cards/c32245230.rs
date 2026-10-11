//! The Deal of Destiny
//! 運命の契約
use super::prelude::*;

pub const ID: u32 = 32245230;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[27062594],
    listed_series: &[SET_UTOPIC],
    counter_place_list: &[0x207],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x207)
    card::enable_counter_permit(d, c, 0x207, None, None);
    // c:SetCounterLimit(0x207,1)
    card::set_counter_limit(d, c, 0x207, 1);
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(counter)));
    card::register_effect(d, c, e2, false);
    // xyz summon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_controler(d, c, tp) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn counter(d: &mut Duel, a: &EffArgs) {
    // local ct=eg:FilterCount(s.cfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| cfilter(d, c, a.tp), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // e:GetHandler():AddCounter(0x207,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0x207, 1, false);
    }
}
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_player(d, c, tp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,1-tp) and rp==1-tp
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, 1 - a.tp), 1, NO_EX) && a.rp == 1 - a.tp
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanRemoveCounter(tp,0x207,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, 0x207, 1, REASON_COST);
    }
    // c:RemoveCounter(tp,0x207,1,REASON_EFFECT)
    card::remove_counter(d, c, a.tp, 0x207, 1, REASON_EFFECT);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, gc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_UTOPIC) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsType(TYPE_XYZ)
    //     and Duel.GetLocationCountFromEx(tp,tp,gc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_set_card(d, c, &[SET_UTOPIC as u16])
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_type(d, c, TYPE_XYZ)
        && duel::get_location_count_from_ex(d, tp, Some(tp), match gc { Some(g) => g.into(), None => NO_EX }, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
fn gyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(27062594) and (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToGrave()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_code(d, c, &[27062594])
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_grave(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| gyfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local gc=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| gyfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &sel);
    // if gc and Duel.SendtoGrave(gc,REASON_EFFECT)>0 and gc:IsLocation(LOCATION_GRAVE) then
    if let Some(gc) = gc {
        if duel::send_to_grave(d, gc, REASON_EFFECT, None, None) > 0 && card::is_location(d, gc, LOCATION_GRAVE) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local xc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,gc):GetFirst()
            let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, Some(gc))), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            if let Some(xc) = group::get_first(d, &sel2) {
                // if xc and Duel.SpecialSummon(xc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
                if duel::special_summon(d, xc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // xc:CompleteProcedure()
                    card::complete_procedure(d, xc);
                    // Duel.Overlay(xc,c)
                    duel::overlay(d, xc, c, false);
                }
            }
        }
    }
}
