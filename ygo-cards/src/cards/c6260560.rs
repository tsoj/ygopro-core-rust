//! 炎魔刃レーヴァテイン
//! Laevateinn the Burning Blade
use super::prelude::*;

pub const ID: u32 = 6260560;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure: 2 FIRE monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_FIRE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, sc, st, Some(tp)))), 2, None, None, None, None);

    // Special Summon 1 FIRE monster from your GY with the same original Type but a lower original Level than the Tributed FIRE monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);

    // If a face-down card(s) in your Spell & Trap Zone would be destroyed by an opponent's card effect, you can banish this card you control instead
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let tp = effect::get_handler_player(d, e);
        repfilter(d, c, tp)
    }));
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    card::register_effect(d, c, e2, false);
}
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsAttribute(ATTRIBUTE_FIRE) and c:HasLevel() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetOriginalRace(),c:GetOriginalLevel())
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::has_level(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let race = card::get_original_race(d, c);
            let lv = card::get_original_level(d, c);
            duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, race, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64, lv: u32) -> bool {
    // c:IsAttribute(ATTRIBUTE_FIRE) and c:HasLevel() and c:IsOriginalRace(race) and c:GetOriginalLevel()<lv
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::has_level(d, c)
        && card::is_original_race(d, c, race)
        && card::get_original_level(d, c) < lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,c,e,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&|d, x| spcostfilter(d, x, a.e, a.tp)), 1, 1, false, None, c.into());
    }
    // local sc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,c,e,tp):GetFirst()
    let sg = aux::select_release_group_cost(d, a.tp, Some(&|d, x| spcostfilter(d, x, a.e, a.tp)), 1, 1, false, None, c.into());
    let sc = group::get_first(d, &sg).nil();
    // e:SetLabel(sc:GetOriginalRace(),sc:GetOriginalLevel())
    let race = card::get_original_race(d, sc);
    let lv = card::get_original_level(d, sc);
    effect::set_label(d, a.e, &[race as i64, lv as i64]);
    // Duel.Release(sc,REASON_COST)
    duel::release(d, CardSet::from_card(sc), REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local race,lv=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels.first().copied().unwrap_or(0) as u64;
    let lv = labels.get(1).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,race,lv)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, race, lv)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsFacedown() and c:IsLocation(LOCATION_STZONE) and c:IsControler(tp) and c:IsReasonPlayer(1-tp)
    //     and c:IsReason(REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_facedown(d, c)
        && card::is_location(d, c, LOCATION_STZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason_player(d, c, 1 - tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsStatus(STATUS_DESTROY_CONFIRMED) and c:IsAbleToRemove()
    //     and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg();
        return !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
