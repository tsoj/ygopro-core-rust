//! Blackfeather Darkrage Dragon
use super::prelude::*;

pub const ID: u32 = 60992105;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Send up to 5 cards from the top of your Deck to the GY, then if any monsters were sent to the GY by this effect, this card gains 400 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp end)
    effect::set_condition(d, e1, Some(condition(damaged_by_self_cond)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==tp end`
fn damaged_by_self_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 1);
    true
}
/// `s.monfilter(c)`
fn monfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsLocation(LOCATION_GRAVE)
    card::is_monster(d, c) && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e);
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let mut ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if ct>5 then ct=5 end
    if ct > 5 {
        ct = 5;
    }
    // local ac=ct==1 and 1 or Duel.AnnounceNumberRange(tp,1,ct)
    let ac = if ct == 1 {
        1
    } else {
        duel::announce_number_range(d, tp, Some(1), Some(ct as u32), &[]).nil().0 as i32
    };
    // if Duel.DiscardDeck(tp,ac,REASON_EFFECT)==0 then return end
    if duel::discard_deck(d, tp, ac as u16, REASON_EFFECT) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if og:IsExists(s.monfilter,1,nil) and c:IsFaceup() and c:IsRelateToEffect(e) then
    if group::is_exists(d, &og, &monfilter, 1, NO_EX) && card::is_faceup(d, c.nil()) && card::is_relate_to_effect(d, c.nil(), e) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // This card gains 400 ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c.nil());
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(400)
        effect::set_value(d, e1, 400);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c.nil(), e1, false);
    }
}
