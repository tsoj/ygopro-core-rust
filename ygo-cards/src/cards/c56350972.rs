//! 竜姫神サフィラ
//! Saffira, Queen of Dragons
use super::prelude::*;

pub const ID: u32 = 56350972;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[80566312], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCondition(s.regcon1)
    effect::set_condition(d, e1, Some(condition(regcon1)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCondition(s.regcon2)
    effect::set_condition(d, e2, Some(condition(regcon2)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regcon1(e,tp,eg,ep,ev,re,r,rp)`
fn regcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRitualSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
/// `s.regfilter(c)`
fn regfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsPreviousLocation(LOCATION_HAND|LOCATION_DECK)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_DECK)
}
/// `s.regcon2(e,tp,eg,ep,ev,re,r,rp)`
fn regcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.regfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| regfilter(d, c), 1, NO_EX)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsPlayerCanDraw(tp,2)
    let b1 = duel::is_player_can_draw(d, tp, 2);
    // local b2=Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)~=0
    let b2 = duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) != 0;
    // local b3=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local ops={}
    let mut ops: Vec<u64> = Vec::new();
    // local opval={}
    // (opval[off-1] in Lua; index k holds the value of the k-th added option, 0-based)
    let mut opval: Vec<i64> = Vec::new();
    // local off=1
    // if b1 then ops[off]=aux.Stringid(id,1) opval[off-1]=1 off=off+1 end
    if b1 {
        ops.push(aux::stringid(ID, 1));
        opval.push(1);
    }
    // if b2 then ops[off]=aux.Stringid(id,2) opval[off-1]=2 off=off+1 end
    if b2 {
        ops.push(aux::stringid(ID, 2));
        opval.push(2);
    }
    // if b3 then ops[off]=aux.Stringid(id,3) opval[off-1]=3 off=off+1 end
    if b3 {
        ops.push(aux::stringid(ID, 3));
        opval.push(3);
    }
    // local op=Duel.SelectOption(tp,table.unpack(ops))
    // (the API returns the 0-based index, Lua's op is 1-based)
    let op = duel::select_option(d, tp, true, &ops) as usize + 1;
    // local sel=opval[op]
    // (a missing entry is nil in Lua; SetLabel(nil) reads as 0)
    let sel = opval.get(op).copied().unwrap_or(0);
    // e:SetLabel(sel)
    effect::set_label(d, a.e, &[sel]);
    if sel == 1 {
        // e:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
        effect::set_category(d, a.e, CATEGORY_DRAW + CATEGORY_HANDES);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,0,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    } else if sel == 2 {
        // e:SetCategory(CATEGORY_HANDES)
        effect::set_category(d, a.e, CATEGORY_HANDES);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,1-tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    } else {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, a.e);
    if sel == 1 {
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    } else if sel == 2 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sg=g:RandomSelect(tp,1)
        let sg = group::random_select(d, &g, tp, 1);
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
