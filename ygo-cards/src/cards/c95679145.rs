//! Dogmatika Maximus
use super::prelude::*;

pub const ID: u32 = 95679145;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 2 monsters with different names from extra deck to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c,ft)`: a Fusion, Synchro, Xyz or Link monster that can be banished as a cost
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,1,nil,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,1,nil,ft)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (for the lines below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_EXTRA, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>=2 and #g2>=2 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2 && g2.len() >= 2;
    }
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=2 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_TOGRAVE)
        let dn = aux::dncheck();
        let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&dn), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
    // local dg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,nil)
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_EXTRA, NO_EX);
    // if #dg>=2 then
    if dg.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local tg=dg:Select(1-tp,2,2,nil)
        let tg = group::select(d, &dg, 1 - tp, 2, 2, false, NO_EX).nil();
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
    }
    // local ge1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
    // ge1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, ge1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // ge1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, ge1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // ge1:SetTargetRange(1,0)
    effect::set_target_range(d, ge1, 1, 0);
    // ge1:SetTarget(s.splimit)
    effect::set_target(d, ge1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // ge1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge1,tp)
    duel::register_effect(d, ge1, tp);
    // aux.RegisterClientHint(e:GetHandler(),EFFECT_FLAG_OATH,tp,1,0,aux.Stringid(id,2),nil)
    aux::register_client_hint(d, c, EFFECT_FLAG_OATH, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // aux.addTempLizardCheck(e:GetHandler(),tp)
    aux::add_temp_lizard_check(d, c, tp, None, None, None, None, None);
}
