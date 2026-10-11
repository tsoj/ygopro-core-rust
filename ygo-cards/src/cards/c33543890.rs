//! Drytron Beta Rastaban
use super::prelude::*;

pub const ID: u32 = 33543890;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRYTRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Normal Summoned/Set
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.spconlimit)
    effect::set_value_fn(d, e1, lua_fn(spconlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Drytron.TributeCost)
    effect::set_cost(d, e2, Some(aux::drytron::tribute_cost()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.sumfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(sumfilter));
}
// function s.spconlimit(e,se,sp,st)
fn spconlimit(d: &mut Duel, a: Args) -> bool {
    let se = a.e(1);
    // return se:IsHasType(EFFECT_TYPE_ACTIONS) and se:GetHandler():IsSetCard(SET_DRYTRON)
    if !effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS) {
        return false;
    }
    let sh = effect::get_handler(d, se).nil();
    card::is_set_card(d, sh, &[SET_DRYTRON as u16])
}
// function s.sumfilter(c)
fn sumfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    // return not c:IsSummonableCard()
    !card::is_summonable_card(d, c)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,c:GetLocation())
    let loc = card::get_location(d, c) as i32;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, loc);
    true
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRYTRON) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_DRYTRON as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
    {
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_REMOVED, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_RETURN)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_RETURN, None, None);
        }
    }
}
