//! Anti-GMX Final Experiment
//! Ａ－ＧＭＸ最終検証
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 1463589;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GMX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate only if you control a "GMX" monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GMX),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e0, Some(condition(e0_condition)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Once per turn, when your opponent activates a monster effect on the field (except during the Damage Step): You can excavate the top 5 cards of your Deck, then if you excavated a "GMX" card(s), negate that activated effect, also place all the excavated cards on the top or bottom of the Deck in any order
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.excavcon)
    effect::set_condition(d, e1, Some(condition(excavcon)));
    // e1:SetTarget(s.excavtg)
    effect::set_target(d, e1, Some(cost_target(excavtg)));
    // e1:SetOperation(s.excavop)
    effect::set_operation(d, e1, Some(operation(excavop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GMX),tp,LOCATION_MZONE,0,1,nil) end
fn e0_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GMX as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.excavcon(e,tp,eg,ep,ev,re,r,rp)
fn excavcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_MZONE as i64
}
// function s.excavtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=5 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 5;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
// function s.excavop(e,tp,eg,ep,ev,re,r,rp)
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local decktop5=Duel.GetDecktopGroup(tp,5)
    let decktop5 = duel::get_decktop_group(d, tp, 5);
    // if #decktop5==0 then return end
    if decktop5.is_empty() {
        return;
    }
    let n = decktop5.len();
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // Duel.RaiseEvent(decktop5,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,#decktop5)
    duel::raise_event(d, decktop5.clone(), EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, n as u32);
    // if decktop5:IsExists(Card.IsSetCard,1,nil,SET_GMX) then
    if group::is_exists(d, &decktop5, &|d, c| card::is_set_card(d, c, &[SET_GMX as u16]), 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
    // local op=0
    let mut op = 0;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>5 then
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 5 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    }
    // if op==0 then
    if op == 0 {
        // Duel.SortDecktop(tp,tp,#decktop5)
        duel::sort_decktop(d, tp, tp, n as u16);
    } else {
        // Duel.MoveToDeckBottom(decktop5,tp)
        duel::move_to_deck_bottom(d, &decktop5);
        // Duel.SortDeckbottom(tp,tp,#decktop5)
        duel::sort_deckbottom(d, tp, tp, n as u16);
    }
}
