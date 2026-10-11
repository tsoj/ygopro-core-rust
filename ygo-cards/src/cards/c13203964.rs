//! Perfectron Hydradrive Dragon
use super::prelude::*;

pub const ID: u32 = 13203964;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1+ Link Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_LINK),1)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_LINK, sc, st, Some(tp)))), 1, None, None, None, None);
    // If this card is Link Summoned, or at the end of the Damage Step if this card battled: Destroy as many monsters your opponent controls as possible, then inflict 300 damage to your opponent for each Link Monster in your GY
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1a, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1a, Some(condition(cond1)));
    // e1a:SetTarget(s.destg)
    effect::set_target(d, e1a, Some(cost_target(destg)));
    // e1a:SetOperation(s.desop)
    effect::set_operation(d, e1a, Some(operation(desop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1b, EVENT_DAMAGE_STEP_END);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // While face-up on the field, this card is also LIGHT, WATER, FIRE, and WIND-Attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_ADD_ATTRIBUTE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(ATTRIBUTE_LIGHT|ATTRIBUTE_WATER|ATTRIBUTE_FIRE|ATTRIBUTE_WIND)
    effect::set_value(d, e2, (ATTRIBUTE_LIGHT | ATTRIBUTE_WATER | ATTRIBUTE_FIRE | ATTRIBUTE_WIND) as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Unaffected by the activated effects of monsters that share an Attribute with this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.immval)
    effect::set_value_fn(d, e3, lua_fn(immval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // If this card with 1000 or more ATK would be destroyed by battle or card effect, you can make its ATK become reduced by 1000 instead
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function(e) return e:GetHandler():IsLinkSummoned() end
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // local dam=300*Duel.GetMatchingGroupCount(Card.IsLinkMonster,tp,LOCATION_GRAVE,0,nil)
    let dam = damage_amount(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
// 300 * the number of Link Monsters in the GY of `tp`
fn damage_amount(d: &mut Duel, tp: u8) -> i32 {
    300 * duel::get_matching_group_count(d, Some(&|d, x| card::is_link_monster(d, x)), tp, LOCATION_GRAVE, 0, NO_EX) as i32
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local dam=300*Duel.GetMatchingGroupCount(Card.IsLinkMonster,tp,LOCATION_GRAVE,0,nil)
        let dam = damage_amount(d, a.tp);
        // if dam>0 then
        if dam > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,dam,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, dam as i64, REASON_EFFECT, false, None);
        }
    }
}
// function s.immval(e,te)
fn immval(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    // local trig_loc,trig_attr=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_ATTRIBUTE)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_ATTRIBUTE]);
    let trig_loc = info[0].int() as u32;
    let trig_attr = info[1].int() as u32;
    // if not (te:IsMonsterEffect() and te:IsActivated()) then return false end
    if !(effect::is_monster_effect(d, te) && effect::is_activated(d, te)) {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // if not Duel.IsChainSolving() or (tc:IsRelateToEffect(te) and tc:IsFaceup() and tc:IsLocation(trig_loc)) then
    if !duel::is_chain_solving(d) || (card::is_relate_to_effect(d, tc, te) && card::is_faceup(d, tc) && card::is_location(d, tc, trig_loc)) {
        // return c:IsAttribute(tc:GetAttribute())
        let att = card::get_attribute(d, tc);
        card::is_attribute(d, c, att)
    } else {
        // return c:GetAttribute()&trig_attr>0
        (card::get_attribute(d, c) & trig_attr) > 0
    }
}
// function s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAttackAbove(1000) and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE) end
    if a.chk == Some(0) {
        return card::is_attack_above(d, c, 1000) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Make its ATK become reduced by 1000 instead
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(c:GetAttack()-1000)
        let atk = card::get_attack(d, c) as i64 - 1000;
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // return true
        return true;
    }
    // return false
    false
}
