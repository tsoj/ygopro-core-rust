//! Lightsworn Archfiend Minerva (Athenian Lightsworn)
use super::prelude::*;

pub const ID: u32 = 18843291;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LIGHTSWORN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // "Lightsworn" monsters you control cannot be banished by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e1, EFFECT_CANNOT_REMOVE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(s.rmlimit)
    effect::set_target(d, e1, Some(lua_fn(rmlimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send "Lightsworn" monsters with different Types from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register the number of "Lightsworn" materials used for its summon
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2a, EFFECT_MATERIAL_CHECK);
    // e2a:SetValue(s.valcheck)
    effect::set_value_fn(d, e2a, lua_fn(valcheck));
    // e2a:SetLabelObject(e2)
    effect::set_label_object(d, e2a, Value::Effect(e2));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Send cards from the top of your Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_DECKDES);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.toptgcost)
    effect::set_cost(d, e3, Some(cost_target(toptgcost)));
    // e3:SetTarget(s.toptgtg)
    effect::set_target(d, e3, Some(cost_target(toptgtg)));
    // e3:SetOperation(function(e,tp) Duel.DiscardDeck(tp,e:GetLabel(),REASON_EFFECT) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        let n = effect::get_label(d, a.e);
        duel::discard_deck(d, a.tp, n as u16, REASON_EFFECT);
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // function(e,c,tp,r)
    let e = a.e(0);
    let c = a.c(1);
    let r = a.int(3) as u32;
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()
    //     and c:IsControler(e:GetHandlerPlayer()) and not c:IsImmuneToEffect(e) and r&REASON_EFFECT>0
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, effect::get_handler_player(d, e))
        && !card::is_immune_to_effect(d, c, e)
        && r & REASON_EFFECT > 0
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabel()>0 and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return effect::get_label(d, a.e) > 0 && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=math.min(e:GetLabel(),#g)
    let ct = effect::get_label(d, a.e).min(g.len() as i64) as u16;
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,ct,aux.dpcheck(Card.GetRace),1,tp,HINTMSG_TOGRAVE)
    let dp = aux::dpcheck(race_value);
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(1),
        Some(ct as usize),
        Some(&dp),
        1,
        tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #tg>0 then Duel.SendtoGrave(tg,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::send_to_grave(d, &tg, REASON_EFFECT, None, None);
    }
}
fn race_value(d: &mut Duel, c: CardId) -> i64 {
    // Card.GetRace
    card::get_race(d, c) as i64
}
fn valcheck(d: &mut Duel, a: Args) {
    // function(e,c)
    let e = a.e(0);
    let c = a.c(1);
    // local ct=c:GetMaterial():FilterCount(Card.IsSetCard,nil,SET_LIGHTSWORN,c,SUMMON_TYPE_SYNCHRO,e:GetHandlerPlayer())
    let mat = card::get_material(d, c);
    let ct = group::filter_count(d, &mat, &|d, x| card::is_set_card(d, x, &[SET_LIGHTSWORN as u16]), NO_EX);
    // e:GetLabelObject():SetLabel(ct)
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e2, &[ct as i64]);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn toptgcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,1)
    //     and Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, tp, 1)
            && duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local ct=1
    let mut ct: u16 = 1;
    // for i=2,4 do if Duel.IsPlayerCanDiscardDeckAsCost(tp,i) then ct=ct+1 end end
    for i in 2..=4 {
        if duel::is_player_can_discard_deck_as_cost(d, tp, i) {
            ct += 1;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, ct, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    true
}
fn toptgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,e:GetLabel())
    let n = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, n as i32);
    true
}
