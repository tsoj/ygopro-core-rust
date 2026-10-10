//! T.G. Hyper Librarian
use super::prelude::*;

pub const ID: u32 = 90953320;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tg = group::get_first(d, &eg);
    // return #eg==1 and tg~=e:GetHandler() and tg:GetSummonType()==SUMMON_TYPE_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    eg.len() == 1 && tg != Some(h) && card::get_summon_type(d, tg.nil()) == SUMMON_TYPE_SYNCHRO
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) or e:GetHandler():IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || card::is_facedown(d, h) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
