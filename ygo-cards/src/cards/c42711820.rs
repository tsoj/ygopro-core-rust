//! Scrap Warrior
use super::prelude::*;

pub const ID: u32 = 42711820;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[16449363, CARD_JUNK_SYNCHRON, CARD_JUNK_WARRIOR, ID],
    listed_series: &[SET_WARRIOR],
    material: &[16449363],
    material_setcode: &[SET_SYNCHRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Scrap Synchron" + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(tunerfilter)),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Add to your hand, or send to the GY, 1 "Junk Synchron" or 1 card that mentions "Junk Warrior" from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, h)
    })));
    // e1:SetTarget(s.thtgtg)
    effect::set_target(d, e1, Some(cost_target(thtgtg)));
    // e1:SetOperation(s.thtgop)
    effect::set_operation(d, e1, Some(operation(thtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // The activated effects of monsters that mention "Junk Warrior", and Synchro Monsters with "Warrior" in their original names, that you control cannot be negated, except "Scrap Warrior"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e2, EFFECT_CANNOT_DISEFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.effval)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| effval(d, a.e(0), a.int(1))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.tunerfilter(c,lc,stype,tp)
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(lc,stype,tp,16449363) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, lc, stype, tp, &[16449363]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
// s.thtgfilter(c)
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_JUNK_SYNCHRON) or c:ListsCode(CARD_JUNK_WARRIOR)) and (c:IsAbleToHand() or c:IsAbleToGrave())
    (card::is_code(d, c, &[CARD_JUNK_SYNCHRON]) || card::lists_code(d, c, &[CARD_JUNK_WARRIOR]))
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
// s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thtgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thtgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thtgop(e,tp,eg,ep,ev,re,r,rp)
fn thtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.thtgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thtgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // aux.ToHandOrElse(g,tp)
        aux::to_hand_or_else(d, g, tp, None, None, None);
    }
}
// s.effval(e,ct)
fn effval(d: &mut Duel, e: EffectId, ct: i64) -> bool {
    // local trig_e,trig_p,trig_loc=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, ct as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_LOCATION]);
    let trig_e = info[0].as_effect().nil();
    let trig_p = info[1].int() as u8;
    let trig_loc = info[2].int() as u32;
    // if not (trig_p==e:GetHandlerPlayer() and (trig_loc&LOCATION_MZONE)>0) then return false end
    if !(trig_p == effect::get_handler_player(d, e) && (trig_loc & LOCATION_MZONE) > 0) {
        return false;
    }
    // local trig_c=trig_e:GetHandler()
    let trig_c = effect::get_handler(d, trig_e).nil();
    // return (trig_c:ListsCode(CARD_JUNK_WARRIOR) or (trig_c:IsSynchroMonster() and trig_c:IsOriginalSetCard(SET_WARRIOR))) and not trig_c:IsCode(id)
    (card::lists_code(d, trig_c, &[CARD_JUNK_WARRIOR]) || (card::is_synchro_monster(d, trig_c) && card::is_original_set_card(d, trig_c, &[SET_WARRIOR as u16])))
        && !card::is_code(d, trig_c, &[ID])
}
