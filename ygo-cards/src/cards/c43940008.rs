//! Duel Tower
use super::prelude::*;

pub const ID: u32 = 43940008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Reveal monsters from the Deck and Special Summon 1 monster from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.rvop)
    effect::set_operation(d, e1, Some(operation(rvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy all cards during the next End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.desregop)
    effect::set_operation(d, e2, Some(operation(desregop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rvfilter(c,tp)`
fn rvfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsMonster() and c:GetTextAttack()>=0 and c:IsAbleToRemove(tp,POS_FACEDOWN) and not c:IsPublic()
    card::is_monster(d, c) && card::get_text_attack(d, c) >= 0 && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None) && !card::is_public(d, c)
}
/// `s.rvop(e,tp,eg,ep,ev,re,r,rp)`
fn rvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local players={Duel.GetTurnPlayer(),1-Duel.GetTurnPlayer()}
    let players = [duel::get_turn_player(d), 1 - duel::get_turn_player(d)];
    // Each player can choose to reveal
    // local rv={}
    let mut rv: [Option<CardSet>; 2] = [None, None];
    // for _,p in ipairs(players) do
    for p in players {
        // if Duel.IsExistingMatchingCard(s.rvfilter,p,LOCATION_DECK,0,1,nil,tp)
        //     and Duel.SelectYesNo(p,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| rvfilter(d, c, a.tp)), p, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, p, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
            // rv[p]=Duel.SelectMatchingCard(p,s.rvfilter,p,LOCATION_DECK,0,1,1,nil,tp)
            rv[p as usize] = duel::select_matching_card(d, p, Some(&|d, c| rvfilter(d, c, a.tp)), p, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        }
    }
    // Reveal later so player B doesn't know what player A chose
    // local atk={}
    let mut atk = [-1i64; 2];
    // for _,p in ipairs(players) do
    for p in players {
        // local g=rv[p]
        let g = rv[p as usize].clone();
        // if g and #g>0 then
        match g {
            Some(g) if !g.is_empty() => {
                // Duel.ConfirmCards(1-p,g)
                duel::confirm_cards(d, 1 - p, g.clone());
                // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
                duel::remove(d, g.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
                // atk[p]=g:GetFirst():GetAttack()
                let first = group::get_first(d, &g).nil();
                atk[p as usize] = card::get_attack(d, first) as i64;
            }
            // else atk[p]=-1 end
            _ => atk[p as usize] = -1,
        }
    }
    // The player that revealed the highest ATK can Special Summon
    // if atk[tp]==atk[1-tp] then return end
    let t = a.tp as usize;
    let o = (1 - a.tp) as usize;
    if atk[t] == atk[o] {
        return;
    }
    // local p=(atk[tp]>atk[1-tp]) and tp or 1-tp
    let p = if atk[t] > atk[o] { a.tp } else { 1 - a.tp };
    // if not Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,p,LOCATION_HAND,0,1,nil,e,0,p,false,false)
    //     or not Duel.SelectYesNo(p,aux.Stringid(id,3)) then return end
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, p, false, false, None, None, None)), p, LOCATION_HAND, 0, 1, NO_EX)
        || !duel::select_yes_no(d, p, aux::stringid(ID, 3))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(p,Card.IsCanBeSpecialSummoned,p,LOCATION_HAND,0,1,1,nil,e,0,p,false,false):GetFirst()
    let sg = duel::select_matching_card(d, p, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, p, false, false, None, None, None)), p, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // if Duel.SpecialSummonStep(sc,0,p,p,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, sc, 0, p, p, false, false, POS_FACEUP as u8, None) {
        // Can attack directly
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel() and e:GetHandler():HasFlagEffect(id)
    duel::is_turn_player(d, a.tp)
        && (duel::get_turn_count(d, None) as i64) != effect::get_label(d, a.e)
        && card::has_flag_effect(d, effect::get_handler(d, a.e).nil(), ID, None)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():ResetFlagEffect(id)
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, ID);
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.desregop(e,tp,eg,ep,ev,re,r,rp)`
fn desregop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Destroy all cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(Duel.GetTurnCount())
    effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64]);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,Duel.IsTurnPlayer(tp) and 2 or 1)
    let reset_count = if duel::is_turn_player(d, a.tp) { 2 } else { 1 };
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_count);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
}
