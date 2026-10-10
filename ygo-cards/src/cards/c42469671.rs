//! Thunderforce Attack
use super::prelude::*;

pub const ID: u32 = 42469671;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SLIFER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_INACTIVATE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsOriginalCodeRule,CARD_SLIFER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_original_code_rule(d, c, &[CARD_SLIFER])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    // if Duel.IsMainPhase() and Duel.IsTurnPlayer(tp) then
    if duel::is_main_phase(d, None) && duel::is_turn_player(d, a.tp) {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_DRAW);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    } else {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
    }
    true
}
/// `s.filter(c,p)`
fn filter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:IsControler(p) and c:IsLocation(LOCATION_GRAVE) and c:IsMonster()
    card::is_controler(d, c, p) && card::is_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
    // local ct=Duel.GetOperatedGroup():FilterCount(s.filter,nil,1-tp)
    let og = duel::get_operated_group(d);
    let ct = group::filter_count(d, &og, &|d, c| filter(d, c, 1 - a.tp), NO_EX);
    // if Duel.IsMainPhase() and Duel.IsTurnPlayer(tp) and ct>0
    //     and Duel.IsPlayerCanDraw(tp,ct) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_main_phase(d, None)
        && duel::is_turn_player(d, a.tp)
        && ct > 0
        && duel::is_player_can_draw(d, a.tp, ct as u32)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetCondition(s.atkcon)
        effect::set_condition(d, e1, Some(condition(atkcon)));
        // e1:SetTarget(s.atktg)
        effect::set_target(d, e1, Some(effect_card_fn(atktg)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
        // e2:SetOperation(s.checkop)
        effect::set_operation(d, e2, Some(operation(checkop)));
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) > 0
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()~=e:GetLabel()
    card::get_field_id(d, c) as i64 != effect::get_label(d, e)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local a=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let aa = group::get_first(d, &eg).nil();
    // if Duel.GetFlagEffect(tp,id)>0 or not a:IsControler(tp) then return end
    if duel::get_flag_effect(d, a.tp, ID) > 0 || !card::is_controler(d, aa, a.tp) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // e:GetLabelObject():SetLabel(a:GetFieldID())
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let fid = card::get_field_id(d, aa) as i64;
    effect::set_label(d, lo, &[fid]);
}
