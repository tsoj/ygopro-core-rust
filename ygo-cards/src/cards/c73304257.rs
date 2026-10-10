//! Alpha, the Master of Beasts
use super::prelude::*;

pub const ID: u32 = 73304257;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon from the hand
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return monsters to the hand
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumatk(tp)`
fn sumatk(d: &mut Duel, tp: u8) -> i64 {
    // return Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil):GetSum(Card.GetAttack)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and s.sumatk(1-tp)>s.sumatk(tp)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && sumatk(d, 1 - tp) > sumatk(d, tp)
}
/// `s.sthfilter(c)`
fn sthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACES_BEAST_BWARRIOR_WINGB) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64) && card::is_able_to_hand(d, c, None)
}
/// `s.othfilter(c)`
fn othfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.sthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && sthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.sthfilter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.othfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&sthfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&othfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.othfilter,tp,0,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&othfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local tg=Duel.SelectTarget(tp,s.sthfilter,tp,LOCATION_MZONE,0,1,ct,nil)
    let tg = duel::select_target(d, tp, Some(&sthfilter), tp, LOCATION_MZONE, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,ct*2,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), (ct * 2) as u8, PLAYER_ALL as u8, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // "Zoo King Alpha" you control cannot attack directly this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsCode,id))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[ID]))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_else(CardSet::new);
    // if Duel.SendtoHand(tg,nil,REASON_EFFECT)>0 then
    if duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_HAND)
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
        // local g=Duel.GetMatchingGroup(s.othfilter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&othfilter), tp, 0, LOCATION_MZONE, NO_EX);
        // if ct==0 or #g<ct then return end
        if ct == 0 || g.len() < ct {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local sg=g:Select(tp,ct,ct,nil)
        let sg = group::select(d, &g, tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
