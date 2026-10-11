//! Freezing Chains of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 43582229;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate and Special Summon from GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // Unaffected by activated effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_condition(d, e2, Some(lua_fn(unacon)));
    // aux.TargetBoolFunction(Card.IsSetCard,SET_ICE_BARRIER)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, x| card::is_set_card(d, x, &[SET_ICE_BARRIER as u16]))));
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_value_fn(d, e2, lua_fn(unaval));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_level_below(d, c, 4)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, a.e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.unafilter(c)`
fn unafilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
}
/// `s.unacon(e)`
fn unacon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.unafilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,3,nil)
    let tp = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&unafilter), tp, LOCATION_MZONE, 0, 3, NO_EX)
}
/// `s.unaval(e,te)`
fn unaval(d: &mut Duel, a: Args) -> bool {
    // local tc=te:GetOwner()
    let te = a.e(1);
    let e = a.e(0);
    let tc = effect::get_owner(d, te).nil();
    // return tc:IsSpecialSummoned() and tc:IsSummonLocation(LOCATION_EXTRA) and te:GetOwnerPlayer()~=e:GetHandlerPlayer()
    //     and te:IsMonsterEffect() and te:IsActivated() and te:GetActivateLocation()==LOCATION_MZONE
    if !card::is_special_summoned(d, tc) || !card::is_summon_location(d, tc, LOCATION_EXTRA) {
        return false;
    }
    if effect::get_owner_player(d, te) == effect::get_handler_player(d, e) {
        return false;
    }
    effect::is_monster_effect(d, te) && effect::is_activated(d, te) && effect::get_activate_location(d, te) == LOCATION_MZONE
}
