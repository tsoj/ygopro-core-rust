//! World Legacy - "World Shield"
use super::prelude::*;

pub const ID: u32 = 55787576;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target/indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // spsummon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    // local tc=te:GetOwner()
    let tc = effect::get_owner(d, te).nil();
    // return tc:IsSpecialSummoned() and tc:IsSummonLocation(LOCATION_EXTRA) and te:IsMonsterEffect()
    //   and te:IsActivated() and te:GetActivateLocation()==LOCATION_MZONE
    card::is_special_summoned(d, tc)
        && card::is_summon_location(d, tc, LOCATION_EXTRA)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && effect::get_activate_location(d, te) == LOCATION_MZONE
}
/// `s.tgtg(e,c)`
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_WORLD_LEGACY) and (e:GetHandler():GetColumnGroup():IsContains(c) or e:GetHandler()==c)
    let h = effect::get_handler(d, e).nil();
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16])) {
        return false;
    }
    let col = card::get_column_group(d, h, 0, 0);
    group::is_contains(&col, c) || h == c
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,1-tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, 1 - tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
    {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),1-tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), 1 - tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
        // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        //   and #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
        if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && !g.is_empty()
            && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
