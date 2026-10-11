//! Xyz Encore
use super::prelude::*;

pub const ID: u32 = 78610936;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsAbleToExtra() and c:GetOverlayCount()>0
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_able_to_extra(d, c) && card::get_overlay_count(d, c) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(aux.FALSE)
        duel::set_chain_limit(d, aux_false());
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local mg=tc:GetOverlayGroup()
    let mg = card::get_overlay_group(d, tc);
    // Duel.SendtoGrave(mg,REASON_EFFECT)
    duel::send_to_grave(d, &mg, REASON_EFFECT, None, None);
    // Duel.AdjustInstantly(tc)
    duel::adjust_instantly(d, Some(tc));
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0 {
        // local g=mg:Filter(aux.NecroValleyFilter(s.spfilter),nil,e,tp)
        let mut g = group::filter(d, &mg, &aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp)), NO_EX);
        // local ft=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ft>0 and #g>0 then
        if ft > 0 && !g.is_empty() {
            // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
            if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
                ft = 1;
            }
            // if #g>ft then
            if g.len() as i32 > ft {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // g=g:Select(tp,ft,ft,nil)
                g = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
            }
            // local tc=g:GetFirst()
            let _ = group::get_first(d, &g);
            // for tc in aux.Next(g) do
            for tc in group::iter(d, &g) {
                // Duel.SpecialSummonStep(tc,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
                duel::special_summon_step(d, tc, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
                // if tc:HasLevel() and tc:IsLevelAbove(2) then
                if card::has_level(d, tc) && card::is_level_above(d, tc, 2) {
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_UPDATE_LEVEL)
                    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
                    // e1:SetValue(-1)
                    effect::set_value(d, e1, -1);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                }
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
