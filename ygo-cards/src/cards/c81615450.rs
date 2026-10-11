//! Reptilianne Ramifications
//! レプティレス・ラミフィケーション
use super::prelude::*;

pub const ID: u32 = 81615450;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_REPTILIANNE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_REPTILIANNE) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_REPTILIANNE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.mthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mthtg(d: &mut Duel, tp: u8, chk: i64) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_DECK,0,1,nil) end
    if chk == 0 {
        return duel::is_existing_matching_card(d, Some(&mfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.stfilter(c)`
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_REPTILIANNE) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_REPTILIANNE as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.stthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn stthtg(d: &mut Duel, tp: u8, chk: i64) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.stfilter,tp,LOCATION_DECK,0,1,nil) end
    if chk == 0 {
        return duel::is_existing_matching_card(d, Some(&stfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, e: EffectId, tp: u8, chk: i64) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if chk == 0 {
        return duel::is_existing_matching_card(d, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // e:SetCategory(e:GetCategory()+CATEGORY_ATKCHANGE)
    let cat = effect::get_category(d, e);
    effect::set_category(d, e, cat + CATEGORY_ATKCHANGE);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b0=s.mthtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b0 = mthtg(d, tp, 0);
    // local b1=s.stthtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = stthtg(d, tp, 0);
    // local b2=s.atktg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = atktg(d, e, tp, 0);
    // if chk==0 then return b0 and (b1 or b2) or (b1 and b2) end
    if a.chk == Some(0) {
        return (b0 && (b1 || b2)) || (b1 && b2);
    }
    // local sel=0
    let mut sel: i64 = 0;
    // for ct=1,2 do
    for _ct in 1..=2 {
        // local stable={}
        let mut stable: Vec<i64> = Vec::new();
        // local dtable={}
        let mut dtable: Vec<u64> = Vec::new();
        // if b0 and (sel&0x1==0) then table.insert(stable,0x1) table.insert(dtable,aux.Stringid(id,0)) end
        if b0 && (sel & 0x1) == 0 {
            stable.push(0x1);
            dtable.push(aux::stringid(ID, 0));
        }
        // if b1 and (sel&0x2==0) then ...
        if b1 && (sel & 0x2) == 0 {
            stable.push(0x2);
            dtable.push(aux::stringid(ID, 1));
        }
        // if b2 and (sel&0x4==0) then ...
        if b2 && (sel & 0x4) == 0 {
            stable.push(0x4);
            dtable.push(aux::stringid(ID, 2));
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EFFECT as u64);
        // local op=Duel.SelectOption(tp,table.unpack(dtable))+1
        let op = duel::select_option(d, tp, true, &dtable);
        // sel=sel+stable[op]
        sel += stable[op as usize];
    }
    // if (sel&0x1==0x1) then s.mthtg(e,tp,eg,ep,ev,re,r,rp,1) end
    if (sel & 0x1) == 0x1 {
        mthtg(d, tp, 1);
    }
    // if (sel&0x2==0x2) then s.stthtg(e,tp,eg,ep,ev,re,r,rp,1) end
    if (sel & 0x2) == 0x2 {
        stthtg(d, tp, 1);
    }
    // if (sel&0x4==0x4) then s.atktg(e,tp,eg,ep,ev,re,r,rp,1) end
    if (sel & 0x4) == 0x4 {
        atktg(d, e, tp, 1);
    }
    // e:SetLabel(sel)
    effect::set_label(d, e, &[sel]);
    true
}
/// `s.mthop(e,tp,eg,ep,ev,re,r,rp)`
fn mthop(d: &mut Duel, tp: u8) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.mfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&mfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) Duel.ConfirmCards(1-tp,g) end
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.stthop(e,tp,eg,ep,ev,re,r,rp)`
fn stthop(d: &mut Duel, tp: u8) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.stfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&stfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) Duel.ConfirmCards(1-tp,g) end
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, e: EffectId, tp: u8) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // g:GetFirst():RegisterEffect(e1)
        let first = group::get_first(d, &g).nil();
        card::register_effect(d, first, e1, false);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, e);
    // local ct=0
    let mut ct = 0;
    // if (sel&0x1==0x1) then
    if (sel & 0x1) == 0x1 {
        // s.mthop(e,tp,eg,ep,ev,re,r,rp)
        mthop(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // ct=ct+1
        ct += 1;
    }
    // if (sel&0x2==0x2) then
    if (sel & 0x2) == 0x2 {
        // s.stthop(e,tp,eg,ep,ev,re,r,rp)
        stthop(d, tp);
        // if ct==0 then Duel.BreakEffect() end
        if ct == 0 {
            duel::break_effect(d);
        }
    }
    // if (sel&0x4==0x4) then s.atkop(e,tp,eg,ep,ev,re,r,rp) end
    if (sel & 0x4) == 0x4 {
        atkop(d, e, tp);
    }
}
