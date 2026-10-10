//! Eradicator Epidemic Virus
use super::prelude::*;

pub const ID: u32 = 54974237;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at your opponent's hand, all Spells/Traps they control, and all cards they draw until the 3rd end of their turn, and destroy all cards of that declared type
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_TOHAND|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND | TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:GetAttack()>=2500
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::get_attack(d, c) >= 2500
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // local decl_type=2<<Duel.SelectOption(tp,DECLTYPE_SPELL,DECLTYPE_TRAP)
    let decl_type = 2i64 << duel::select_option(d, a.tp, true, &[DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    // e:SetLabel(decl_type)
    effect::set_label(d, a.e, &[decl_type]);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,decl_type),tp,0,LOCATION_ONFIELD,nil)
    let dt = decl_type as u32;
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, dt)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.cffilter(c)
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND) or (c:IsFacedown() and c:IsSpellTrap())
    card::is_location(d, c, LOCATION_HAND) || (card::is_facedown(d, c) && card::is_spell_trap(d, c))
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local decl_type=e:GetLabel()
    let decl_type = effect::get_label(d, a.e) as u32;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD|LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // local cg=g:Filter(s.cffilter,nil)
        let cg = group::filter(d, &g, &cffilter, NO_EX);
        // Duel.ConfirmCards(tp,cg)
        duel::confirm_cards(d, a.tp, cg);
        // local dg=g:Filter(Card.IsType,nil,decl_type)
        let dg = group::filter(d, &g, &|d, x| card::is_type(d, x, decl_type), NO_EX);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
    // Look at all cards your opponent draws until the 3rd end of their turn, and destroy all cards of that declared type
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetLabel(decl_type)
    effect::set_label(d, e1, &[decl_type as i64]);
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.turncon)
    effect::set_condition(d, e2, Some(condition(turncon)));
    // e2:SetOperation(s.turnop)
    effect::set_operation(d, e2, Some(operation(turnop)));
    // e2:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // local descnum=tp==c:GetOwner() and 0 or 1
    let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetDescription(aux.Stringid(id,descnum))
    effect::set_description(d, e3, aux::stringid(ID, descnum));
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetCode(1082946)
    effect::set_code(d, e3, 1082946);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e3, a.tp);
    // e3:SetOperation(s.reset)
    effect::set_operation(d, e3, Some(operation(reset)));
    // e3:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(e:GetHandler())
    let e4 = effect::create_effect(d, c);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
}
// function s.reset(e,tp,eg,ep,ev,re,r,rp)
fn reset(d: &mut Duel, a: &EffArgs) {
    // s.turnop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    turnop_impl(d, lo, Some(a.e));
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if ep==e:GetOwnerPlayer() then return end
    if a.ep == effect::get_owner_player(d, a.e) {
        return;
    }
    // local hg=eg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let eg = a.eg.clone().nil();
    let hg = group::filter(d, &eg, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-ep,hg)
    duel::confirm_cards(d, 1 - a.ep, hg.clone());
    // local dg=hg:Filter(Card.IsType,nil,e:GetLabel())
    let label = effect::get_label(d, a.e) as u32;
    let dg = group::filter(d, &hg, &|d, x| card::is_type(d, x, label), NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
    // Duel.ShuffleHand(ep)
    duel::shuffle_hand(d, a.ep);
}
// function s.turncon(e,tp,eg,ep,ev,re,r,rp)
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.turnop(e,tp,eg,ep,ev,re,r,rp)
fn turnop(d: &mut Duel, a: &EffArgs) {
    turnop_impl(d, a.e, a.re);
}
// The body of s.turnop; `re` is the reset effect when it is called from s.reset.
fn turnop_impl(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    // local ct=e:GetLabel()+1
    let ct = effect::get_label(d, e) + 1;
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct]);
    // e:GetHandler():SetTurnCounter(ct)
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, ct as u16);
    // if ct==3 then
    if ct == 3 {
        // e:GetLabelObject():Reset()
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::reset(d, lo);
        // if re then re:Reset() end
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
