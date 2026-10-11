//! Witchcrafter Scroll
//! ウィッチクラフト・スクロール
use super::prelude::*;

pub const ID: u32 = 19673561;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

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
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // replace discard effect
    // local e2=Witchcrafter.CreateCostReplaceEffect(c)
    let e2 = aux::witchcrafter::create_cost_replace_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(base) return base:GetHandler():IsAbleToGraveAsCost() end)
    effect::set_condition(d, e2, Some(lua_fn(replace_cond)));
    // e2:SetOperation(function(base) Duel.SendtoGrave(base:GetHandler(),REASON_COST) end)
    effect::set_operation(d, e2, Some(lua_fn(replace_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to S/T Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.tfcond)
    effect::set_condition(d, e4, Some(condition(tfcond)));
    // e4:SetTarget(s.tftg)
    effect::set_target(d, e4, Some(cost_target(tftg)));
    // e4:SetOperation(s.tfop)
    effect::set_operation(d, e4, Some(operation(tfop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(base) return base:GetHandler():IsAbleToGraveAsCost() end`
fn replace_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_able_to_grave_as_cost(d, h)
}
/// `function(base) Duel.SendtoGrave(base:GetHandler(),REASON_COST) end`
fn replace_op(d: &mut Duel, a: Args) {
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local rc=eg:GetFirst()
    let _rc = group::get_first(d, &a.eg.clone().nil());
    // for rc in aux.Next(eg) do
    for rc in group::iter(d, &a.eg.clone().nil()) {
        // if rc:IsStatus(STATUS_OPPO_BATTLE) then
        if card::is_status(d, rc, STATUS_OPPO_BATTLE) {
            // if rc:IsRelateToBattle() then
            if card::is_relate_to_battle(d, rc) {
                // if rc:IsControler(tp) and rc:IsRace(RACE_SPELLCASTER) then return true end
                if card::is_controler(d, rc, a.tp) && card::is_race(d, rc, RACE_SPELLCASTER) {
                    return true;
                }
            } else if card::is_previous_controler(d, rc, a.tp) && card::get_previous_race_on_field(d, rc) == RACE_SPELLCASTER {
                // if rc:IsPreviousControler(tp) and rc:GetPreviousRaceOnField()==RACE_SPELLCASTER then return true end
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.tfcond(e,tp,eg,ep,ev,re,r,rp)`
fn tfcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WITCHCRAFTER),tp,LOCATION_MZONE,0,1,nil) and Duel.IsTurnPlayer(tp)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_turn_player(d, a.tp)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
