//! Terror of Trishula
use super::prelude::*;

pub const ID: u32 = 6075533;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ICE_BARRIER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish based on numbers of "Ice Barrier" Synchro Monster(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate effect that targets "Ice Barrier" Synchro Monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(function(_,_,_,_,ev) Duel.NegateEffect(ev) end)
    effect::set_operation(
        d,
        e2,
        Some(operation(|d, a| {
            duel::negate_effect(d, a.ev as u8);
        })),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.smfilter(c,tp)
fn smfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and c:IsSetCard(SET_ICE_BARRIER) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
}
// function s.bancheck(tp,loc)
fn bancheck(d: &mut Duel, tp: u8, loc: u32) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,loc,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        loc,
        1,
        NO_EX,
    )
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.smfilter,tp,LOCATION_MZONE,0,nil,tp)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| smfilter(d, c, a.tp)),
            a.tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        // local dnc=math.min(g:GetClassCount(Card.GetCode),3)
        let dnc = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64).min(3);
        // if dnc==0 then return false end
        if dnc == 0 {
            return false;
        }
        // local b1=dnc>0 and s.bancheck(tp,LOCATION_ONFIELD)
        let b1 = dnc > 0 && bancheck(d, a.tp, LOCATION_ONFIELD);
        // local b2=dnc>1 and s.bancheck(tp,LOCATION_GRAVE)
        let b2 = dnc > 1 && bancheck(d, a.tp, LOCATION_GRAVE);
        // local b3=dnc>2 and s.bancheck(tp,LOCATION_HAND)
        let b3 = dnc > 2 && bancheck(d, a.tp, LOCATION_HAND);
        // return b1 or b2 or b3
        return b1 || b2 || b3;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_HAND)
    duel::set_possible_operation_info(
        d,
        0,
        CATEGORY_REMOVE,
        None,
        1,
        1 - a.tp,
        (LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_HAND) as i32,
    );
    true
}
// function s.banlocop(tp,loc,gf)
// (`gf` is Group.Select when `random` is false, Group.RandomSelect when it is true)
fn banlocop(d: &mut Duel, tp: u8, loc: u32, random: bool) -> i32 {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,loc,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        loc,
        NO_EX,
    );
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local sg=gf(g,tp,1,1,nil)
        let sg = if random {
            group::random_select(d, &g, tp, 1)
        } else {
            group::select(d, &g, tp, 1, 1, false, NO_EX).nil()
        };
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // return Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        return duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // return 0
    0
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.smfilter,tp,LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| smfilter(d, c, tp)),
        tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // local dnc=g:GetClassCount(Card.GetCode)
    let dnc = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
    // if dnc==0 then return end
    if dnc == 0 {
        return;
    }
    // local break_chk=0
    let mut break_chk = 0;
    // if dnc>0 then
    if dnc > 0 {
        // break_chk=s.banlocop(tp,LOCATION_ONFIELD,Group.Select)
        break_chk = banlocop(d, tp, LOCATION_ONFIELD, false);
    }
    // if dnc>1 then
    if dnc > 1 {
        // if break_chk>0 then Duel.BreakEffect() end
        if break_chk > 0 {
            duel::break_effect(d);
        }
        // break_chk=s.banlocop(tp,LOCATION_GRAVE,Group.Select)
        break_chk = banlocop(d, tp, LOCATION_GRAVE, false);
    }
    // if dnc>2 then
    if dnc > 2 {
        // if break_chk>0 then Duel.BreakEffect() end
        if break_chk > 0 {
            duel::break_effect(d);
        }
        // break_chk=s.banlocop(tp,LOCATION_HAND,Group.RandomSelect)
        banlocop(d, tp, LOCATION_HAND, true);
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) or rp==tp then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) || a.rp == a.tp {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0]
        .as_group()
        .cloned()
        .nil();
    // return tg:IsExists(s.smfilter,1,nil,tp) and Duel.IsChainDisablable(ev)
    group::is_exists(d, &tg, &|d, c| smfilter(d, c, a.tp), 1, NO_EX)
        && duel::is_chain_disablable(d, a.ev as u8)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
