//! 終刻獄徒 ディアクトロス / DoomZ Break - Diactorus
use super::prelude::*;

pub const ID: u32 = 31010081;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MEDIUS_THE_PURE], listed_series: &[SET_DOOMZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Destroy 1 "DoomZ" card in your hand or face-up field, then you can destroy 1 monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand, or send to the GY, 1 Equip Spell from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thtgcon)
    effect::set_condition(d, e2, Some(condition(thtgcon)));
    // e2:SetTarget(s.thtgtg)
    effect::set_target(d, e2, Some(cost_target(thtgtg)));
    // e2:SetOperation(s.thtgop)
    effect::set_operation(d, e2, Some(operation(thtgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DOOMZ) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_DOOMZ as u16]) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,0,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 0, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,LOCATION_MZONE)>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, LOCATION_MZONE) > 0
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let dg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.HintSelection(dg)
            duel::hint_selection(d, dg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
/// `s.thtgconfilter(c)`
fn thtgconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_DOOMZ) and c:IsMonster()) or c:IsCode(CARD_MEDIUS_THE_PURE)
    (card::is_set_card(d, c, &[SET_DOOMZ as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_MEDIUS_THE_PURE])
}
/// `s.thtgcon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE) and c:GetOverlayGroup():IsExists(s.thtgconfilter,1,nil)
    card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && {
            let overlay = card::get_overlay_group(d, c);
            group::is_exists(d, &overlay, &thtgconfilter, 1, NO_EX)
        }
}
/// `s.thtgfilter(c)`
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsEquipSpell() and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_equip_spell(d, c) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thtgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thtgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thtgop(e,tp,eg,ep,ev,re,r,rp)`
fn thtgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
    // local g=Duel.SelectMatchingCard(tp,s.thtgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thtgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // aux.ToHandOrElse(g,tp)
        aux::to_hand_or_else(d, g, a.tp, None, None, None);
    }
}
