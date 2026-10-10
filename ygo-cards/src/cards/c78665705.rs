//! Ancient Chant
//! Scripted by Hel
use super::prelude::*;

pub const ID: u32 = 78665705;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "The Winged Dragon of Ra" from your Deck or GY to your hand, and if you do, you can Tribute Summon 1 monster during your Main Phase this turn, in addition to your Normal Summon/Set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp) return Duel.IsPlayerCanSummon(tp) and Duel.IsPlayerCanAdditionalTributeSummon(tp) end)
    effect::set_condition(d, e1, Some(condition(activate_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply an "if you Tribute Summon "The Winged Dragon of Ra" this turn, its original ATK/DEF become the combined original ATK/DEF of the monsters Tributed for its Summon" effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function(e,tp) return not Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e2, Some(condition(effect2_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsPlayerCanSummon(tp) and Duel.IsPlayerCanAdditionalTributeSummon(tp) end`
fn activate_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_can_summon(d, a.tp, None) && duel::is_player_can_additional_tribute_summon(d, a.tp)
}
/// `function(e,tp) return not Duel.HasFlagEffect(tp,id) end`
fn effect2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::has_flag_effect(d, a.tp, ID, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_RA) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_RA]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // if not Duel.IsPlayerCanAdditionalTributeSummon(tp) then return end
        if !duel::is_player_can_additional_tribute_summon(d, a.tp) {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
        // You can Tribute Summon 1 monster during your Main Phase this turn, in addition to your Normal Summon/Set
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
        effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
        // e1:SetTargetRange(LOCATION_HAND,0)
        effect::set_target_range(d, e1, LOCATION_HAND, 0);
        // e1:SetValue(0x1)
        effect::set_value(d, e1, 0x1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_EXTRA_SET_COUNT)
        effect::set_code(d, e2, EFFECT_EXTRA_SET_COUNT);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,4))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 4)), 0, None);
    // If you Tribute Summon "The Winged Dragon of Ra" this turn, its original ATK/DEF become the combined original ATK/DEF of the monsters Tributed for its Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.setatkdefop)
    effect::set_operation(d, e1, Some(operation(setatkdefop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.textstatfilter(func)`: the stat, or 0 when it is negative
fn textstat(d: &mut Duel, c: CardId, attack: bool) -> i64 {
    let stat = if attack { card::get_text_attack(d, c) } else { card::get_text_defense(d, c) } as i64;
    if stat > 0 { stat } else { 0 }
}
/// `s.setatkdefop(e,tp,eg,ep,ev,re,r,rp)`
fn setatkdefop(d: &mut Duel, a: &EffArgs) {
    // local sc=eg:GetFirst()
    let sc = group::get_first(d, &a.eg()).nil();
    // if sc:IsSummonPlayer(tp) and sc:IsTributeSummoned() and sc:IsCode(CARD_RA) then
    if card::is_summon_player(d, sc, a.tp) && card::is_tribute_summoned(d, sc) && card::is_code(d, sc, &[CARD_RA]) {
        // local mg=sc:GetMaterial()
        let mg = card::get_material(d, sc);
        // local atk=mg:GetSum(s.textstatfilter(Card.GetTextAttack))
        let atk = group::get_sum(d, &mg, &|d, c| textstat(d, c, true));
        // local def=mg:GetSum(s.textstatfilter(Card.GetTextDefense))
        let def = group::get_sum(d, &mg, &|d, c| textstat(d, c, false));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        // e2:SetValue(def)
        effect::set_value(d, e2, def);
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, e2, false);
    }
}
