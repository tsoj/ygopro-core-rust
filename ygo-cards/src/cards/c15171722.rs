//! Evolsaur Lios
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 15171722;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[93504463, 74100225],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Evolutionary Bridge" or "Evo-Singularity" from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 FIRE Reptile or Dinosaur monster to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_LVCHANGE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_LVCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(condition(tgcon)));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Check if it's Special Summoned by a FIRE monster's effect
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3a, EVENT_SPSUMMON_SUCCESS);
    // e3a:SetOperation(s.regop)
    effect::set_operation(d, e3a, Some(operation(regop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(93504463,74100225) and c:IsSSetable()
    card::is_code(d, c, &[93504463, 74100225]) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, a.tp, g, None, true);
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsNormalSummoned() or (c:IsSpecialSummoned() and c:HasFlagEffect(id))
    card::is_normal_summoned(d, c) || (card::is_special_summoned(d, c) && card::has_flag_effect(d, c, ID, None))
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_REPTILE|RACE_DINOSAUR) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.cfilter(c,lv,rac)`
fn cfilter(d: &mut Duel, c: CardId, lv: u32, rac: u64) -> bool {
    // return c:IsFaceup() and c:HasLevel() and (not c:IsRace(rac) or not c:IsLevel(lv))
    card::is_faceup(d, c) && card::has_level(d, c) && (!card::is_race(d, c, rac) || !card::is_level(d, c, &[lv]))
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sel) {
        // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) then
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0 && card::is_location(d, sc, LOCATION_GRAVE) {
            // local lv=sc:GetLevel()
            let lv = card::get_level(d, sc);
            // local rac=sc:GetRace()
            let rac = card::get_race(d, sc);
            // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,lv,rac)
            let g = duel::get_matching_group(d, Some(&|d: &mut Duel, x| cfilter(d, x, lv, rac)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
            // if #g>=2 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if g.len() >= 2 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
                // local sg=g:FilterSelect(tp,s.cfilter,2,2,nil,lv,rac)
                let sg = group::filter_select(d, &g, a.tp, &|d: &mut Duel, x| cfilter(d, x, lv, rac), 2, 2, false, NO_EX).nil();
                // Duel.HintSelection(sg,true)
                duel::hint_selection(d, sg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // for tc in sg:Iter() do
                for tc in group::iter(d, &sg) {
                    // if not tc:IsLevel(lv) then
                    if !card::is_level(d, tc, &[lv]) {
                        // local e1=Effect.CreateEffect(c)
                        let e1 = effect::create_effect(d, c);
                        // e1:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                        // e1:SetCode(EFFECT_CHANGE_LEVEL)
                        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
                        // e1:SetValue(lv)
                        effect::set_value(d, e1, lv as i64);
                        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                        // tc:RegisterEffect(e1)
                        card::register_effect(d, tc, e1, false);
                    }
                    // if not tc:IsRace(rac) then
                    if !card::is_race(d, tc, rac) {
                        // local e2=Effect.CreateEffect(c)
                        let e2 = effect::create_effect(d, c);
                        // e2:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                        // e2:SetCode(EFFECT_CHANGE_RACE)
                        effect::set_code(d, e2, EFFECT_CHANGE_RACE);
                        // e2:SetValue(rac)
                        effect::set_value(d, e2, rac as i64);
                        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                        // tc:RegisterEffect(e2)
                        card::register_effect(d, tc, e2, false);
                    }
                }
            }
        }
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if re and re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_FIRE) then
    if let Some(re) = a.re {
        if effect::is_monster_effect(d, re) {
            let rh = effect::get_handler(d, re).nil();
            if card::is_attribute(d, rh, ATTRIBUTE_FIRE) {
                // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TEMP_REMOVE),0,1)
                let h = effect::get_handler(d, a.e).nil();
                card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TEMP_REMOVE)), 0, 1, 0, 0);
            }
        }
    }
}
