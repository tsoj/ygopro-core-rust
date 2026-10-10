//! Aurkus, Lightsworn Druid
use super::prelude::*;

pub const ID: u32 = 7183277;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Neither player can target "Lightsworn" monsters (anywhere) with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(function(_,c) return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster() and c:IsFaceup() end)
    effect::set_target(d, e1, Some(effect_card_fn(etarget)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send the top 2 cards of your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(function(_,tp) Duel.DiscardDeck(tp,2,REASON_EFFECT) end)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.etarget(e,c)`: the target filter of e1
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
/// `function(_,tp) return Duel.IsTurnPlayer(tp) end` of e2
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
/// `function(_,tp) Duel.DiscardDeck(tp,2,REASON_EFFECT) end` of e2
fn op(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
}
