//! Mementotlan Tatsunootoshigo
use super::prelude::*;

pub const ID: u32 = 81677154;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEMENTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon itself from the hand
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
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Send cards from the Deck to the GY with a total Levels equal to or lower than the destroyed monster's original Level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOGRAVE);
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
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:FilterCount(Card.IsSetCard,nil,SET_MEMENTO)==#g
    group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]), NO_EX) == g.len()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MEMENTO) and c:IsFaceup() and c:HasLevel()
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,c:GetOriginalLevel())
    if !(card::is_set_card(d, c, &[SET_MEMENTO as u16]) && card::is_faceup(d, c) && card::has_level(d, c)) {
        return false;
    }
    let lv = card::get_original_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tgfilter(c,lv)`
fn tgfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsSetCard(SET_MEMENTO) and c:IsMonster() and c:IsLevelBelow(lv) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_MEMENTO as u16]) && card::is_monster(d, c) && card::is_level_below(d, c, lv) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(lv)`: the returned `function(sg,e,tp,mg)`
fn rescon(lv: u32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // return sg:GetSum(Card.GetLevel)<=lv and sg:GetClassCount(Card.GetCode)==#sg,sg:GetClassCount(Card.GetCode)~=#sg
        let sum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
        let cc = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64);
        (sum <= lv as i64 && cc == sg.len(), cc != sg.len())
    }
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.Destroy(tc,REASON_EFFECT)>0 then
    let Some(tc) = tc else {
        return;
    };
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // local lv=tc:GetOriginalLevel()
        let lv = card::get_original_level(d, tc);
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil,lv)
        let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, lv)), tp, LOCATION_DECK, 0, NO_EX);
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,lv,s.rescon(lv),1,tp,HINTMSG_TOGRAVE,s.rescon(lv))
        let rc = rescon(lv);
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            tp,
            Some(1),
            Some(lv as usize),
            Some(&rc),
            1,
            tp,
            Some(HINTMSG_TOGRAVE as u64),
            Some(&rc),
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg>0 then
        if sg.len() > 0 {
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
