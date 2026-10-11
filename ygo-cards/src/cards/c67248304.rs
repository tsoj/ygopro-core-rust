//! Reverse Jar
use super::prelude::*;

pub const ID: u32 = 67248304;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change as many other monsters to face-down defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_TOHAND+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_TOHAND + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.AND(Card.IsAbleToHand,aux.FaceupFilter(Card.IsSpellTrap))`
fn thg_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None) && card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local fdg=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let fdg = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // local thg=Duel.GetMatchingGroup(aux.AND(Card.IsAbleToHand,aux.FaceupFilter(Card.IsSpellTrap)),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let thg = duel::get_matching_group(d, Some(&thg_filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,fdg,#fdg,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(fdg.clone()), fdg.len() as u8, tp, POS_FACEDOWN_DEFENSE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,thg,#thg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(thg.clone()), thg.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)==0 then return end
    if duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) == 0 {
        return;
    }
    // local g2=Duel.GetMatchingGroup(aux.AND(Card.IsAbleToHand,aux.FaceupFilter(Card.IsSpellTrap)),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&thg_filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g2==0 then return end
    if g2.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SendtoHand(g2,nil,REASON_EFFECT)==0 then return end
    if duel::send_to_hand(d, g2, None, REASON_EFFECT, None) == 0 {
        return;
    }
    // g2=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_HAND)
    let mut g2 = duel::get_operated_group(d);
    group::match_(d, &mut g2, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    // local ct1=g2:FilterCount(Card.IsControler,nil,tp)
    let ct1 = group::filter_count(d, &g2, &|d, x| card::is_controler(d, x, tp), NO_EX) as i32;
    // local ct2=#g2-ct1
    let ct2 = g2.len() as i32 - ct1;
    // local sel=false
    let mut sel = false;
    // --Each player can Set Spells/Traps from hand
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local step=turn_p==0 and 1 or -1
    // for p=turn_p,1-turn_p,step do (turn_p is 0 or 1, so the order is {turn_p, 1-turn_p})
    let players: Vec<u8> = if turn_p == 0 { vec![0, 1] } else { vec![1, 0] };
    for p in players {
        // local setg=Duel.GetMatchingGroup(Card.IsSSetable,p,LOCATION_HAND,0,nil,false,p)
        let setg = duel::get_matching_group(d, Some(&|d, x| card::is_s_setable(d, x, false, Some(p))), p, LOCATION_HAND, 0, NO_EX);
        // local setmax_ct=0
        // if p==tp then setmax_ct=math.min(#setg,ct1) else setmax_ct=math.min(#setg,ct2) end
        let setmax_ct = if p == tp { (setg.len() as i32).min(ct1) } else { (setg.len() as i32).min(ct2) };
        // if #setg>0 and setmax_ct>0 and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
        if !setg.is_empty() && setmax_ct > 0 && duel::select_yes_no(d, p, aux::stringid(ID, 1)) {
            // Duel.ShuffleHand(p)
            duel::shuffle_hand(d, p);
            // if not sel then Duel.BreakEffect() end
            if !sel {
                duel::break_effect(d);
            }
            // sel=true
            sel = true;
            // local sg=aux.SelectUnselectGroup(setg,e,p,1,setmax_ct,s.rescon,1,p,HINTMSG_SET)
            let sg = aux::select_unselect_group(
                d,
                &setg,
                Some(e),
                p,
                Some(1),
                Some(setmax_ct as usize),
                Some(&rescon),
                1,
                p,
                Some(HINTMSG_SET as u64),
                None,
                None,
                false,
            )
            .unwrap_or_default();
            // Duel.SSet(p,sg,p,false)
            duel::s_set(d, p, sg, Some(p), false);
        }
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsSSetable,nil,false,tp)==#sg
    let mut res = group::filter_count(d, sg, &|d, x| card::is_s_setable(d, x, false, Some(tp)), NO_EX) == sg.len();
    // and sg:FilterCount(aux.NOT(Card.IsType),nil,TYPE_FIELD)<=Duel.GetLocationCount(tp,LOCATION_SZONE)
    if res {
        res = (group::filter_count(d, sg, &|d, x| !card::is_type(d, x, TYPE_FIELD), NO_EX) as i32)
            <= duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    }
    // and sg:FilterCount(Card.IsType,nil,TYPE_FIELD)<=1
    if res {
        res = group::filter_count(d, sg, &|d, x| card::is_type(d, x, TYPE_FIELD), NO_EX) <= 1;
    }
    (res, false)
}
