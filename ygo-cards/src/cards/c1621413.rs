//! Dark Requiem Xyz Dragon
use super::prelude::*;

pub const ID: u32 = 1621413;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[16195942], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Xyz summon
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Change ATK to 0 and increase its own ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Negate monster effect activation and special summon 1 Xyz monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_NEGATE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_NEGATE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,16195942)
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, c| card::is_code(d, c, &[16195942]), 1, NO_EX)
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
            // local atk=tc:GetBaseAttack()
            let atk = card::get_base_attack(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // if c:IsRelateToEffect(e) and c:IsFaceup() then
            if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e2:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // e2:SetValue(atk)
                effect::set_value(d, e2, atk as i64);
                // c:RegisterEffect(e2)
                card::register_effect(d, c, e2, false);
            }
        }
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and rp~=tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    //   and e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,16195942)
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[16195942]), 1, NO_EX)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_XYZ) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rh, re) {
        return;
    }
    if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
        let e = a.e;
        let tp = a.tp;
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
