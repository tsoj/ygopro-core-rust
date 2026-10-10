//! Interrupted Kaiju Slumber
use super::prelude::*;

pub const ID: u32 = 99330325;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KAIJU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy all monsters on field, then special summon 2 "Kaiju" monsters to each field, from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
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
    // Add 1 "Kaiju" monster from deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.chkfilter1(c,e,tp)`
fn chkfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_KAIJU) and c:IsMonster() and
    //     not c:IsHasEffect(EFFECT_REVIVE_LIMIT) and Duel.IsPlayerCanSpecialSummon(tp,0,POS_FACEUP,tp,c)
    //     and Duel.IsExistingMatchingCard(s.chkfilter2,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    card::is_set_card(d, c, &[SET_KAIJU as u16])
        && card::is_monster(d, c)
        && card::is_has_effect(d, c, EFFECT_REVIVE_LIMIT, None).is_empty()
        && duel::is_player_can_special_summon(d, tp, Some((0, POS_FACEUP as u8, tp, c)))
        && {
            let code = card::get_code(d, c).first().copied().nil();
            duel::is_existing_matching_card(d, Some(&|d, x| chkfilter2(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.chkfilter2(c,e,tp,cd)`
fn chkfilter2(d: &mut Duel, c: CardId, _e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsSetCard(SET_KAIJU) and c:IsMonster() and not c:IsCode(cd)
    //     and not c:IsHasEffect(EFFECT_REVIVE_LIMIT) and Duel.IsPlayerCanSpecialSummon(tp,0,POS_FACEUP,1-tp,c)
    card::is_set_card(d, c, &[SET_KAIJU as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[cd])
        && card::is_has_effect(d, c, EFFECT_REVIVE_LIMIT, None).is_empty()
        && duel::is_player_can_special_summon(d, tp, Some((0, POS_FACEUP as u8, 1 - tp, c)))
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_KAIJU) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    card::is_set_card(d, c, &[SET_KAIJU as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && {
            let code = card::get_code(d, c).first().copied().nil();
            duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.filter2(c,e,tp,cd)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsSetCard(SET_KAIJU) and c:IsMonster() and not c:IsCode(cd)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK,1-tp)
    card::is_set_card(d, c, &[SET_KAIJU as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[cd])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>-Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>-Duel.GetFieldGroupCount(1-tp,LOCATION_MZONE,0)
    //     and Duel.IsExistingMatchingCard(s.chkfilter1,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -duel::get_field_group_count(d, tp, LOCATION_MZONE, 0)
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > -duel::get_field_group_count(d, 1 - tp, LOCATION_MZONE, 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| chkfilter1(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dg=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let dg = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // if Duel.Destroy(dg,REASON_EFFECT)==0 then return end
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #sg>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 then
    if !sg.is_empty()
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local g1=sg:Select(tp,1,1,nil)
        let g1 = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        // local tc1=g1:GetFirst()
        let tc1 = group::get_first(d, &g1);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,e,tp,tc1:GetCode())
        let code1 = card::get_code(d, tc1.nil()).first().copied().nil();
        let g2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, a.e, tp, code1)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc2=g2:GetFirst()
        let tc2 = group::get_first(d, &g2);
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon_step(d, tc1.nil(), 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,1-tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon_step(d, tc2.nil(), 0, tp, 1 - tp, false, false, POS_FACEUP_ATTACK as u8, None);
        // Cannot change their battle positions
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3313)
        effect::set_description(d, e1, 3313);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e1)
        card::register_effect(d, tc1.nil(), e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // tc2:RegisterEffect(e2)
        card::register_effect(d, tc2.nil(), e2, false);
        // Must attack, if able to
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetDescription(3200)
        effect::set_description(d, e3, 3200);
        // e3:SetCode(EFFECT_MUST_ATTACK)
        effect::set_code(d, e3, EFFECT_MUST_ATTACK);
        // tc1:RegisterEffect(e3)
        card::register_effect(d, tc1.nil(), e3, false);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // tc2:RegisterEffect(e4)
        card::register_effect(d, tc2.nil(), e4, false);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KAIJU) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_KAIJU as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
