//! Floowandereeze & Empen
//! ふわんだりぃず×えんぺん
//! scripted by XyLeN
use super::prelude::*;

pub const ID: u32 = 80611581;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOOWANDEREEZE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Search 1 "Flundereeze" Spell/Trap and Summon 1 monster from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.trsumcon)
    effect::set_condition(d, e1, Some(condition(trsumcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Cannot activate the monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.trsumcon)
    effect::set_condition(d, e2, Some(condition(trsumcon)));
    // e2:SetValue(s.actlmtval)
    effect::set_value_fn(d, e2, lua_fn(actlmtval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Halve the ATK/DEF of that monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.adcon)
    effect::set_condition(d, e3, Some(condition(adcon)));
    // e3:SetCost(s.adcost)
    effect::set_cost(d, e3, Some(cost_target(adcost)));
    // e3:SetOperation(s.adop)
    effect::set_operation(d, e3, Some(operation(adop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.trsumcon(e,tp,eg,ep,ev,re,r,rp)`
fn trsumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FLOOWANDEREEZE) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FLOOWANDEREEZE as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonable(true,nil)
    card::is_summonable(d, c, true, None, 0, Some(0x1f))
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // if not g:GetFirst():IsLocation(LOCATION_HAND) then return end
        let first = group::get_first(d, &g).nil();
        if !card::is_location(d, first, LOCATION_HAND) {
            return;
        }
        // local sg=Duel.GetMatchingGroup(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
        let sg = duel::get_matching_group(d, Some(&sumfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=sg:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.Summon(tp,sc,true,nil)
            duel::summon(d, tp, sc, true, None, 0, None);
        }
    }
}
/// `s.actlmtval(e,re,rp)`
fn actlmtval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsSpecialSummoned() and rc:IsAttackPos() and rc:IsLocation(LOCATION_MZONE)
    //     and re:IsMonsterEffect()
    card::is_special_summoned(d, rc) && card::is_attack_pos(d, rc) && card::is_location(d, rc, LOCATION_MZONE) && effect::is_monster_effect(d, re)
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattleTarget()~=nil
    let h = effect::get_handler(d, a.e).nil();
    card::get_battle_target(d, h).is_some()
}
/// `s.adcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn adcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,nil)
    //     and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEUP as u8))), tp, LOCATION_HAND, 0, 1, NO_EX)
            && !card::has_flag_effect(d, c, ID, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEUP as u8))), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // if bc:IsRelateToBattle() and bc:IsFaceup() then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(bc:GetAttack()/2)
        let atk = card::get_attack(d, bc);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(bc:GetDefense()/2)
        let def = card::get_defense(d, bc);
        effect::set_value(d, e2, lua_int(def as f64 / 2.0));
        // bc:RegisterEffect(e2)
        card::register_effect(d, bc, e2, false);
    }
}
