//! How Did Dai Get Here?
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 91323605;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp,rp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8, rp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    //     and ((c:IsReason(REASON_EFFECT) and rp==1-tp)
    //     or (c:IsReason(REASON_BATTLE) and Duel.GetAttacker():IsControler(1-tp)))
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && ((card::is_reason(d, c, REASON_EFFECT) && rp == 1 - tp)
            || (card::is_reason(d, c, REASON_BATTLE) && card::is_controler(d, duel::get_attacker(d).nil(), 1 - tp)))
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp,rp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp, a.rp), 1, NO_EX)
}
// function s.fieldfilter(c,codes)
fn fieldfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsFieldSpell() and not c:IsCode(codes) and not c:IsForbidden()
    card::is_field_spell(d, c) && !card::is_code(d, c, codes) && !card::is_forbidden(d, c)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,nil):GetClass(Card.GetCode)`:
/// the codes of the face-up Field Spells (the table `GetClass` returns, in its sorted order)
fn fzone_codes(d: &mut Duel) -> Vec<u32> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    group::get_class(d, &g, &|d, c| card::get_code(d, c)[0] as i64)
        .into_iter()
        .map(|v| v as u32)
        .collect()
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local codes=Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,nil):GetClass(Card.GetCode)
        let codes = fzone_codes(d);
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // return #sg>=5 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.fieldfilter,tp,LOCATION_DECK,0,1,nil,codes)
        //     and aux.SelectUnselectGroup(sg,e,tp,5,5,aux.dncheck,0)
        return sg.len() >= 5
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| fieldfilter(d, c, &codes)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && {
                let rs = aux::dncheck();
                matches!(
                    aux::select_unselect_group(d, &sg, Some(a.e), tp, Some(5), Some(5), Some(&rs), 0, tp, None, None, None, false),
                    Err(true)
                )
            };
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local codes=Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,nil):GetClass(Card.GetCode)
    let codes = fzone_codes(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.fieldfilter,tp,LOCATION_DECK,0,1,1,nil,codes):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| fieldfilter(d, c, &codes)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // if Duel.MoveToField(tc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::move_to_field(d, tc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // if #sg<5 then return end
        if sg.len() < 5 {
            return;
        }
        // local g=aux.SelectUnselectGroup(sg,e,tp,5,5,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let rs = aux::dncheck();
        let g = aux::select_unselect_group(
            d,
            &sg,
            Some(a.e),
            tp,
            Some(5),
            Some(5),
            Some(&rs),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // local sc=g:RandomSelect(1-tp,1)
        let sc = group::random_select(d, &g, 1 - tp, 1);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, sc.clone());
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
    }
}
