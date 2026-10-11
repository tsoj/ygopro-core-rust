//! D - Force
use super::prelude::*;

pub const ID: u32 = 6186304;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[83965310], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e0, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetTarget(s.target)
    effect::set_target(d, e0, Some(cost_target(target)));
    // e0:SetOperation(s.activate)
    effect::set_operation(d, e0, Some(operation(activate)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_DRAW)
    effect::set_code(d, e1, EFFECT_CANNOT_DRAW);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCondition(aux.AND(s.plasmacon,s.drawcon))
    effect::set_condition(d, e1, Some(condition(|d, a| plasmacon(d, a) && drawcon(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_DRAW_COUNT)
    effect::set_code(d, e2, EFFECT_DRAW_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCondition(s.plasmacon)
    effect::set_condition(d, e2, Some(condition(plasmacon)));
    // e2:SetValue(0)
    effect::set_value(d, e2, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your cards cannot be targeted
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e3, LOCATION_ONFIELD, 0);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetCondition(s.plasmacon)
    effect::set_condition(d, e3, Some(condition(plasmacon)));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // ATK up,effect indestructible,and double attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetCondition(s.plasmacon)
    effect::set_condition(d, e4, Some(condition(plasmacon)));
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsCode,83965310))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[83965310]))));
    // e4:SetValue(s.val)
    effect::set_value_fn(d, e4, effect_card_value(val));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // e5:SetValue(aux.indoval)
    effect::set_value_fn(d, e5, aux::indoval());
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e6, EFFECT_EXTRA_ATTACK);
    // e6:SetValue(1)
    effect::set_value(d, e6, 1);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(83965310) and c:IsAbleToHand()
    card::is_code(d, c, &[83965310]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if g.len() > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
/// `s.plasmacon(e)`
fn plasmacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,83965310),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[83965310]))), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.drawcon(e)`
fn drawcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DRAW)
    duel::is_phase(d, PHASE_DRAW)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,0,LOCATION_GRAVE,LOCATION_GRAVE,nil)*100
    duel::get_matching_group_count(d, Some(&|d, c| card::is_monster(d, c)), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64 * 100
}
