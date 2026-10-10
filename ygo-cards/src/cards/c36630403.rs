//! Shiranui Spectralsword
use super::prelude::*;

pub const ID: u32 = 36630403;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.exccon)
    effect::set_condition(d, e1, Some(aux::exccon()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,e,tp,lv)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // local clv=c:GetLevel()
    let clv = card::get_level(d, c);
    // return clv>0 and not c:IsType(TYPE_TUNER) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv+clv)
    clv > 0
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_race(d, c, RACE_ZOMBIE)
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, c2| filter2(d, c2, e, tp, lv + clv)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,lv)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:GetLevel()==lv and c:IsRace(RACE_ZOMBIE) and c:IsType(TYPE_SYNCHRO) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_level(d, c) == lv
        && card::is_race(d, c, RACE_ZOMBIE)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter1(chkc,e,tp,e:GetHandler():GetLevel()) end
    if let Some(chkc) = a.chkc() {
        let lv = card::get_level(d, h);
        return card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_GRAVE)
            && filter1(d, chkc, a.e, a.tp, lv);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    //     and e:GetHandler():IsAbleToRemove()
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil,e,tp,e:GetHandler():GetLevel()) end
    if a.chk == Some(0) {
        let hp = effect::get_handler_player(d, a.e);
        let affected = duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_some();
        if affected || !card::is_able_to_remove(d, h, None, None, None) {
            return false;
        }
        let lv = card::get_level(d, h);
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, a.tp, lv)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,e:GetHandler():GetLevel())
    let lv = card::get_level(d, h);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp, lv)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let mut g = g;
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_effect(d, tc.nil(), a.e) {
        return;
    }
    // local lv=c:GetLevel()+tc:GetLevel()
    let tc = tc.nil();
    let lv = card::get_level(d, c) + card::get_level(d, tc);
    // local g=Group.FromCards(c,tc)
    let g = CardSet::from_iter([c, tc]);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==2 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c2| filter2(d, c2, a.e, a.tp, lv)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
