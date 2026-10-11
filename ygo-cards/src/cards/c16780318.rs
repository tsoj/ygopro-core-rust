//! Super All In!
use super::prelude::*;

pub const ID: u32 = 16780318;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return synchro to extra deck, special summon 4 "Flower Cardian" monsters from GY, draw 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
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
    // Indicate the fact it was sent to GY this turn by "Flower Cardian" monster effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.exfilter(c,tp)`
fn exfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsAbleToHand()
    //     and ((c:GetSequence()<5 and ft>2) or ft>3)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_able_to_hand(d, c, None) && ((card::get_sequence(d, c) < 5 && ft > 2) || ft > 3)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FLOWER_CARDIAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,4,nil,e,tp)
    //     and Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| exfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 4, NO_EX)
            && duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,4,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 4, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| exfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local td=g:GetFirst()
    let td = group::get_first(d, &g);
    // if td and Duel.SendtoHand(td,nil,REASON_EFFECT)~=0 and td:IsLocation(LOCATION_EXTRA) then
    if let Some(td) = td {
        if duel::send_to_hand(d, td, None, REASON_EFFECT, None) != 0 && card::is_location(d, td, LOCATION_EXTRA) {
            // if Duel.GetLocationCount(tp,LOCATION_MZONE)<4 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
            if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 4
                || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
            {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,4,4,nil,e,tp)
            let f = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
            let sg = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_GRAVE, 0, 4, 4, false, NO_EX).nil();
            // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)~=0 then
            if !sg.is_empty() && duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
                let _ = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
                // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
                if duel::draw(d, tp, 1, REASON_EFFECT) == 0 {
                    return;
                }
                // local tc=Duel.GetOperatedGroup():GetFirst()
                let operated = duel::get_operated_group(d);
                let tc = group::get_first(d, &operated).nil();
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - tp, tc);
                // if tc:IsMonster() and tc:IsSetCard(SET_FLOWER_CARDIAN) then
                if card::is_monster(d, tc) && card::is_set_card(d, tc, &[SET_FLOWER_CARDIAN as u16]) {
                    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
                    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
                        // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
                        duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
                    }
                } else {
                    // If drawn card was not a "Flower Cardian" monster, destroy all monster you control and halve your LP
                    // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
                    let dg = duel::get_matching_group(d, None, tp, LOCATION_MZONE, 0, NO_EX);
                    // Duel.Destroy(dg,REASON_EFFECT)
                    duel::destroy(d, dg, REASON_EFFECT, None, None);
                    // if #Duel.GetOperatedGroup()>0 then
                    if !duel::get_operated_group(d).is_empty() {
                        // local lp=Duel.GetLP(tp)
                        let lp = duel::get_lp(d, tp);
                        // Duel.SetLP(tp,math.ceil(lp/2))
                        duel::set_lp(d, tp, (lp as f64 / 2.0).ceil() as i32);
                    }
                }
            }
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    let Some(re) = a.re else { return false };
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return e:GetHandler():IsReason(REASON_EFFECT) and rc:IsSetCard(SET_FLOWER_CARDIAN) and rc:IsMonster()
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT) && card::is_set_card(d, rc, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, rc)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.addtg)
    effect::set_target(d, e1, Some(cost_target(addtg)));
    // e1:SetOperation(s.addop)
    effect::set_operation(d, e1, Some(operation(addop)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.addfilter(c)`
fn addfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.addtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn addtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.addfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&addfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.addop(e,tp,eg,ep,ev,re,r,rp)`
fn addop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.addfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&addfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
