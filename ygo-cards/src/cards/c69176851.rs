//! Miasma Necromancer
use super::prelude::*;

pub const ID: u32 = 69176851;


pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters, including a DARK Spellcaster monster
    // Link.AddProcedure(c,nil,2,2,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(matcheck)), None, None);
    // Add 1 Level 5 or higher DARK Spellcaster monster from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(link_summoned)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Normal Summon 1 Spellcaster monster
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2a, CATEGORY_SUMMON);
    // e2a:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2a, EFFECT_TYPE_IGNITION);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCost(s.sumcost)
    effect::set_cost(d, e2a, Some(cost_target(sumcost)));
    // e2a:SetTarget(s.sumtg)
    effect::set_target(d, e2a, Some(cost_target(sumtg)));
    // e2a:SetOperation(s.sumop)
    effect::set_operation(d, e2a, Some(operation(sumop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Workaround so that the card to be sent as cost for "e2a" isn't considered for the potential Tributes (same as "Ancient Gear Commander" [27483935])
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD);
    // e2b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2b:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e2b, EFFECT_UNRELEASABLE_SUM);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2b, LOCATION_MZONE, 0);
    // e2b:SetTarget(function(e,c) return c==e:GetLabelObject() end)
    effect::set_target(d, e2b, Some(effect_card_fn(e2b_target)));
    // e2b:SetValue(1)
    effect::set_value(d, e2b, 1);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // e2a:SetLabelObject(e2b)
    effect::set_label_object(d, e2a, Value::Effect(e2b));
}
// function s.matfilter(c,lc,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK,lc,sumtype,tp) and c:IsRace(RACE_SPELLCASTER,lc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(lc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(lc), sumtype, Some(tp))
}
// function s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(s.matfilter,1,nil,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| matfilter(d, c, lc, sumtype, tp), 1, NO_EX)
}
// function(e) return e:GetHandler():IsLinkSummoned() end
fn link_summoned(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToHand()
    card::is_level_above(d, c, 5) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
        // local code=sc:GetCode()
        let code = card::get_code(d, sc).first().copied().unwrap_or(0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re,tp) return re:GetHandler():IsCode(code) end)
        effect::set_value_fn(
            d,
            e1,
            lua_fn(move |d: &mut Duel, a: Args| {
                let re = a.e(1);
                let h = effect::get_handler(d, re).nil();
                card::is_code(d, h, &[code])
            }),
        );
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
// function s.sumcostfilter(c,tp,eff)
fn sumcostfilter(d: &mut Duel, c: CardId, tp: u8, eff: EffectId) -> bool {
    // if not (c:IsSpellTrap() and c:IsAbleToGraveAsCost()) then return false end
    if !(card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    // eff:SetLabelObject(c)
    effect::set_label_object(d, eff, Value::Card(c));
    // local res=Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,c)
    let res = duel::is_existing_matching_card(d, Some(&sumfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, c.into());
    // eff:SetLabelObject(nil)
    effect::set_label_object(d, eff, Value::Nil);
    // return res
    res
}
// function s.sumcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn sumcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local eff=e:GetLabelObject()
    let eff = effect::get_label_object(d, a.e).as_effect().nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,tp,eff) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| sumcostfilter(d, c, tp, eff)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sumcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,tp,eff)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| sumcostfilter(d, c, tp, eff)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
// function s.sumfilter(c)
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsSummonable(true,nil)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_summonable(d, c, true, None, 0, Some(0x1f))
}
// function s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
// function s.sumop(e,tp,eg,ep,ev,re,r,rp)
fn sumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&sumfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.Summon(tp,sc,true,nil)
        duel::summon(d, tp, sc, true, None, 0, None);
    }
}
// function(e,c) return c==e:GetLabelObject() end
fn e2b_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
