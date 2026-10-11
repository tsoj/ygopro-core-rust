//! Fabled Ragin
use super::prelude::*;

pub const ID: u32 = 47395382;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FABLED),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId, sc: Option<CardId>, st: u64, tp: u8| {
            card::is_set_card_ctx(d, cc, &[SET_FABLED as u16], sc, st, Some(tp))
        })),
        1,
        1,
        Some(synchro::non_tuner(None)),
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
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp,chk)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    card::is_synchro_summoned(d, effect::get_handler(d, a.e).nil())
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local h=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let h = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // return h<2 and Duel.IsPlayerCanDraw(tp,2-h)
        return h < 2 && duel::is_player_can_draw(d, a.tp, (2 - h) as u32);
    }
    // local h=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let h = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2-h)
    duel::set_target_param(d, (2 - h) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2-h)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2 - h);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local h=Duel.GetFieldGroupCount(p,LOCATION_HAND,0)
    let h = duel::get_field_group_count(d, p, LOCATION_HAND, 0);
    // if h>=2 then return end
    if h >= 2 {
        return;
    }
    // Duel.Draw(p,2-h,REASON_EFFECT)
    duel::draw(d, p, (2 - h) as u32, REASON_EFFECT);
}
