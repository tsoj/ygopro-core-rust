//! Gravity Controller
use super::prelude::*;

pub const ID: u32 = 23656668;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.filter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| filter(d, c))), 1, Some(1), None, None, None);
    // cannot link material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetCondition(s.lkcon)
    effect::set_condition(d, e1, Some(condition(lkcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // battle indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.indescon)
    effect::set_condition(d, e2, Some(condition(indescon)));
    // e2:SetValue(s.indes)
    effect::set_value_fn(d, e2, effect_card_fn(indes));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // return
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCondition(s.tdcon)
    effect::set_condition(d, e3, Some(condition(tdcon)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_LINK) and c:GetSequence()>4
    !card::is_type(d, c, TYPE_LINK) && card::get_sequence(d, c) > 4
}
/// `s.lkcon(e)`
fn lkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsLinkSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_link_summoned(d, c)
}
/// `s.indescon(e)`
fn indescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSequence()>4
    let h = effect::get_handler(d, a.e).nil();
    card::get_sequence(d, h) > 4
}
/// `s.indes(e,c)`
fn indes(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.tdcon(e,tp,...,chk)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // return tc and tc:IsControler(1-tp) and tc:GetSequence()>4
    match tc {
        Some(tc) => card::is_controler(d, tc, 1 - a.tp) && card::get_sequence(d, tc) > 4,
        None => false,
    }
}
/// `s.tdtg(e,tp,...,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if chk==0 then return c:IsAbleToDeck() and tc and tc:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c) && tc.is_some_and(|tc| card::is_able_to_deck(d, tc));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,Group.FromCards(c,tc),2,0,0)
    let g = CardSet::from_iter(std::iter::once(c).chain(tc));
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 2, 0, 0);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if c:IsRelateToBattle() and tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, c) && card::is_relate_to_battle(d, tc.nil()) {
        // Duel.SendtoDeck(Group.FromCards(c,tc),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        let g = CardSet::from_iter(std::iter::once(c).chain(tc));
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
