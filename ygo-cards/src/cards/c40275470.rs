//! Jinzo - Energy Shocker
use super::prelude::*;

pub const ID: u32 = 40275470;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO, CARD_DARK_TIME_WIZARD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Jinzo" while in the field or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_JINZO)
    effect::set_value(d, e1, CARD_JINZO as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: You can destroy as many Traps your opponent controls as possible (if a card is Set, reveal it), then this card gains 300 ATK for each card destroyed this way
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetTarget(s.destg)
    effect::set_target(d, e2a, Some(cost_target(destg)));
    // e2a:SetOperation(s.desop)
    effect::set_operation(d, e2a, Some(operation(desop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Your opponent cannot activate Trap Cards or effects while you have another monster in your field or GY that mentions "Dark Time Wizard"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.cannotactcon)
    effect::set_condition(d, e3, Some(condition(cannotactcon)));
    // e3:SetValue(function(e,re,tp) return re:IsTrapEffect() end)
    effect::set_value_fn(d, e3, lua_fn(value_fn));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// e3:SetValue(function(e,re,tp) return re:IsTrapEffect() end)
fn value_fn(d: &mut Duel, a: Args) -> bool {
    // re:IsTrapEffect()
    effect::is_trap_effect(d, a.e(1))
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsTrap() and c:IsFaceup()) or (c:IsSpellTrap() and c:IsFacedown())
    (card::is_trap(d, c) && card::is_faceup(d, c)) || (card::is_spell_trap(d, c) && card::is_facedown(d, c))
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsTrap),tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_trap(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local fdg=g:Filter(Card.IsFacedown,nil)
        let fdg = group::filter(d, &g, &|d, c| card::is_facedown(d, c), NO_EX);
        // if #fdg>0 then Duel.ConfirmCards(tp,fdg) end
        if !fdg.is_empty() {
            duel::confirm_cards(d, tp, fdg);
        }
        // local traps=g:Filter(Card.IsTrap,nil)
        let traps = group::filter(d, &g, &|d, c| card::is_trap(d, c), NO_EX);
        // if #traps==0 then return end
        if traps.is_empty() {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local destroy_count=Duel.Destroy(traps,REASON_EFFECT)
        let destroy_count = duel::destroy(d, traps, REASON_EFFECT, None, None);
        // if destroy_count>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
        if destroy_count > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // This card gains 300 ATK for each card destroyed this way
            // c:UpdateAttack(300*destroy_count)
            card::update_attack(d, c, 300 * destroy_count as i64, None, None, None);
        }
    }
}
// function s.cannotactconfilter(c)
fn cannotactconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_DARK_TIME_WIZARD) and c:IsMonster() and c:IsFaceup()
    card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.cannotactcon(e)
fn cannotactcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cannotactconfilter,e:GetHandlerPlayer(),LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler())
    let hp = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cannotactconfilter), hp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into())
}
