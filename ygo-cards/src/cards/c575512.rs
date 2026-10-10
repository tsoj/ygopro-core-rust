//! PSY-Frame Circuit
use super::prelude::*;

pub const ID: u32 = 575512;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PSY_FRAME], ..ScriptInfo::DEFAULT };

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
    // synchro summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.sccon)
    effect::set_condition(d, e2, Some(condition(sccon)));
    // e2:SetCost(s.sccost)
    effect::set_cost(d, e2, Some(cost_target(sccost)));
    // e2:SetTarget(s.sctg)
    effect::set_target(d, e2, Some(cost_target(sctg)));
    // e2:SetOperation(s.scop)
    effect::set_operation(d, e2, Some(operation(scop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.scfilter(c,tp)`
fn scfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PSY_FRAME) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PSY_FRAME as u16]) && card::is_controler(d, c, tp)
}
/// `s.sccon(e,tp,eg,ep,ev,re,r,rp)`
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.scfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| scfilter(d, c, tp), 1, NO_EX)
}
/// `s.sccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sccost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PSY_FRAME)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PSY_FRAME as u16])
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local mg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil,nil,mg)
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local mg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil,nil,mg)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),nil,mg)
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, Some(mg), 0, 0);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    if card::is_controler(d, tc.nil(), 1 - tp) {
        tc = duel::get_attack_target(d);
    }
    // e:SetLabelObject(tc)
    effect::set_label_object(d, e, tc.map_or(Value::Nil, Value::Card));
    // return tc and tc:IsControler(tp) and tc:IsSetCard(SET_PSY_FRAME) and tc:IsRelateToBattle() and Duel.GetAttackTarget()~=nil
    let Some(tc) = tc else {
        return false;
    };
    card::is_controler(d, tc, tp)
        && card::is_set_card(d, tc, &[SET_PSY_FRAME as u16])
        && card::is_relate_to_battle(d, tc)
        && duel::get_attack_target(d).is_some()
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PSY_FRAME) and c:GetAttack()>0 and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_PSY_FRAME as u16]) && card::get_attack(d, c) > 0 && card::is_discardable(d, c, None, None)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&atkfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, first);
    effect::set_label(d, e, &[atk as i64]);
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
