//! Number S38: Titanic Galaxy
use super::prelude::*;

pub const ID: u32 = 16110708;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[63767246], xyz_number: Some(38), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 9 LIGHT monsters, or 1 "Number 38: Hope Harbinger Dragon Titanic Galaxy"
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT),9,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(9),
        3,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Gains 200 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:GetOverlayCount()*200 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_overlay_count(d, c) as i64 * 200));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach up to 2 Spells/Traps your opponent controls to this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make this card's original ATK become 1500 and able to attack directly
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return not e:GetHandler():IsBaseAttack(1500) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::is_base_attack(d, h, 1500)
        })),
    );
    // e3:SetCost(Cost.DetachFromSelf(1,1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// s.ovfilter(c,tp,lc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,63767246) and c:IsFaceup()
    card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[63767246]) && card::is_faceup(d, c)
}
/// s.attachfilter(c,xc,tp)
fn attachfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT)
    card::is_spell_trap(d, c) && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and s.attachfilter(chkc,c,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && attachfilter(d, chkc, c, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.attachfilter,tp,0,LOCATION_ONFIELD,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| attachfilter(d, x, c, a.tp)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.attachfilter,tp,0,LOCATION_ONFIELD,1,2,nil,c,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| attachfilter(d, x, c, a.tp)), a.tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // local tc1,tc2=g:GetFirst(),g:GetNext()
    let order = group::iter(d, &g);
    let tc1 = order.first().copied();
    let tc2 = order.get(1).copied();
    // Duel.SetChainLimit(function(e,rp,tp) local rc=e:GetHandler() return rc~=tc1 and rc~=tc2 end)
    duel::set_chain_limit(
        d,
        lua_fn(move |d, x: Args| {
            let rc = effect::get_handler(d, x.e(0));
            rc != tc1 && rc != tc2
        }),
    );
    true
}
/// s.attachop(e,tp,eg,ep,ev,re,r,rp)
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Filter(s.attachfilter,nil,c,tp):Remove(Card.IsImmuneToEffect,nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let mut tg = group::filter(d, &targets, &|d, x| attachfilter(d, x, c, a.tp), NO_EX);
    group::remove(d, &mut tg, &|d, x| card::is_immune_to_effect(d, x, a.e), NO_EX);
    // if #tg>0 then Duel.Overlay(c,tg,true) end
    if !tg.is_empty() {
        duel::overlay(d, c, tg, true);
    }
}
/// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if c:IsFaceup() then
        if card::is_faceup(d, c) {
            // Its original ATK becomes 1500 until the end of this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_BASE_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
            // e1:SetValue(1500)
            effect::set_value(d, e1, 1500);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        // It can attack directly this turn
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3205)
        effect::set_description(d, e2, 3205);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
