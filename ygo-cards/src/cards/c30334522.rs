//! Gishki Psychelone
use super::prelude::*;

pub const ID: u32 = 30334522;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GISHKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Look at 1 random card and shuffle it into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) != 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    // local rc=Duel.AnnounceRace(tp,1,RACE_ALL)
    let rc = duel::announce_race(d, tp, 1, RACE_ALL);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // local at=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let at = duel::announce_attribute(d, tp, 1, ATTRIBUTE_ALL);
    // e:SetLabel(rc,at)
    effect::set_label(d, a.e, &[rc as i64, at as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local tc=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1):GetFirst()
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    let rg = group::random_select(d, &g, tp, 1);
    let tc = group::get_first(d, &rg).nil();
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, tp, tc);
    // local rc,at=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let rc = labels.first().copied().unwrap_or(0);
    let at = labels.get(1).copied().unwrap_or(0);
    // if tc:IsRace(rc) and tc:IsAttribute(at) then
    if card::is_race(d, tc, rc as u64) && card::is_attribute(d, tc, at as u32) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else {
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
