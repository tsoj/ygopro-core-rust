//! Abyss Playhouse - Fantastic Theater
use super::prelude::*;

pub const ID: u32 = 77297908;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ABYSS_ACTOR,SET_ABYSS_SCRIPT}
    listed_series: &[SET_ABYSS_ACTOR, SET_ABYSS_SCRIPT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Abyss Script" Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change a monster effect to "Destroy 1 Set Spell/Trap your opponent controls"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.chcon1)
    effect::set_condition(d, e3, Some(condition(chcon1)));
    // e3:SetOperation(s.chop1)
    effect::set_operation(d, e3, Some(operation(chop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and c:IsType(TYPE_PENDULUM) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_public(d, c)
}
/// `s.cfilter2(c,tp)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ABYSS_SCRIPT) and c:IsSpell() and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.cfilter3,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    if !(card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16]) && card::is_spell(d, c) && !card::is_public(d, c)) {
        return false;
    }
    let code = card::get_code(d, c)[0] as i64;
    duel::is_existing_matching_card(d, Some(&|d, x| cfilter3(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.cfilter3(c,code)`
fn cfilter3(d: &mut Duel, c: CardId, code: i64) -> bool {
    // return c:IsSetCard(SET_ABYSS_SCRIPT) and c:IsSpell() and not c:IsCode(code) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16]) && card::is_spell(d, c) && !card::is_code(d, c, &[code as u32]) && card::is_able_to_hand(d, c, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| cfilter2(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let mut g1 = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local g2=Duel.SelectMatchingCard(tp,s.cfilter2,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| cfilter2(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g2:GetFirst():GetCode())
    let code = card::get_code(d, group::get_first(d, &g2).nil())[0] as i64;
    effect::set_label(d, a.e, &[code]);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - tp, g1);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter3,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter3(d, c, label)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.confilter(c)`
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ABYSS_ACTOR) and c:IsPendulumSummoned()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_pendulum_summoned(d, c)
}
/// `s.chcon1(e,tp,eg,ep,ev,re,r,rp)`
fn chcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect()
    //     and Duel.IsExistingMatchingCard(s.confilter,tp,LOCATION_MZONE,0,1,nil)
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_existing_matching_card(d, Some(&confilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.chop1(e,tp,eg,ep,ev,re,r,rp)`
fn chop1(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsSpellTrap()
    card::is_facedown(d, c) && card::is_spell_trap(d, c)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
