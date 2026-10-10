//! Triple Tactics Thrust
use super::prelude::*;

pub const ID: u32 = 35269904;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 Normal Spell/Trap
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.setcon)
    effect::set_condition(d, e1, Some(condition(setcon)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,function(re) return not re:IsMonsterEffect() end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(counter_filter));
}
/// `function(re) return not re:IsMonsterEffect() end`
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    !effect::is_monster_effect(d, a.e(0))
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,1-tp,ACTIVITY_CHAIN)>0
    duel::get_custom_activity_count(d, ID, 1 - a.tp, ACTIVITY_CHAIN) > 0
}
/// `s.stfilter(c,set_chk,tohand_chk)`
fn stfilter(d: &mut Duel, c: CardId, set_chk: bool, tohand_chk: bool) -> bool {
    // return c:IsNormalSpellTrap() and not c:IsCode(id)
    //   and ((tohand_chk and c:IsAbleToHand()) or (set_chk and c:IsSSetable()))
    card::is_normal_spell_trap(d, c)
        && !card::is_code(d, c, &[ID])
        && ((tohand_chk && card::is_able_to_hand(d, c, None)) || (set_chk && card::is_s_setable(d, c, false, None)))
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // local set_chk=ft>0
    let set_chk = ft > 0;
    // local tohand_chk=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    let tohand_chk = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.stfilter,tp,LOCATION_DECK,0,1,nil,set_chk,tohand_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| stfilter(d, c, set_chk, tohand_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local set_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let set_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // local tohand_chk=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    let tohand_chk = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0;
    // if not set_chk and not tohand_chk then return end
    if !set_chk && !tohand_chk {
        return;
    }
    // local desc=aux.Stringid(id,1)
    let mut desc = aux::stringid(ID, 1);
    // if not set_chk then desc=HINTMSG_ATOHAND
    if !set_chk {
        desc = HINTMSG_ATOHAND as u64;
    // elseif not tohand_chk then desc=HINTMSG_SET
    } else if !tohand_chk {
        desc = HINTMSG_SET as u64;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,desc)
    duel::hint(d, HINT_SELECTMSG as u8, tp, desc);
    // local sc=Duel.SelectMatchingCard(tp,s.stfilter,tp,LOCATION_DECK,0,1,1,nil,set_chk,tohand_chk):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| stfilter(d, c, set_chk, tohand_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // if tohand_chk and sc:IsAbleToHand()
    //   and (not set_chk or not sc:IsSSetable() or Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    if tohand_chk
        && card::is_able_to_hand(d, sc, None)
        && (!set_chk || !card::is_s_setable(d, sc, false, None) || duel::select_yes_no(d, tp, aux::stringid(ID, 2)))
    {
        // if Duel.SendtoHand(sc,nil,REASON_EFFECT)==0 then return end
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    // elseif Duel.SSet(tp,sc)>0 then
    } else if duel::s_set(d, tp, CardSet::from_card(sc), None, true) > 0 {
        // Cannot be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_CANNOT_ACT|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_CANNOT_ACT | RESET_PHASE | PHASE_END, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
