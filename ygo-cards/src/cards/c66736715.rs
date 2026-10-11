//! Moissa Knight, the Comet General
use super::prelude::*;

pub const ID: u32 = 66736715;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Place this card on either the top or bottom of the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.tdcon)
    effect::set_condition(d, e1, Some(condition(tdcon)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase this turn, you can conduct 1 Pendulum Summon of a monster(s) from your hand in addition to your Pendulum Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DRAW)
    effect::set_code(d, e2, EVENT_DRAW);
    // e2:SetCondition(function(e,tp) return Pendulum.PlayerCanGainAdditionalPendulumSummon(tp,id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| pendulum::player_can_gain_additional_pendulum_summon(d, a.tp, ID))));
    // e2:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e2, Some(cost::self_reveal()));
    // e2:SetOperation(function(e,tp) Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),nil,tp,LOCATION_HAND,aux.Stringid(id,2),aux.Stringid(id,3),id) end)
    effect::set_operation(
        d,
        e2,
        Some(operation(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            pendulum::grant_additional_pendulum_summon(d, h, None, a.tp, LOCATION_HAND, Some(aux::stringid(ID, 2)), Some(aux::stringid(ID, 3)), ID, None, None);
        })),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdconfilter(c,tp)`
fn tdconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPendulumSummoned() and c:IsSummonPlayer(tp)
    card::is_pendulum_summoned(d, c) && card::is_summon_player(d, c, tp)
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.tdconfilter,1,nil,tp)
    group::is_exists(d, &a.eg(), &|d, c| tdconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsAbleToDeck() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_deck(d, c) {
        // local seq_op=0
        let mut seq_op = 0;
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 then
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 {
            // seq_op=Duel.SelectOption(tp,aux.Stringid(id,4),aux.Stringid(id,5))
            seq_op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 4), aux::stringid(ID, 5)]);
        }
        // Duel.SendtoDeck(c,nil,seq_op,REASON_EFFECT)
        duel::send_to_deck(d, c, None, seq_op, REASON_EFFECT, None);
    }
}
