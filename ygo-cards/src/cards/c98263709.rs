//! Galaxy Mirror Sage
use super::prelude::*;

pub const ID: u32 = 98263709;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain 500 LP per "Galaxy" monster in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.rectg)
    effect::set_target(d, e1, Some(cost_target(rectg)));
    // e1:SetOperation(s.recop)
    effect::set_operation(d, e1, Some(operation(recop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register the fact it was flipped face-up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_FLIP)
    effect::set_code(d, e2, EVENT_FLIP);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetOperation(s.flipop)
    effect::set_operation(d, e2, Some(operation(flipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon 1 level 4 or lower "Galaxy" monster from deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`: `return c:IsSetCard(SET_GALAXY) and c:IsMonster()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GALAXY as u16]) && card::is_monster(d, c)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local val=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*500
    let val = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500;
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(val)
    duel::set_target_param(d, val as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,val)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, val as i32);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, a: &EffArgs) {
    // local val=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*500
    let val = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Recover(p,val,REASON_EFFECT)
    duel::recover(d, p, val, REASON_EFFECT, false, None);
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,(RESET_EVENT|RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE),0,0)
    let h = effect::get_handler(d, a.e).nil();
    let reset = (RESET_EVENT | RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE);
    card::register_flag_effect(d, h, ID, reset, 0, 0, 0, 0);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0 and e:GetHandler():IsReason(REASON_DESTROY)
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0 && card::is_reason(d, h, REASON_DESTROY)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GALAXY) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_set_card(d, c, &[SET_GALAXY as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE) then
    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) {
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
}
