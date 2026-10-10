//! Ra'ten, the Heavenly General
use super::prelude::*;

pub const ID: u32 = 30163008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,99,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(99), Some(group_check(lcheck)), None, None);
    // Special Summon 1 Level 4 or lower monster with the same Type as a targeted monster from your hand to your zone this card points to
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 card your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE_START);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,sumtype,tp)
    group::check_same_property(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp))).0
}
/// `s.spfilter1(c,e,tp,lg,zone)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lg: &CardSet, zone: Option<u32>) -> bool {
    // return c:IsFaceup() and lg:IsContains(c)
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND,0,1,nil,e,tp,c:GetRace(),zone)
    if !(card::is_faceup(d, c) && lg.contains(c)) {
        return false;
    }
    let race = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp, race, zone)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
/// `s.spfilter2(c,e,tp,rc,zone)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: u64, zone: Option<u32>) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(rc) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, rc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), zone)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let zone = card::get_linked_zone(d, h, Some(a.tp));
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.spfilter1(chkc,e,tp,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && spfilter1(d, chkc, a.e, a.tp, &lg, None);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and Duel.IsExistingTarget(s.spfilter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp,lg,zone) end
    if a.chk == Some(0) {
        let (ft, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone));
        return ft > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter1(d, c, a.e, a.tp, &lg, Some(zone))), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.spfilter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp,lg,zone)
    duel::select_target(d, a.tp, Some(&|d, c| spfilter1(d, c, a.e, a.tp, &lg, Some(zone))), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetFirstTarget()
    let tg = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tg:IsRelateToEffect(e) or tg:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tg = tg.nil();
    if !card::is_relate_to_effect(d, tg, a.e) || card::is_facedown(d, tg) {
        return;
    }
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let zone = card::get_linked_zone(d, c, Some(a.tp));
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)<=0 then return end
    let (ft, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone));
    if ft <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_HAND,0,1,1,nil,e,tp,tg:GetRace(),zone):GetFirst()
    let race = card::get_race(d, tg);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter2(d, x, a.e, a.tp, race, Some(zone))), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
