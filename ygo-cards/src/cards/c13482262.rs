//! 魔神儀の隠れ房
//! Impcantation Thanatosis
use super::prelude::*;

pub const ID: u32 = 13482262;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IMPCANTATION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(lua_fn(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.revfilter(c,e,tp)`
fn revfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_IMPCANTATION) and c:IsMonster() and c:IsAbleToDeck() and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,2,nil,e,tp,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_IMPCANTATION as u16])
        && card::is_type(d, c, TYPE_MONSTER)
        && card::is_able_to_deck(d, c)
        && !card::is_public(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 2, NO_EX)
}
/// `s.spfilter(c,e,tp,cd)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsCode(cd) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[cd]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| revfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).unwrap_or_default();
        let tc = group::get_first(d, &tc).nil();
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,2,2,nil,e,tp,tc:GetCode())
        let code = card::get_code(d, tc)[0];
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, code)), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX)
            .unwrap_or_default();
        // if #sg==2 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)==2 then
        if sg.len() == 2 && duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 2 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.descfilter(c,tp)`
fn descfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_RITUAL) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_RITUAL) && card::is_controler(d, c, tp)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.descfilter,1,nil,tp)
    let eg = a.eg();
    group::is_exists(d, &eg, &|d, c| descfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.card(9) {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.int(8) == 0 {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
