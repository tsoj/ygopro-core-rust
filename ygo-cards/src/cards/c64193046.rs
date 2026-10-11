//! Superheavy Samurai Brave Masurawo
use super::prelude::*;

pub const ID: u32 = 64193046;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Attack while in defense position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DEFENSE_ATTACK)
    effect::set_code(d, e1, EFFECT_DEFENSE_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw cards up to 3 cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_SUPERHEAVY_SAMURAI)
    //   and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SUPERHEAVY_SAMURAI) and c:IsDestructable(e)
    //   and not c:IsStatus(STATUS_DESTROY_CONFIRMED|STATUS_BATTLE_DESTROYED)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //   and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,e) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, e)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| desfilter(d, x, e)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // e:SetLabelObject(tc)
        effect::set_label_object(d, e, Value::Card(tc));
        // tc:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let tp = effect::get_handler_player(d, e);
    repfilter(d, c, tp)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsSpellTrapEffect()
    a.rp == 1 - a.tp && effect::is_spell_trap_effect(d, a.re.nil())
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let h = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    // if chk==0 then return h<3 and Duel.IsPlayerCanDraw(tp,3-h) end
    if a.chk == Some(0) {
        return h < 3 && duel::is_player_can_draw(d, a.tp, (3 - h) as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(3-h)
    duel::set_target_param(d, (3 - h) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,3-h)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 3 - h);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local h=Duel.GetFieldGroupCount(p,LOCATION_HAND,0)
    let h = duel::get_field_group_count(d, p, LOCATION_HAND, 0);
    // if h>=3 then return end
    if h >= 3 {
        return;
    }
    // Duel.Draw(p,3-h,REASON_EFFECT)
    duel::draw(d, p, (3 - h) as u32, REASON_EFFECT);
}
