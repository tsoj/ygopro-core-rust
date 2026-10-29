//! Arcana Triumph Joker
use super::prelude::*;

pub const ID: u32 = 93880808;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT, CARD_JACK_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.adval)
    effect::set_value_fn(d, e2, effect_value(adval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.descost)
    effect::set_cost(d, e3, Some(cost_target(descost)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,code)`
fn cfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[code]) && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c)) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetClassCount(Card.GetCode)==3
    let chkf = aux::chkf_mmz(1);
    let ok = chkf(d, sg, e, tp, mg, None).0 && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 3;
    (ok, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,CARD_QUEEN_KNIGHT)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, CARD_QUEEN_KNIGHT)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,CARD_KING_KNIGHT)
    let g2 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, CARD_KING_KNIGHT)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // local g3=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,CARD_JACK_KNIGHT)
    let g3 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, CARD_JACK_KNIGHT)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // g1:Merge(g3)
    group::add_card(&mut g1, &g3);
    // if chk==0 then return aux.SelectUnselectGroup(g1,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g1, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(g1,e,tp,3,3,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &g1, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.adval(e,c)`: 500 for each card in your hand
fn adval(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetFieldGroupCount(0,LOCATION_HAND,LOCATION_HAND)*500
    duel::get_field_group_count(d, 0, LOCATION_HAND, LOCATION_HAND) as i64 * 500
}
/// `s.descfilter(c,tp)`
fn descfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return tpe~=0 and c:IsDiscardable()
    //     and Duel.IsExistingMatchingCard(s.dfilter,tp,0,LOCATION_ONFIELD,1,nil,tpe)
    tpe != 0 && card::is_discardable(d, c, None, None) && duel::is_existing_matching_card(d, Some(&|d, x| dfilter(d, x, tpe)), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.dfilter(c,tpe)`
fn dfilter(d: &mut Duel, c: CardId, tpe: u32) -> bool {
    // return c:IsFaceup() and c:IsType(tpe)
    card::is_faceup(d, c) && card::is_type(d, c, tpe)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.descfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| descfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // Duel.DiscardHand(tp,s.descfilter,1,1,REASON_COST|REASON_DISCARD,nil,tp)
    duel::discard_hand(d, a.tp, Some(&|d, c| descfilter(d, c, a.tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // e:SetLabel(g:GetFirst():GetMainCardType())
    let first = group::get_first(d, &g).nil();
    let mt = card::get_main_card_type(d, first);
    effect::set_label(d, a.e, &[mt as i64]);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,0,LOCATION_ONFIELD,nil,e:GetLabel())
    let tpe = effect::get_label(d, a.e) as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, tpe)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,0,LOCATION_ONFIELD,nil,e:GetLabel())
    let tpe = effect::get_label(d, a.e) as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, tpe)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
