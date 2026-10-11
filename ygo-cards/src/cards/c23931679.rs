//! Ocean Dragon Lord - Kairyu-Shin
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 23931679;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], listed_series: &[SET_KAIRYU_SHIN, SET_SEA_STEALTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Adjust
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EVENT_ADJUST)
    effect::set_code(d, e1, EVENT_ADJUST);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.adjustop)
    effect::set_operation(d, e1, Some(operation(adjustop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot Normal/Flip/Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetCondition(s.umicon)
    effect::set_condition(d, e2, Some(condition(umicon)));
    // e2:SetTarget(s.sumlimit)
    effect::set_target(d, e2, Some(lua_fn(sumlimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e4, EFFECT_FORCE_SPSUMMON_POSITION);
    // e4:SetValue(POS_FACEDOWN)
    effect::set_value(d, e4, POS_FACEDOWN as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Search 1 "Umi", or 1 "Kairyu-Shin"/"Sea Stealth" Spell/Trap
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Cannot activate a monster's effect that flips itself face-up as cost if it would violate the "each player can only control 1 face-up non-WATER monster" clause
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(1,1)
    effect::set_target_range(d, e6, 1, 1);
    // e6:SetValue(function(e,re,tp) ... end)
    effect::set_value_fn(d, e6, lua_fn(cannot_activate_value));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `function(e,re,tp)` of `e6:SetValue`
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let tp = a.p(2);
    let rc = effect::get_handler(d, re).nil();
    // return re:HasSelfChangePositionCost() and rc:IsAttributeExcept(ATTRIBUTE_WATER)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttributeExcept,ATTRIBUTE_WATER),tp,LOCATION_MZONE,0,1,nil)
    effect::has_self_change_position_cost(d, re)
        && card::is_attribute_except(d, rc, ATTRIBUTE_WATER, None, 0, None)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.umicon(e,tp,eg,ep,ev,re,r,rp)`
fn umicon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_UMI),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     or Duel.IsEnvironment(CARD_UMI)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_UMI]))), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        || duel::is_environment(d, CARD_UMI, None, None)
}
/// `s.nonwaterfilter(c)`
fn nonwaterfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttributeExcept(ATTRIBUTE_WATER)
    card::is_faceup(d, c) && card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None)
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local flag0=c:HasFlagEffect(id)
    let flag0 = card::has_flag_effect(d, c, ID, None);
    // local flag1=c:HasFlagEffect(id+1)
    let flag1 = card::has_flag_effect(d, c, ID + 1, None);
    // if not s.umicon(e,tp,eg,ep,ev,re,r,rp) then
    if !umicon(d, a) {
        // if flag0 then c:ResetFlagEffect(id) end
        if flag0 {
            card::reset_flag_effect(d, c, ID);
        }
        // if flag1 then c:ResetFlagEffect(id+1) end
        if flag1 {
            card::reset_flag_effect(d, c, ID + 1);
        }
        // return
        return;
    }
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local g0=Duel.GetMatchingGroup(s.nonwaterfilter,tp,LOCATION_MZONE,0,nil)
    let mut g0 = duel::get_matching_group(d, Some(&nonwaterfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=Duel.GetMatchingGroup(s.nonwaterfilter,tp,0,LOCATION_MZONE,nil)
    let mut g1 = duel::get_matching_group(d, Some(&nonwaterfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local ct0=#g0
    let ct0 = g0.len();
    // local ct1=#g1
    let ct1 = g1.len();
    // if ct0==0 and flag0 then
    if ct0 == 0 && flag0 {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
    } else if ct0 > 0 {
        // if flag0 and ct0>1 then
        if flag0 && ct0 > 1 {
            // local fid=c:GetFlagEffectLabel(id)
            let fid = card::get_flag_effect_label(d, c, ID).first().copied().unwrap_or(0);
            // g0:Match(s.fidfilter,nil,fid)
            group::match_(d, &mut g0, &|d, x| card::get_field_id(d, x) as i64 != fid, NO_EX);
        } else if flag0 && ct0 == 1 {
            // g0:Clear()
            g0 = CardSet::new();
        } else if !flag0 {
            // local tc=g0:GetFirst()
            let mut tc = group::get_first(d, &g0);
            // if ct0>1 then
            if ct0 > 1 {
                // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
                // tc=g0:Select(tp,1,1,nil):GetFirst()
                let sel = group::select(d, &g0, a.tp, 1, 1, false, NO_EX).nil();
                tc = group::get_first(d, &sel);
                // Duel.HintSelection(tc,true)
                duel::hint_selection(d, tc.nil(), true);
            }
            // g0:RemoveCard(tc)
            group::remove_card(&mut g0, &CardSet::from_card(tc.nil()));
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,1,tc:GetFieldID())
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, card::get_field_id(d, tc.nil()) as i64, 0);
        }
    }
    // if ct1==0 and flag1 then
    if ct1 == 0 && flag1 {
        // c:ResetFlagEffect(id+1)
        card::reset_flag_effect(d, c, ID + 1);
    } else if ct1 > 0 {
        // if flag1 and ct1>1 then
        if flag1 && ct1 > 1 {
            // local fid=c:GetFlagEffectLabel(id+1)
            let fid = card::get_flag_effect_label(d, c, ID + 1).first().copied().unwrap_or(0);
            // g1:Match(s.fidfilter,nil,fid)
            group::match_(d, &mut g1, &|d, x| card::get_field_id(d, x) as i64 != fid, NO_EX);
        } else if flag1 && ct1 == 1 {
            // g1:Clear()
            g1 = CardSet::new();
        } else if !flag1 {
            // local tc=g1:GetFirst()
            let mut tc = group::get_first(d, &g1);
            // if ct1>1 then
            if ct1 > 1 {
                // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
                duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, aux::stringid(ID, 1));
                // tc=g1:Select(1-tp,1,1,nil):GetFirst()
                let sel = group::select(d, &g1, 1 - a.tp, 1, 1, false, NO_EX).nil();
                tc = group::get_first(d, &sel);
                // Duel.HintSelection(tc,true)
                duel::hint_selection(d, tc.nil(), true);
            }
            // g1:RemoveCard(tc)
            group::remove_card(&mut g1, &CardSet::from_card(tc.nil()));
            // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD_DISABLE,0,1,tc:GetFieldID())
            card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, card::get_field_id(d, tc.nil()) as i64, 0);
        }
    }
    // local readjust=false
    let mut readjust = false;
    // if #g0>0 then
    if !g0.is_empty() {
        // Duel.SendtoGrave(g0,REASON_RULE,PLAYER_NONE,tp)
        duel::send_to_grave(d, g0, REASON_RULE, Some(PLAYER_NONE as u8), Some(a.tp));
        // readjust=true
        readjust = true;
    }
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
        // readjust=true
        readjust = true;
    }
    // if readjust then Duel.Readjust() end
    if readjust {
        duel::readjust(d);
    }
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // if c:IsAttribute(ATTRIBUTE_WATER) then return false end
    let c = a.c(1);
    if card::is_attribute(d, c, ATTRIBUTE_WATER) {
        return false;
    }
    // return Duel.IsExistingMatchingCard(s.nonwaterfilter,targetp or sump,LOCATION_MZONE,0,1,nil)
    let sump = a.p(2);
    let targetp = match a.v(5) {
        Value::Nil => sump,
        v => v.int() as u8,
    };
    duel::is_existing_matching_card(d, Some(&nonwaterfilter), targetp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and (c:IsCode(CARD_UMI) or (c:IsSetCard({SET_KAIRYU_SHIN,SET_SEA_STEALTH}) and c:IsSpellTrap()))
    card::is_able_to_hand(d, c, None)
        && (card::is_code(d, c, &[CARD_UMI]) || (card::is_set_card(d, c, &[SET_KAIRYU_SHIN as u16, SET_SEA_STEALTH as u16]) && card::is_spell_trap(d, c)))
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
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
