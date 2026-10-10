//! Gilti-Gearfried the Magical Steel Knight
use super::prelude::*;

pub const ID: u32 = 49161188;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion summon procedure
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // If fusion summoned using only monsters on the field, can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Gain ATK equal to half of its DEF
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate card/effect that targets this card, and if you do, destroy 1 card on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e4, CATEGORY_DISABLE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// s.ffilter(c,fc,sumtype,tp,sub,mg,sg)
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsRace(RACE_WARRIOR,fc,sumtype,tp) and c:GetAttribute(fc,sumtype,tp)~=0
    //     and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetAttribute(fc,sumtype,tp),fc,sumtype,tp))
    if !card::is_race_ctx(d, c, RACE_WARRIOR, Some(a.fc), a.sumtype, Some(a.tp)) {
        return false;
    }
    let attr = card::get_attribute_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp));
    if attr == 0 {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => !group::is_exists(d, sg, &|d, x| fusfilter(d, x, attr, a.fc, a.sumtype, a.tp), 1, c.into()),
    }
}
/// s.fusfilter(c,attr,fc,sumtype,tp)
fn fusfilter(d: &mut Duel, c: CardId, attr: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(attr,fc,sumtype,tp) and not c:IsHasEffect(511002961)
    card::is_attribute_ctx(d, c, attr, Some(fc), sumtype, Some(tp)) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()~=0 then
    if effect::get_label(d, a.e) != 0 {
        // Can make a second attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) {
    // local c=e:GetHandler()
    let e = a.e(0);
    let c = effect::get_handler(d, e).nil();
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local lg=g:Filter(Card.IsLocation,nil,LOCATION_MZONE)
    let lg = group::filter(d, &g, &|d, x| card::is_location(d, x, LOCATION_MZONE), NO_EX);
    // if #g==#lg then
    if g.len() == lg.len() {
        // e:GetLabelObject():SetLabel(id)
        let e1 = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, e1, &[ID as i64]);
    }
}
/// s.costfil(c)
fn costfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToRemoveAsCost()
    card::is_spell(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfil,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfil), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfil,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfil), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc and bc:IsFaceup() and bc:IsRelateToBattle()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    match bc {
        None => false,
        Some(bc) => card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc),
    }
}
/// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local value=c:GetDefense()
    let mut value = card::get_defense(d, c);
    // if value<0 then value=0 end
    if value < 0 {
        value = 0;
    }
    // if c:IsRelateToEffect(e) and c:IsFaceup() and value>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && value > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(value/2)
        effect::set_value(d, e1, lua_int(value as f64 / 2.0));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local loc,tg=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TARGET_CARDS]);
    let loc = info[0].int();
    // if not tg or not tg:IsContains(c) then return false end
    let Some(tg) = info[1].as_group().cloned() else {
        return false;
    };
    if !group::is_contains(&tg, c) {
        return false;
    }
    // return Duel.IsChainDisablable(ev) and loc~=LOCATION_DECK
    duel::is_chain_disablable(d, a.ev as u8) && loc != LOCATION_DECK as i64
}
/// s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateEffect(ev) then return end
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
