//! Transcode Talker
use super::prelude::*;

pub const ID: u32 = 46947713;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[46947713], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // atk gain
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    // untarget
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsRace(RACE_CYBERSE)
    card::is_race(d, a.c(0), RACE_CYBERSE)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetMutualLinkedGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_mutual_linked_group_count(d, h) > 0
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local g=e:GetHandler():GetMutualLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let g = card::get_mutual_linked_group(d, h);
    // return c==e:GetHandler() or g:IsContains(c)
    c == h || group::is_contains(&g, c)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    let h = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    let h = effect::get_handler(d, a.e).nil();
    aux::add_temp_lizard_check(d, h, tp, Some(lua_fn(lizfilter)), None, None, None, None);
    true
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsOriginalRace(RACE_CYBERSE)
    !card::is_original_race(d, a.c(1), RACE_CYBERSE)
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsRace(RACE_CYBERSE)
    !card::is_race(d, a.c(1), RACE_CYBERSE)
}
/// `s.filter(c,e,tp,zone)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsLinkBelow(3) and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_race(d, c, RACE_CYBERSE)
        && card::is_link_below(d, c, 3)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp)) & ZONES_MMZ;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp,zone) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp, zone);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,zone)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc = Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp)) & ZONES_MMZ;
    // if tc and tc:IsRelateToEffect(e) and zone~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && zone != 0 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    }
}
