//! Debris Dragon
use super::prelude::*;

pub const ID: u32 = 14943837;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro custom
    let e4 = effect::create_effect(d, c);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_SYNCHRO_MAT_RESTRICTION);
    effect::set_value_fn(d, e4, lua_fn(synclimitfilter));
    card::register_effect(d, c, e4, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value_fn(d, e2, lua_fn(synlimit));
    card::register_effect(d, c, e2, false);
    // summon success
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e3, Some(cost_target(sumtg)));
    effect::set_operation(d, e3, Some(operation(sumop)));
    card::register_effect(d, c, e3, false);
}
/// `s.synlimit(e,c)`
fn synlimit(d: &mut Duel, a: Args<'_>) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else { return false };
    // return not c:IsRace(RACE_DRAGON)
    !card::is_race(d, c, RACE_DRAGON)
}
/// `s.synclimitfilter(e,c)`
fn synclimitfilter(d: &mut Duel, a: Args<'_>) -> bool {
    // return c:GetLevel()~=4
    card::get_level(d, a.c(1)) != 4
}
/// `s.filter2(c,e,sp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsAttackBelow(500) and c:IsCanBeSpecialSummoned(e,0,sp,false,false)
    card::is_attack_below(d, c, 500) && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, None, None, None)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter2(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter2(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
    if card::is_relate_to_effect(d, tc.nil(), a.e) && duel::special_summon_step(d, tc.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc.nil(), e1, true);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc.nil(), e2, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
