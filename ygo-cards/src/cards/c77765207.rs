//! Emblema Oath
use super::prelude::*;

pub const ID: u32 = 77765207;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CENTUR_ION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.setfilter(c,ft)`
fn setfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsSpellTrap() and c:IsSSetable() and (ft>0 or c:IsFieldSpell())
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None) && (ft > 0 || card::is_field_spell(d, c))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // local b1=ft>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = ft > 0 && duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,ft)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, ft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then e:SetCategory(0)
    if op == 1 {
        effect::set_category(d, e, 0);
    // elseif op==2 then e:SetCategory(CATEGORY_SET)
    } else if op == 2 {
        effect::set_category(d, e, CATEGORY_SET);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if op==1 then
    if op == 1 {
        // Place 1 "Centurion" monster from your Deck to your Spell & Trap Zone as a Continuous Trap
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if let Some(sc) = sc {
            if duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
                // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
                card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
                // local c=e:GetHandler()
                let c = effect::get_handler(d, e).nil();
                // Treat it as a Continuous Trap
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_TYPE)
                effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                // e1:SetValue(TYPE_TRAP|TYPE_CONTINUOUS)
                effect::set_value(d, e1, (TYPE_TRAP | TYPE_CONTINUOUS) as i64);
                // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
                effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
                // Cannot Special Summon from the Extra Deck, except "Centurion" monsters
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e2, aux::stringid(ID, 3));
                // e2:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e2, EFFECT_TYPE_FIELD);
                // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
                // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e2:SetTargetRange(1,0)
                effect::set_target_range(d, e2, 1, 0);
                // e2:SetCondition(function() return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_ONFIELD,0,1,nil,sc:GetOriginalCodeRule()) end)
                effect::set_condition(d, e2, Some(lua_fn(move |d, _a: Args| {
                    let code = card::get_original_code_rule(d, sc);
                    duel::is_existing_matching_card(d, Some(&|d, x| spconfilter(d, x, code[0])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
                })));
                // e2:SetTarget(function(_e,_c) return not _c:IsSetCard(SET_CENTUR_ION) and _c:IsLocation(LOCATION_EXTRA) end)
                effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && card::is_location(d, c, LOCATION_EXTRA))));
                // e2:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e2,tp)
                duel::register_effect(d, e2, tp);
                // Clock Lizard check
                // aux.addTempLizardCheck(c,tp,function(_e,_c) return not _c:IsOriginalSetCard(SET_CENTUR_ION) end)
                aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_set_card(d, c, &[SET_CENTUR_ION as u16]))), None, None, None, None);
            }
        }
    // elseif op==2 then
    } else if op == 2 {
        // Set 1 "Centurion" Spell/Trap directly from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,ft)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c, ft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g.clone(), None, true);
        }
    }
}
/// `s.spconfilter(c,code)`
fn spconfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and (c:HasFlagEffect(id) or c:IsOriginalCodeRule(code))
    card::is_faceup(d, c) && (card::has_flag_effect(d, c, ID, None) || card::is_original_code_rule(d, c, &[code]))
}
