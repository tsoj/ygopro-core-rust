//! Dracotail Arthalion
use super::prelude::*;

pub const ID: u32 = 33760966;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Dragontail" monster + 1+ monsters in the hand
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsLocation,LOCATION_HAND),1,99,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DRACOTAIL))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_location(d, c, LOCATION_HAND))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
            card::is_set_card_ctx(d, c, &[SET_DRACOTAIL as u16], Some(fc), st, Some(tp))
        }))],
    );
    // Register how many materials are used from the hand
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(function(e,c) e:SetLabel(c:GetMaterial():FilterCount(Card.IsLocation,nil,LOCATION_HAND)) end)
    effect::set_value_fn(d, e0, lua_fn(material_check));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Return monsters on the field and/or in any GY(s) to the hand, up to the number of monsters used as material from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and e:GetLabelObject():GetLabel()>0 end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from the GY, but banish it when it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of monsters being sent to the GY at the same time
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(function() return not Duel.IsPhase(PHASE_DAMAGE) end)
    effect::set_condition(d, e3, Some(condition(|d, _a| !duel::is_phase(d, PHASE_DAMAGE))));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetOperation(function(e) e:GetHandler():ResetFlagEffect(id+1) end)
    effect::set_operation(d, e4, Some(operation(reset_flag_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e5, EVENT_CHAIN_SOLVED);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EVENT_BREAK_EFFECT)
    effect::set_code(d, e6, EVENT_BREAK_EFFECT);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function(e,tp,eg,ep,ev,re,r,rp,c) ... end: e:SetLabel(c:GetMaterial():FilterCount(Card.IsLocation,nil,LOCATION_HAND))
fn material_check(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    let m = card::get_material(d, c);
    let n = group::filter_count(d, &m, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    effect::set_label(d, e, &[n as i64]);
}
// function(e) return e:GetHandler():IsFusionSummoned() and e:GetLabelObject():GetLabel()>0 end
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_fusion_summoned(d, h) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) > 0
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsMonster() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_monster(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsMonster,Card.IsAbleToHand),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_hand(d, c, None)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // local ct=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsMonster,Card.IsAbleToHand),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,ct,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_monster(d, c) && card::is_able_to_hand(d, c, None)),
        a.tp,
        LOCATION_MZONE | LOCATION_GRAVE,
        LOCATION_MZONE | LOCATION_GRAVE,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // local ct=eg:FilterCount(Card.IsMonster,nil)
    let ct = group::filter_count(d, &eg, &|d, x| card::is_monster(d, x), NO_EX);
    // if ct==0 or eg:IsContains(c) then return end
    if ct == 0 || group::is_contains(&eg, c) {
        return;
    }
    // if ct>=2 or c:HasFlagEffect(id+1) then
    if ct >= 2 || card::has_flag_effect(d, c, ID + 1, None) {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,ep,ev)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, a.ep, a.ev);
    }
    // local eff=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_EFFECT)
    let eff = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_EFFECT]);
    // if eff then
    if eff[0].truthy() {
        // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
        card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    }
}
// function(e) e:GetHandler():ResetFlagEffect(id+1) end
fn reset_flag_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID + 1);
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // --Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
