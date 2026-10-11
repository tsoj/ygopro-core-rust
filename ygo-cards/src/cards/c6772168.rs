//! Invoked Oceanus
//! 召喚獣オーケアノス
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 6772168;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALEISTER],
    material_setcode: &[SET_ALEISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Aleister" monster + 1 DARK or WATER monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALEISTER),aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK|ATTRIBUTE_WATER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ALEISTER as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, (ATTRIBUTE_DARK | ATTRIBUTE_WATER) as u32, Some(fc), st, Some(tp)))),
        ],
    );
    // While this card is in the Main Monster Zone, your opponent's monsters cannot target monsters for attacks, except this one
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MMZONE)
    effect::set_range(d, e1, LOCATION_MMZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c~=e:GetHandler() end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| c != effect::get_handler(d, e).nil()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While this card is in the Extra Monster Zone, any monster sent to your opponent's GY is banished instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    // e2:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e2, LOCATION_EMZONE);
    // e2:SetTarget(s.rmtarget)
    effect::set_target(d, e2, Some(effect_card_fn(rmtarget)));
    // e2:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e2, LOCATION_ALL, LOCATION_ALL);
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can banish this card from your GY, then target 1 Fusion Monster you control; it can attack directly this turn. You can only use this effect of "Invoked Okeanos" once per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_able_to_enter_bp(d))));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.diratktg)
    effect::set_target(d, e3, Some(cost_target(diratktg)));
    // e3:SetOperation(s.diratkop)
    effect::set_operation(d, e3, Some(operation(diratkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.rmtarget(e,c)
fn rmtarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return not (c:IsOwner(tp) or c:IsSpellTrap() or c:IsLocation(LOCATION_OVERLAY)) and Duel.IsPlayerCanRemove(tp,c)
    !(card::is_owner(d, c, tp) || card::is_spell_trap(d, c) || card::is_location(d, c, LOCATION_OVERLAY))
        && duel::is_player_can_remove(d, tp, Some((c, None)))
}
// function s.diratktgfilter(c)
fn diratktgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFusionMonster() and c:IsFaceup() and not c:IsHasEffect(EFFECT_DIRECT_ATTACK)
    card::is_fusion_monster(d, c) && card::is_faceup(d, c) && card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty()
}
// function s.diratktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn diratktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.diratktgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && diratktgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.diratktgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&diratktgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.diratktgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&diratktgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.diratkop(e,tp,eg,ep,ev,re,r,rp)
fn diratkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // --It can attack directly this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
