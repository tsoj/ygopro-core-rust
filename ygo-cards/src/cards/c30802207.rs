//! Exosister Carpedivem
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 30802207;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

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
    // Cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_EXOSISTER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_EXOSISTER as u16]))));
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, lua_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect() and re:GetHandler():IsSummonLocation(LOCATION_GRAVE)
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_summon_location(d, h, LOCATION_GRAVE)
}
/// `s.negcfilter(c,tp)`
fn negcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsXyzSummoned() and c:IsSetCard(SET_EXOSISTER) and c:IsSummonPlayer(tp)
    card::is_xyz_summoned(d, c) && card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_summon_player(d, c, tp)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.negcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| negcfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // local ac=Duel.AnnounceCard(tp)
    let ac = duel::announce_card(d, a.tp, &[]);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD as i32);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // c:SetHint(CHINT_CARD,ac)
    card::set_hint(d, c, CHINT_CARD as u8, ac as u64);
    // Negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    // e1:SetLabel(ac)
    effect::set_label(d, e1, &[ac]);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetLabel(ac)
    effect::set_label(d, e2, &[ac]);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // Reset the card hint at the end of the turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_TURN_END)
    effect::set_code(d, e3, EVENT_TURN_END);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) e:GetHandler():SetHint(0,0) e:Reset() return false end)
    effect::set_condition(d, e3, Some(condition(reset_hint_cond)));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
/// `function(e) e:GetHandler():SetHint(0,0) e:Reset() return false end` (of e3 in `s.negop`)
fn reset_hint_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::set_hint(d, h, 0, 0);
    effect::reset(d, a.e);
    false
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, e);
    // local code1,code2=c:GetOriginalCodeRule()
    let v = card::get_original_code_rule(d, c);
    // return code1==code or code2==code
    v.first().map(|&x| x as i64) == Some(code) || v.get(1).map(|&x| x as i64) == Some(code)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // local code1,code2=re:GetHandler():GetOriginalCodeRule()
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    let v = card::get_original_code_rule(d, h);
    // return code1==code or code2==code
    v.first().map(|&x| x as i64) == Some(code) || v.get(1).map(|&x| x as i64) == Some(code)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at_card = duel::get_attacker(d).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return (a:IsControler(tp) and a:IsSetCard(SET_EXOSISTER))
    //     or (at and at:IsControler(tp) and at:IsFaceup() and at:IsSetCard(SET_EXOSISTER))
    if card::is_controler(d, at_card, a.tp) && card::is_set_card(d, at_card, &[SET_EXOSISTER as u16]) {
        return true;
    }
    match at {
        Some(at) => card::is_controler(d, at, a.tp) && card::is_faceup(d, at) && card::is_set_card(d, at, &[SET_EXOSISTER as u16]),
        None => false,
    }
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.desfilter(chkc) and chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return desfilter(d, chkc) && card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let tc = duel::select_target(d, a.tp, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tc), 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
