//! HTS Psyhemuth
use super::prelude::*;

pub const ID: u32 = 15028680;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if chk==0 then
    if a.chk == Some(0) {
        // return (t==c and a:IsAbleToRemove()) or (a==c and t~=nil and t:IsAbleToRemove())
        if t == Some(c) && card::is_able_to_remove(d, at.nil(), None, None, None) {
            return true;
        }
        return at == Some(c) && t.is_some() && card::is_able_to_remove(d, t.nil(), None, None, None);
    }
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if a:IsRelateToBattle() then g:AddCard(a) end
    let at = at.nil();
    if card::is_relate_to_battle(d, at) {
        group::add_card(&mut g, &CardSet::from_card(at));
    }
    // if t~=nil and t:IsRelateToBattle() then g:AddCard(t) end
    if let Some(t) = t {
        if card::is_relate_to_battle(d, t) {
            group::add_card(&mut g, &CardSet::from_card(t));
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // local g=Group.FromCards(a,d)
    let mut g = CardSet::new();
    if let Some(at) = at {
        group::add_card(&mut g, &CardSet::from_card(at));
    }
    if let Some(df) = df {
        group::add_card(&mut g, &CardSet::from_card(df));
    }
    // local rg=g:Filter(Card.IsRelateToBattle,nil)
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
