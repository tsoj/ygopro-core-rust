//! Virakam, the Divine Spirit of Artifice
//! 人工神霊ヴィラカム
use super::prelude::*;

pub const ID: u32 = 10673071;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_INVOCATION],
    listed_series: &[SET_ALEISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you have an "Aleister" monster in your field or GY: You can Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can Set 1 "Invocation", or 1 Spell that mentions it, from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When your opponent activates a card or effect in response to the activation of your Fusion Monster's effect
    // (Quick Effect): You can banish this card from the field, and if you do, negate that opponent's effect,
    // and if you do that, banish that card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ALEISTER) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ALEISTER as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_INVOCATION) or (c:IsSpell() and c:ListsCode(CARD_INVOCATION))) and c:IsSSetable()
    (card::is_code(d, c, &[CARD_INVOCATION]) || (card::is_spell(d, c) && card::lists_code(d, c, &[CARD_INVOCATION])))
        && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local chainlink=Duel.GetCurrentChain(true)-1
    let chainlink = duel::get_current_chain(d, true) - 1;
    // if not (chainlink>0 and Duel.IsChainDisablable(ev) and ep==1-tp) then return false end
    if !(chainlink > 0 && duel::is_chain_disablable(d, a.ev as u8) && a.ep == 1 - tp) {
        return false;
    }
    // local trig_player,trig_type=Duel.GetChainInfo(chainlink,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_TYPE)
    let info = duel::get_chain_info(d, chainlink as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_TYPE]);
    let trig_player = info[0].int();
    let trig_type = info[1].int();
    // return trig_player==tp and (trig_type&TYPE_FUSION)>0
    trig_player == tp as i64 && (trig_type & TYPE_FUSION as i64) > 0
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    // if chk==0 then return c:IsAbleToRemove() and (rc:IsAbleToRemove(tp)
    //     or (not relation and Duel.IsPlayerCanRemove(tp))) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && (card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // if relation then
    if relation {
        // rc:CreateEffectRelation(e)
        card::create_effect_relation(d, rc, a.e);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,Group.FromCards(c,rc),1,tp,0)
        let mut g = CardSet::from_card(c);
        group::add_card(&mut g, &CardSet::from_card(rc));
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,rc:GetPreviousControler(),rc:GetPreviousLocation())
        let prev_controler = card::get_previous_controler(d, rc);
        let prev_location = card::get_previous_location(d, rc);
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, prev_controler, prev_location as i32);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_REMOVED)
    //     and Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::remove(d, c, 0, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_REMOVED)
        && duel::negate_effect(d, a.ev as u8)
        && {
            let rc = effect::get_handler(d, a.re.nil()).nil();
            card::is_relate_to_effect(d, rc, a.re.nil())
        }
    {
        // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, a.eg.clone().nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
