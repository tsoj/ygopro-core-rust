//! Brutal Beast Battle
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 36809777;

/// `s.types`
const TYPES: [u32; 5] = [TYPE_RITUAL, TYPE_FUSION, TYPE_SYNCHRO, TYPE_XYZ, TYPE_LINK];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make players send monsters to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsType,t)`
fn is_faceup_type(d: &mut Duel, c: CardId, t: u32) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, t)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // for p=0,1 do
        for p in 0..2u8 {
            // for _,t in ipairs(s.types) do
            for &t in TYPES.iter() {
                // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,t),p,LOCATION_MZONE,0,2,nil) then
                if duel::is_existing_matching_card(d, Some(&|d, c| is_faceup_type(d, c, t)), p, LOCATION_MZONE, 0, 2, NO_EX) {
                    return true;
                }
            }
        }
        return false;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.cancelcon(sg,e,tp,mg)`
fn cancelcon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // for _,t in ipairs(s.types) do
    for &t in TYPES.iter() {
        // if mg:IsExists(Card.IsType,1,nil,t) and mg:FilterCount(Card.IsType,sg,t)~=1 then return false end
        if group::is_exists(d, mg, &|d, c| card::is_type(d, c, t), 1, NO_EX)
            && group::filter_count(d, mg, &|d, c| card::is_type(d, c, t), sg.into()) != 1
        {
            return (false, false);
        }
    }
    // return true
    (true, false)
}
/// `s.typecount(p)`
fn typecount(d: &mut Duel, p: u8) -> u32 {
    // local ct=0
    let mut ct = 0;
    // for _,t in ipairs(s.types) do
    for &t in TYPES.iter() {
        // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,t),p,LOCATION_MZONE,0,1,nil) then ct=ct+1 end
        if duel::is_existing_matching_card(d, Some(&|d, c| is_faceup_type(d, c, t)), p, LOCATION_MZONE, 0, 1, NO_EX) {
            ct += 1;
        }
    }
    ct
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local draw={}
    // draw[0]=false
    // draw[1]=false
    let mut draw = [false, false];
    // for p=0,1 do
    for p in 0..2u8 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,p,LOCATION_MZONE,0,nil)
        let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), p, LOCATION_MZONE, 0, NO_EX);
        // for _,t in ipairs(s.types) do
        for &t in TYPES.iter() {
            // if g:FilterCount(Card.IsType,nil,t)==1 then
            if group::filter_count(d, &g, &|d, c| card::is_type(d, c, t), NO_EX) == 1 {
                // g:Remove(Card.IsType,nil,t)
                group::remove(d, &mut g, &|d, c| card::is_type(d, c, t), NO_EX);
            }
        }
        // if #g>0 then
        if !g.is_empty() {
            // local tg=aux.SelectUnselectGroup(g,e,p,1,99,s.cancelcon,1,p,HINTMSG_TOGRAVE,s.cancelcon)
            let tg = aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                p,
                Some(1),
                Some(99),
                Some(&cancelcon),
                1,
                p,
                Some(HINTMSG_TOGRAVE as u64),
                Some(&cancelcon),
                None,
                false,
            )
            .unwrap_or_default();
            // if Duel.SendtoGrave(tg,REASON_EFFECT,PLAYER_NONE,p)>0 then
            if duel::send_to_grave(d, tg, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(p)) > 0 {
                // draw[p]=true
                draw[p as usize] = true;
            }
        }
    }
    // if draw[tp] or draw[1-tp] then
    if draw[tp as usize] || draw[(1 - tp) as usize] {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if draw[tp] then Duel.Draw(tp,s.typecount(tp),REASON_EFFECT) end
        if draw[tp as usize] {
            let n = typecount(d, tp);
            duel::draw(d, tp, n, REASON_EFFECT);
        }
        // if draw[1-tp] then Duel.Draw(1-tp,s.typecount(1-tp),REASON_EFFECT) end
        if draw[(1 - tp) as usize] {
            let n = typecount(d, 1 - tp);
            duel::draw(d, 1 - tp, n, REASON_EFFECT);
        }
    }
}
