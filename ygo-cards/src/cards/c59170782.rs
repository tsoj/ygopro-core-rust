//! Mermail Abysstrite
use super::prelude::*;

pub const ID: u32 = 59170782;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MERMAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,3,3)
    xyz::add_procedure(d, c, None, Some(3), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Change target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.tgcon1)
    effect::set_condition(d, e1, Some(condition(tgcon1)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetOperation(s.tgop1)
    effect::set_operation(d, e1, Some(operation(tgop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.tgcon2)
    effect::set_condition(d, e2, Some(condition(tgcon2)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetOperation(s.tgop2)
    effect::set_operation(d, e2, Some(operation(tgop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tgcon1(e,tp,eg,ep,ev,re,r,rp)
fn tgcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsSpellTrapEffect() or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == a.tp || !effect::is_spell_trap_effect(d, a.re.nil()) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g or #g~=1 then return false end
    let Some(g) = g else { return false };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc==c or tc:IsControler(1-tp) or tc:IsFacedown() or not tc:IsLocation(LOCATION_MZONE) or not tc:IsSetCard(SET_MERMAIL) then return false end
    if tc == c
        || card::is_controler(d, tc, 1 - a.tp)
        || card::is_facedown(d, tc)
        || !card::is_location(d, tc, LOCATION_MZONE)
        || !card::is_set_card(d, tc, &[SET_MERMAIL as u16])
    {
        return false;
    }
    // return Duel.CheckChainTarget(ev,c)
    duel::check_chain_target(d, a.ev as u8, c)
}
// function s.tgop1(e,tp,eg,ep,ev,re,r,rp)
fn tgop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if Duel.CheckChainTarget(ev,c) then
        if duel::check_chain_target(d, a.ev as u8, c) {
            // local g=Group.CreateGroup()
            // g:AddCard(c)
            let g = CardSet::from_card(c);
            // Duel.ChangeTargetCard(ev,g)
            duel::change_target_card(d, a.ev as u8, &g);
        }
    }
}
// function s.tgcon2(e,tp,eg,ep,ev,re,r,rp)
fn tgcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // if tp==Duel.GetTurnPlayer() then return false end
    if a.tp == duel::get_turn_player(d) {
        return false;
    }
    // local at=Duel.GetAttackTarget()
    // if at and at:IsFaceup() and at:IsSetCard(SET_MERMAIL) then
    if let Some(at) = duel::get_attack_target(d) {
        if card::is_faceup(d, at) && card::is_set_card(d, at, &[SET_MERMAIL as u16]) {
            // local ag=eg:GetFirst():GetAttackableTarget()
            let tc = group::get_first(d, &a.eg.clone().nil()).nil();
            let (ag, _) = card::get_attackable_target(d, tc);
            // return ag:IsContains(e:GetHandler())
            let c = effect::get_handler(d, a.e).nil();
            return ag.contains(c);
        }
    }
    // return false
    false
}
// function s.tgop2(e,tp,eg,ep,ev,re,r,rp)
fn tgop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && !card::is_immune_to_effect(d, duel::get_attacker(d).nil(), a.e) {
        // Duel.ChangeAttackTarget(c)
        duel::change_attack_target(d, Some(c), false);
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MERMAIL) and c:GetCode()~=id and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MERMAIL as u16])
        && card::get_code(d, c).first() != Some(&ID)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
