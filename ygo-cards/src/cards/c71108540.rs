//! Invincible Demise Lord
//! 不死のデスロード
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 71108540;
const CARD_SLASH_DRAW: u32 = 71344451;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID, CARD_SLASH_DRAW],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During the End Phase, if a monster(s) was destroyed by battle this turn: You can Special Summon this card from your hand or GY, and if "Invincible Demise Lord" was destroyed by battle this turn, this card's original ATK becomes 3000, also it cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp)
    //     return Duel.HasFlagEffect(0,id)
    // end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::has_flag_effect(d, 0, ID, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can reveal cards in your Deck, including "Slash Draw", equal to the number of cards your opponent controls +1 and place those revealed cards on top of the Deck in any order
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of monsters destroyed by battle
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DESTROYED)
        effect::set_code(d, ge1, EVENT_BATTLE_DESTROYED);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        effect::set_operation(d, ge1, Some(operation(battle_destroyed_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// The operation of the global check: function(e,tp,eg,ep,ev,re,r,rp)
fn battle_destroyed_op(d: &mut Duel, a: &EffArgs) {
    // Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // if eg:IsExists(Card.IsPreviousCodeOnField,1,nil,id) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_previous_code_on_field(d, c, &[ID]), 1, NO_EX) {
        // Duel.RegisterFlagEffect(1,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 1, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    //     and Duel.HasFlagEffect(1,id) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)
        && duel::has_flag_effect(d, 1, ID, None)
    {
        // If "Invincible Demise Lord" was destroyed by battle this turn, this card's original ATK becomes 3000, also it cannot be destroyed by card effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(3000)
        effect::set_value(d, e1, 3000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3001)
        effect::set_description(d, e2, 3001);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
        let deck_count = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0);
        // return deck_count>1 and deck_count>=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)+1
        //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_DECK,0,1,nil,CARD_SLASH_DRAW)
        return deck_count > 1
            && deck_count >= duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) + 1
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_SLASH_DRAW])), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // (falls off the end in Lua: the value is not used)
    true
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local res=sg:IsExists(Card.IsCode,1,nil,CARD_SLASH_DRAW)
    let res = group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_SLASH_DRAW]), 1, NO_EX);
    // return res,not res
    (res, !res)
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_DECK,0,1,nil,CARD_SLASH_DRAW) then return end
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_SLASH_DRAW])), a.tp, LOCATION_DECK, 0, 1, NO_EX) {
        return;
    }
    // local reveal_count=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)+1
    let reveal_count = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) + 1;
    // local g=Duel.GetFieldGroup(tp,LOCATION_DECK,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_DECK, 0);
    // if #g<reveal_count then return end
    if (g.len() as i32) < reveal_count {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,reveal_count,reveal_count,s.rescon,1,tp,HINTMSG_CONFIRM)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(reveal_count as usize),
        Some(reveal_count as usize),
        Some(rs),
        1,
        a.tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg==reveal_count then
    if (sg.len() as i32) == reveal_count {
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.MoveToDeckTop(sg)
        duel::move_to_deck_top(d, sg);
        // if reveal_count>1 then Duel.SortDecktop(tp,tp,reveal_count) end
        if reveal_count > 1 {
            duel::sort_decktop(d, a.tp, a.tp, reveal_count as u16);
        }
    }
}
