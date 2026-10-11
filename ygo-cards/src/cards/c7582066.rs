//! Psychic Nightmare
use super::prelude::*;

pub const ID: u32 = 7582066;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_PSYCHIC),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, sc, st, Some(tp)))),
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
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0
    duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) != 0
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) || duel::get_field_group_count(d, tp, 0, LOCATION_HAND) == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1,nil)
    let fg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    let g = group::random_select(d, &fg, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CARDTYPE as u64);
    // local op=Duel.SelectOption(tp,70,71,72)
    let op = duel::select_option(d, tp, true, &[70, 71, 72]);
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, tp, CardSet::from_card(tc));
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
    // if (op==0 and tc:IsMonster()) or (op==1 and tc:IsSpell()) or (op==2 and tc:IsTrap()) then
    let matched = (op == 0 && card::is_monster(d, tc)) || (op == 1 && card::is_spell(d, tc)) || (op == 2 && card::is_trap(d, tc));
    if matched {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
