//! Teller of Fairy Tails
//! 妖精伝姫を語る者
use super::prelude::*;

pub const ID: u32 = 4026187;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FAIRY_TAIL],
    listed_names: &[CARD_PRINCE_OF_FAIRIES],
    material_setcode: &[SET_FAIRY_TAIL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 LIGHT "Fairy Tail" monster + 1 Spellcaster monster
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, cc, fc, st, tp| matfilter(d, cc, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, cc, fc, st, tp| card::is_race_ctx(d, cc, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // Your "Fairy Tail" monsters cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_FAIRY_TAIL) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_FAIRY_TAIL as u16]))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy as many cards your opponent controls as possible, and if you do, inflict 500 damage to your opponent for each card destroyed
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2a, CATEGORY_DESTROY | CATEGORY_DAMAGE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and e:GetLabel()==1 end)
    effect::set_condition(
        d,
        e2a,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_fusion_summoned(d, h) && effect::get_label(d, a.e) == 1
        })),
    );
    // e2a:SetTarget(s.destg)
    effect::set_target(d, e2a, Some(cost_target(destg)));
    // e2a:SetOperation(s.desop)
    effect::set_operation(d, e2a, Some(operation(desop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2b, EFFECT_TYPE_SINGLE);
    // e2b:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2b, EFFECT_MATERIAL_CHECK);
    // e2b:SetValue(function(e,c) e2a:SetLabel(c:GetMaterial():IsExists(Card.IsCode,1,nil,CARD_PRINCE_OF_FAIRIES) and 1 or 0) end)
    effect::set_value_fn(
        d,
        e2b,
        lua_fn(move |d, a| {
            let cc = a.c(1);
            let mats = card::get_material(d, cc);
            let v = group::is_exists(d, &mats, &|d, m| card::is_code(d, m, &[CARD_PRINCE_OF_FAIRIES]), 1, NO_EX);
            effect::set_label(d, e2a, &[if v { 1 } else { 0 }]);
            Value::Nil
        }),
    );
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Negate the activation of an opponent's card or effect, and if you do, destroy that card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_NEGATE | CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP | EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            if a.rp != 1 - a.tp {
                return false;
            }
            let h = effect::get_handler(d, a.e).nil();
            if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
                return false;
            }
            duel::is_chain_negatable(d, a.ev as u8)
        })),
    );
    // e3:SetCost(s.negcost)
    effect::set_cost(d, e3, Some(cost_target(negcost)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,fc,sumtype,tp) and c:IsSetCard(SET_FAIRY_TAIL,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_FAIRY_TAIL as u16], Some(fc), sumtype, Some(tp))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, n as i32 * 500);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if #g>0 then
    if !g.is_empty() {
        // local dam=Duel.Destroy(g,REASON_EFFECT)*500
        let dam = duel::destroy(d, g, REASON_EFFECT, None, None) as i64 * 500;
        // if dam>0 then
        if dam > 0 {
            // Duel.Damage(1-tp,dam,REASON_EFFECT)
            duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
        }
    }
}
/// `s.negcostfilter(c)`
fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FAIRY_TAIL) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_FAIRY_TAIL as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.negcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&negcostfilter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&negcostfilter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, tp, 0);
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
