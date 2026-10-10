//! Light and Darkness Dragon
use super::prelude::*;

pub const ID: u32 = 47297616;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:AddCannotBeSpecialSummoned()
    card::add_cannot_be_special_summoned(d, c);
    // While face-up on the field, this card is also DARK-Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of a Spell/Trap Card or monster effect, and if you do, this card loses exactly 500 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) or re:IsMonsterEffect() end)
    effect::set_condition(d, e2, Some(condition(|d, a| effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, a.re.nil()))));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all cards you control and Special Summon 1 monster from your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(function(e) return e:GetHandler():IsReason(REASON_DESTROY) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_reason(d, h, REASON_DESTROY)
    })));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // "Double Snare" validity
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None);
    }
    // if c:IsHasEffect(EFFECT_REVERSE_UPDATE) then
    if !card::is_has_effect(d, c, EFFECT_REVERSE_UPDATE, None).is_empty() {
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:IsAttackAbove(500) and c:IsDefenseAbove(500)
    //     and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.GetCurrentChain()==ev+1
    //     and Duel.NegateActivation(ev) then
    if card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_attack_above(d, c, 500)
        && card::is_defense_above(d, c, 500)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::get_current_chain(d, false) == a.ev as i32 + 1
        && duel::negate_activation(d, a.ev as u8)
    {
        // This card loses exactly 500 ATK/DEF
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-500)
        effect::set_value(d, e1, -500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp)
    //     and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_controler(d, chkc, a.tp)
            && card::is_can_be_special_summoned(d, chkc, a.e, 0, a.tp, false, false, None, None, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,0,1,1,nil,e,0,tp,false,false)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // local dg=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,0)
    let dg = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,tp,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
