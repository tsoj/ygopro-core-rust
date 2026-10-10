//! Penguin Sword
//! Scripted by Neo Yuno
use super::prelude::*;

pub const ID: u32 = 69792699;

// s.listed_series={SET_PENGUIN}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PENGUIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_PENGUIN))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_PENGUIN as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(800)
    effect::set_value(d, e2, 800);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.retcon)
    effect::set_condition(d, e3, Some(condition(retcon)));
    // e3:SetTarget(s.rettg)
    effect::set_target(d, e3, Some(cost_target(rettg)));
    // e3:SetOperation(s.retop)
    effect::set_operation(d, e3, Some(operation(retop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e4, EVENT_TO_HAND);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.negtg)
    effect::set_target(d, e4, Some(cost_target(negtg)));
    // e4:SetOperation(s.negop)
    effect::set_operation(d, e4, Some(operation(negop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_REMOVE)
    effect::set_code(d, e5, EVENT_REMOVE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    if a.ep == a.tp {
        return false;
    }
    let first = group::get_first(d, &a.eg.clone().nil());
    let h = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, h)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD as u32, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_ONFIELD as i32);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD as u32, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.cfilter(c,tp,re)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, re: Option<EffectId>) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD) and c:GetPreviousControler()==1-tp
    //     and c:IsReason(REASON_EFFECT) and re:GetHandler():IsSetCard(SET_PENGUIN)
    if !(card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::get_previous_controler(d, c) == 1 - tp
        && card::is_reason(d, c, REASON_EFFECT))
    {
        return false;
    }
    let h = effect::get_handler(d, re.nil()).nil();
    card::is_set_card(d, h, &[SET_PENGUIN as u16])
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re;
    // if chk==0 then return eg and eg:IsExists(s.cfilter,1,nil,tp,re) end
    if a.chk == Some(0) {
        return match a.eg.clone() {
            Some(eg) => group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, re), 1, NO_EX),
            None => false,
        };
    }
    // local g=eg:Filter(s.cfilter,nil,tp,re)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| cfilter(d, c, tp, re), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsOriginalCodeRule(e:GetLabel())
    let label = effect::get_label(d, e);
    card::is_original_code_rule(d, c, &[label as u32])
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsOriginalCodeRule(e:GetLabel())
    let label = effect::get_label(d, a.e);
    let h = effect::get_handler(d, a.re.nil()).nil();
    card::is_original_code_rule(d, h, &[label as u32])
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local code=tc:GetOriginalCode()
        let code = card::get_original_code(d, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
        effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
        // e1:SetLabel(code)
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetTarget(s.distg)
        effect::set_target(d, e1, Some(effect_card_fn(distg)));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
        // e2:SetLabel(code)
        effect::set_label(d, e2, &[code as i64]);
        // e2:SetCondition(s.discon)
        effect::set_condition(d, e2, Some(condition(discon)));
        // e2:SetOperation(s.disop)
        effect::set_operation(d, e2, Some(operation(disop)));
        // e2:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
        effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
        // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
        effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
    }
}
