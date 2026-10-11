//! Sacred Scrolls of the Gizmek Legend
use super::prelude::*;

pub const ID: u32 = 24793135;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x206)
    card::enable_counter_permit(d, c, 0x206, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.exctg)
    effect::set_target(d, e2, Some(cost_target(exctg)));
    // e2:SetOperation(s.excop)
    effect::set_operation(d, e2, Some(operation(excop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place counter
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.countcon)
    effect::set_condition(d, e3, Some(condition(countcon)));
    // e3:SetOperation(s.countop)
    effect::set_operation(d, e3, Some(operation(countop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Cannot activate
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e5, EFFECT_CANNOT_ACTIVATE);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetTargetRange(1,1)
    effect::set_target_range(d, e5, 1, 1);
    // e5:SetCondition(s.limitcon)
    effect::set_condition(d, e5, Some(condition(limitcon)));
    // e5:SetValue(s.aclimit)
    effect::set_value_fn(d, e5, lua_fn(aclimit));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.exctg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>2 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.excfilter(c)
fn excfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsDefense(c:GetAttack()) and c:IsAbleToHand()
    card::is_race(d, c, RACE_MACHINE as u64) && defense_is_attack(d, c) && card::is_able_to_hand(d, c, None)
}
/// `c:IsDefense(c:GetAttack())`
fn defense_is_attack(d: &mut Duel, c: CardId) -> bool {
    let atk = card::get_attack(d, c) as u32;
    card::is_defense(d, c, &[atk])
}
// function s.excop(e,tp,eg,ep,ev,re,r,rp)
fn excop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, tp, 3);
    // local g=Duel.GetDecktopGroup(tp,3)
    let mut g = duel::get_decktop_group(d, tp, 3);
    // local hg=g:Filter(s.excfilter,nil)
    let hg = group::filter(d, &g, &excfilter, NO_EX);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if #g>0 then
    if !g.is_empty() {
        // if #hg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !hg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=hg:Select(tp,1,1,nil)
            let sg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
            // if sg:GetFirst():IsAbleToHand() then
            let first = group::get_first(d, &sg).nil();
            if card::is_able_to_hand(d, first, None) {
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg.clone());
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, tp);
                // g:RemoveCard(sg:GetFirst())
                let first = group::get_first(d, &sg).nil();
                group::remove_card(&mut g, &CardSet::from_card(first));
            } else {
                // Duel.SendtoGrave(sg,REASON_RULE)
                duel::send_to_grave(d, sg, REASON_RULE, None, None);
            }
        }
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsDefense(c:GetAttack())
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE as u64) && defense_is_attack(d, c)
}
// function s.countcon(e,tp,eg,ep,ev,re,r,rp)
fn countcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
// function s.countop(e,tp,eg,ep,ev,re,r,rp)
fn countop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x206,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x206, 1, false);
}
// function s.limitcon(e,tp,eg,ep,ev,re,r,rp)
fn limitcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x206)>=10
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x206) >= 10
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // local loc=re:GetActivateLocation()
    let loc = effect::get_activate_location(d, re);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return loc==LOCATION_MZONE and re:IsMonsterEffect() and rc:IsDefenseAbove(0) and not rc:IsDefense(rc:GetAttack())
    loc == LOCATION_MZONE
        && effect::is_monster_effect(d, re)
        && card::is_defense_above(d, rc, 0)
        && !defense_is_attack(d, rc)
}
