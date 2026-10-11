//! Lollipo☆Yummy
//! ロリポー☆ヤミー
use super::prelude::*;

pub const ID: u32 = 4215180;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control a Link-1 monster or a Level 2 Synchro Monster, you can Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 card from your opponent's GY into the Deck, or, if this card was Special Summoned by a Synchro Monster's effect, you can banish that card instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_REMOVE);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLink(1) or (c:IsType(TYPE_SYNCHRO) and c:IsLevel(2))) and c:IsFaceup()
    (card::is_link(d, c, &[1]) || (card::is_type(d, c, TYPE_SYNCHRO) && card::is_level(d, c, &[2]))) && card::is_faceup(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tgfilter(c,sp_chk)`
fn tgfilter(d: &mut Duel, c: CardId, sp_chk: bool) -> bool {
    // return c:IsAbleToDeck() or (sp_chk and c:IsAbleToRemove())
    card::is_able_to_deck(d, c) || (sp_chk && card::is_able_to_remove(d, c, None, None, None))
}
/// `local sp_chk=re and e:GetHandler():IsSpecialSummoned() and re:IsMonsterEffect() and re:GetHandler():IsOriginalType(TYPE_SYNCHRO)`
fn sp_chk_of(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else {
        return false;
    };
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_special_summoned(d, h) || !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_original_type(d, rh, TYPE_SYNCHRO)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sp_chk=...
    let sp_chk = sp_chk_of(d, a);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tgfilter(chkc,e:GetLabel()==1) end
    if let Some(chkc) = a.chkc() {
        let label_is_1 = effect::get_label(d, a.e) == 1;
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tgfilter(d, chkc, label_is_1);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_GRAVE,1,nil,sp_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d: &mut Duel, c: CardId| tgfilter(d, c, sp_chk)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // e:SetLabel(sp_chk and 1 or 0)
    effect::set_label(d, a.e, &[if sp_chk { 1 } else { 0 }]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_GRAVE,1,1,nil,sp_chk)
    let g = duel::select_target(d, a.tp, Some(&|d: &mut Duel, c: CardId| tgfilter(d, c, sp_chk)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if sp_chk then
    if sp_chk {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), 1, a.tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), 1, a.tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), 1, a.tp, 0);
    }
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local sp_chk=e:GetLabel()==1
    let sp_chk = effect::get_label(d, a.e) == 1;
    // local b1=tc:IsAbleToDeck()
    let b1 = card::is_able_to_deck(d, tc);
    // local b2=sp_chk and tc:IsAbleToRemove()
    let b2 = sp_chk && card::is_able_to_remove(d, tc, None, None, None);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=nil
    // if sp_chk then
    let op = if sp_chk {
        // op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
        duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else {
        // op=1
        Some(1)
    };
    // if op==1 then
    if op == Some(1) {
        // Shuffle it into the Deck
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else if op == Some(2) {
        // Banish it
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
