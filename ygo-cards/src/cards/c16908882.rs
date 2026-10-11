//! Number 104: Masquerade Vain
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 16908882;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BARIANS], xyz_number: Some(104), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Once per turn: You can detach 1 material from this card; add 1 "Barian's" Spell/Trap from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // A monster that was Xyz Summoned using this card on the field as material gains this effect
    // ● If it is Xyz Summoned: Activate this effect; your opponent can banish 4 monsters with different Attributes and 1 Spell from their Deck. If they do not, you banish the top 10 cards of their Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r==REASON_XYZ end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BARIANS) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BARIANS as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
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
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` condition of e2: `r==REASON_XYZ`
fn e2_cond(_d: &mut Duel, a: Args) -> bool {
    a.int(6) == REASON_XYZ as i64
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(ban_target)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(ban_op)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` condition of the effect added to the Xyz monster
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsXyzSummoned() and eg:IsContains(c)
    card::is_xyz_summoned(d, c) && group::is_contains(&a.g(2), c)
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)` of the effect added to the Xyz monster
fn ban_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg==5 and sg:GetClassCount(Card.GetAttribute)==5, sg:FilterCount(Card.IsSpell,nil)~=1
    let first = sg.len() == 5 && group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 5;
    let second = group::filter_count(d, sg, &|d, c| card::is_spell(d, c), NO_EX) != 1;
    (first, second)
}
/// `s.banfilter(c)`
fn banfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsMonster() or c:IsSpell()) and c:IsAbleToRemove()
    (card::is_monster(d, c) || card::is_spell(d, c)) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)` of the effect added to the Xyz monster
fn ban_op(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // local g=Duel.GetMatchingGroup(s.banfilter,tp,0,LOCATION_DECK,nil)
    let g = duel::get_matching_group(d, Some(&banfilter), a.tp, 0, LOCATION_DECK, NO_EX);
    // if g:IsExists(Card.IsSpell,1,nil) and g:GetClassCount(Card.GetAttribute)>4 and Duel.SelectYesNo(opp,aux.Stringid(id,2)) then
    if group::is_exists(d, &g, &|d, c| card::is_spell(d, c), 1, NO_EX)
        && group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) > 4
        && duel::select_yes_no(d, opp, aux::stringid(ID, 2))
    {
        // local sg=aux.SelectUnselectGroup(g,e,opp,5,5,s.rescon,1,opp,HINTMSG_REMOVE)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), opp, Some(5), Some(5), Some(&rescon), 1, opp, Some(HINTMSG_REMOVE as u64), None, None, false)
            .unwrap_or_default();
        // if #sg==5 then
        if sg.len() == 5 {
            // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT,nil,opp)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, Some(opp));
        }
    } else {
        // local dg=Duel.GetDecktopGroup(opp,10)
        let dg = duel::get_decktop_group(d, opp, 10);
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(dg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, dg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
