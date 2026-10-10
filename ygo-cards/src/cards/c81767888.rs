//! Branded Sword
use super::prelude::*;

pub const ID: u32 = 81767888;

// s.listed_names={CARD_ALBAZ,id+1}
// s.listed_series={SET_BRANDED}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ, ID + 1], listed_series: &[SET_BRANDED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 banished "Fallen of Albaz" or monster that lists it to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE as u32));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BRANDED) and c:IsSpellTrap() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_BRANDED as u16]) && card::is_spell_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2500,2000,8,RACE_DRAGON,ATTRIBUTE_DARK)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2500),
        defense: Some(2000),
        level: Some(8),
        race: Some(RACE_DRAGON),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local bg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let bg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return #bg>0 and ft>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2500,2000,8,RACE_DRAGON,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return !bg.is_empty() && ft > 0 && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // ft=math.min(ft,#bg)
    ft = ft.min(bg.len() as i32);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,ft,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()~=0
        let res = effect::get_label(d, a.e) != 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ct as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, a.tp, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<ct
    //   or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2500,2000,8,RACE_DRAGON,ATTRIBUTE_DARK) then return end
    if (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64) < ct || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ct>1 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() && ct > 1 {
        return;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_ALBAZ) or (c:IsMonster() and c:ListsCode(CARD_ALBAZ)))
    //   and c:IsAbleToHand()
    card::is_faceup(d, c) && (card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_ALBAZ])))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
    }
}
