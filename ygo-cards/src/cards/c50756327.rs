//! Crowley, the First Propheseer
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 50756327;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER as u64, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // decrease tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.ntcon)
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    // e2:SetTarget(aux.FieldSummonProcTg(s.nttg))
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(effect_card_fn(nttg)), None)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local dg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let dg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // return dg:GetClassCount(Card.GetCode)>=3
        return group::get_class_count(d, &dg, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let mut g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=3 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) >= 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sg1=g:Select(tp,1,1,nil)
        let sg1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,sg1:GetFirst():GetCode())
        let codes1 = card::get_code(d, group::get_first(d, &sg1).nil());
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes1), NO_EX);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sg2=g:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,sg2:GetFirst():GetCode())
        let codes2 = card::get_code(d, group::get_first(d, &sg2).nil());
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes2), NO_EX);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sg3=g:Select(tp,1,1,nil)
        let sg3 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        let mut sg1 = sg1;
        group::add_card(&mut sg1, &sg2);
        // sg1:Merge(sg3)
        group::add_card(&mut sg1, &sg3);
        // Duel.ConfirmCards(1-tp,sg1)
        duel::confirm_cards(d, 1 - tp, sg1.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        // local cg=sg1:Select(1-tp,1,1,nil)
        let cg = group::select(d, &sg1, 1 - tp, 1, 1, false, NO_EX).nil();
        // local tc=cg:GetFirst()
        let tc = group::get_first(d, &cg).nil();
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0 && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.nttg(e,c)`
fn nttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsRace(RACE_SPELLCASTER)
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_SPELLCASTER as u64)
}
