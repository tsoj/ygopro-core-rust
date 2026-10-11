//! Flower Cardian Lightshower
use super::prelude::*;

pub const ID: u32 = 42291297;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),3,3)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 3, 3, None, None, None, None, None);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_FLOWER_CARDIAN))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]))));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be target
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_DRAW)
    effect::set_code(d, e4, EVENT_DRAW);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // ep effects
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e5, CATEGORY_DISABLE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.epcon)
    effect::set_condition(d, e5, Some(condition(epcon)));
    // e5:SetTarget(s.eptg)
    effect::set_target(d, e5, Some(cost_target(eptg)));
    // e5:SetOperation(s.epop)
    effect::set_operation(d, e5, Some(operation(epop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and r==REASON_RULE
    a.ep != a.tp && a.r == REASON_RULE
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1500)
    duel::set_target_param(d, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.epcon(e,tp,eg,ep,ev,re,r,rp)`
fn epcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.eptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EFFECT as u64);
    // local op=0
    // if c:IsNegatableMonster() then op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    // else op=Duel.SelectOption(tp,aux.Stringid(id,1)) end
    let op = if card::is_negatable_monster(d, c) {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)])
    };
    // if op==0 then
    if op == 0 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,c,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(c)), 1, 0, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.epop(e,tp,eg,ep,ev,re,r,rp)`
fn epop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_SKIP_DP)
        effect::set_code(d, e1, EFFECT_SKIP_DP);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    // elseif c:IsFaceup() and c:IsRelateToEffect(e) then
    else if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.NegateRelatedChain(c,RESET_TURN_SET)
        duel::negate_related_chain(d, c, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
