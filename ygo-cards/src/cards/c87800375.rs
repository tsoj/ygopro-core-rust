//! Until Noble Arms are Needed Once Again
use super::prelude::*;

pub const ID: u32 = 87800375;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS, SET_NOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and c:IsSetCard(SET_NOBLE_ARMS)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP) && card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
        // if ct>0 then
        if ct > 0 {
            // local g=Duel.GetDecktopGroup(tp,ct)
            let g = duel::get_decktop_group(d, a.tp, ct as u32);
            // local result=g:FilterCount(Card.IsAbleToHand,nil)>0
            let result = group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
            // return ct>0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=ct and result end
            return ct > 0 && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= ct as i32 && result;
        }
        // end (falls through: nil, which is false)
        return false;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)
    let mut ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, a.tp, ct as u32);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let g = duel::get_decktop_group(d, a.tp, ct as u32);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if sg:GetFirst():IsAbleToHand() then
    let first = group::get_first(d, &sg).nil();
    if card::is_able_to_hand(d, first, None) {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    } else {
        // Duel.SendtoGrave(sg,REASON_RULE)
        duel::send_to_grave(d, sg, REASON_RULE, None, None);
    }
    // ct=ct-1
    ct -= 1;
    // if ct>0 then Duel.SortDecktop(tp,tp,ct) end
    if ct > 0 {
        duel::sort_decktop(d, a.tp, a.tp, ct as u16);
    }
}
/// `s.eqfilter(c,cd)`
fn eqfilter(d: &mut Duel, c: CardId, cd: u32) -> bool {
    // return c:IsCode(cd) and (c:IsFaceup() or not c:IsOnField())
    card::is_code(d, c, &[cd]) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil,c:GetCode())
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && {
            let codes = card::get_code(d, c);
            !duel::is_existing_matching_card(
                d,
                Some(&|d, x| eqfilter(d, x, codes[0])),
                tp,
                LOCATION_ONFIELD | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            )
        }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
