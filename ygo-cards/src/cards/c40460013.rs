//! Zalamander Catalyzer
//! 炎魔の触媒
use super::prelude::*;

pub const ID: u32 = 40460013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon and discard
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcostfilter(c,e,tp,dc_chk,sp_chk)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, dc_chk: bool, sp_chk: bool) -> bool {
    // return c:IsRace(RACE_FIEND) and not c:IsPublic()
    //     and ((sp_chk and c:IsDiscardable(REASON_EFFECT)) or (dc_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_race(d, c, RACE_FIEND)
        && !card::is_public(d, c)
        && ((sp_chk && card::is_discardable(d, c, Some(REASON_EFFECT), None))
            || (dc_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dc_chk=c:IsDiscardable(REASON_EFFECT)
    let dc_chk = card::is_discardable(d, c, Some(REASON_EFFECT), None);
    // local sp_chk=c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let sp_chk = card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    // if chk==0 then return (dc_chk or sp_chk) and not c:IsPublic()
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c,e,tp,dc_chk,sp_chk) end
    if a.chk == Some(0) {
        return (dc_chk || sp_chk)
            && !card::is_public(d, c)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, a.e, tp, dc_chk, sp_chk)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp,dc_chk,sp_chk)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, a.e, tp, dc_chk, sp_chk)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(g), 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and g:IsExists(Card.IsDiscardable,1,c,REASON_EFFECT)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, g, &|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None), 1, c.into())
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.spfilter,1,1,nil,e,tp,g)
    let gg = g.clone();
    let e = a.e;
    let sg = group::filter_select(d, &g, tp, &|d, x| spfilter(d, x, e, tp, &gg), 1, 1, false, NO_EX).nil();
    // if #sg==1 then
    if sg.len() == 1 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SendtoGrave(g:Sub(sg),REASON_EFFECT|REASON_DISCARD)
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        duel::send_to_grave(d, rest, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
/// `s.thconfilter(c,tp,ct)`
fn thconfilter(d: &mut Duel, c: CardId, tp: u8, ct: usize) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return (c:IsPreviousControler(tp) and c:GetPreviousRaceOnField()&RACE_FIEND>0)
    //     or (ct==1 and bc and bc:IsControler(tp) and bc:IsRace(RACE_FIEND))
    (card::is_previous_controler(d, c, tp) && (card::get_previous_race_on_field(d, c) & RACE_FIEND) > 0)
        || (ct == 1 && bc.is_some_and(|bc| card::is_controler(d, bc, tp) && card::is_race(d, bc, RACE_FIEND)))
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.thconfilter,1,nil,tp,#eg)
    let contains = match effect::get_handler(d, a.e) {
        Some(c) => group::is_contains(&eg, c),
        None => false,
    };
    let n = eg.len();
    !contains && group::is_exists(d, &eg, &|d, x| thconfilter(d, x, tp, n), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, c);
    }
}
