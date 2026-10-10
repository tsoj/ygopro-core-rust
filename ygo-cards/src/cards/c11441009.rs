//! P.U.N.K. JAM FEVER!
use super::prelude::*;

pub const ID: u32 = 11441009;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 8 monsters, or 1 "P.U.N.K." Fusion or Synchro Monster
    // Xyz.AddProcedure(c,nil,8,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(d, c, None, Some(8), 2, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), Some(xyz::MaxCt::N(2)), Some(lua_fn(xyzop)), false, None);
    // Draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.AND(Cost.PayLP(600),Cost.DetachFromSelf(1)))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::pay_lp(cost::PayLp::Amount(600)), cost::detach_from_self(cost::Count::N(1), None, None)])));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of another monster's effect, and if you do, destroy that monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(Cost.DetachFromSelf(1,1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PUNK,xyzc,SUMMON_TYPE_XYZ,tp)
    //     and (c:IsType(TYPE_FUSION,xyzc,SUMMON_TYPE_XYZ,tp) or c:IsType(TYPE_SYNCHRO,xyzc,SUMMON_TYPE_XYZ,tp))
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_PUNK as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && (card::is_type_ctx(d, c, TYPE_FUSION, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
            || card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp)))
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // return true
    true
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.negconfilter(c)`
fn negconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(3) and c:IsRace(RACE_PSYCHIC)
    card::is_level(d, c, &[3]) && card::is_race(d, c, RACE_PSYCHIC)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler()~=e:GetHandler() and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(s.negconfilter,tp,LOCATION_GRAVE,0,1,nil)
    let re = a.re.nil();
    effect::get_handler(d, re).nil() != effect::get_handler(d, a.e).nil()
        && effect::is_monster_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&negconfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
