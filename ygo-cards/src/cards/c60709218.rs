//! Bond Between Teacher and Student
use super::prelude::*;

pub const ID: u32 = 60709218;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN_GIRL, CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_DARK_MAGICIAN),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_DARK_MAGICIAN]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_DARK_MAGICIAN_GIRL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_DARK_MAGICIAN_GIRL]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(2314238,75190122,49702428,70168345) and c:IsSSetable()
    card::is_code(d, c, &[2314238, 75190122, 49702428, 70168345]) && card::is_s_setable(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let nv = aux::necro_valley_filter(|d, c| filter(d, c, e, tp));
    let sel = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
            let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
            // if #g>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty()
                && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
                // local tc=g:Select(tp,1,1,nil)
                let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                // Duel.SSet(tp,tc)
                duel::s_set(d, tp, tc, None, true);
            }
        }
    }
}
