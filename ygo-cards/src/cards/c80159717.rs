//! Dragunity Knight - Trident
use super::prelude::*;

pub const ID: u32 = 80159717;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_WINGEDBEAST),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WINGEDBEAST, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // extra
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.excost)
    effect::set_cost(d, e1, Some(cost_target(excost)));
    // e1:SetOperation(s.exop)
    effect::set_operation(d, e1, Some(operation(exop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.excost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn excost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)
    let mut ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_EXTRA);
    // if ct>3 then ct=3 end
    if ct > 3 {
        ct = 3;
    }
    // if chk==0 then return ct>0 and Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, ct as u16, false, NO_EX).nil();
    let n = g.len() as i64;
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[n]);
    true
}
/// `s.exop(e,tp,eg,ep,ev,re,r,rp)`
fn exop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if #g<ct then return end
    if (g.len() as i64) < ct {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sg=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,ct,ct,nil)
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_EXTRA, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - a.tp);
}
